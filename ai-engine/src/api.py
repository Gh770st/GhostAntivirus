"""
GhostAntivirus AI Engine REST API
FastAPI server for threat detection and analysis
"""

import asyncio
import logging
import time
import psutil
from pathlib import Path
from typing import List, Optional, Dict, Any
import uuid
from datetime import datetime

from fastapi import FastAPI, HTTPException, UploadFile, File, BackgroundTasks, Depends, Security
from fastapi.security import HTTPBearer, HTTPAuthorizationCredentials
from fastapi.middleware.cors import CORSMiddleware
from fastapi.responses import JSONResponse
import uvicorn
import structlog
from pydantic import BaseModel

from .engine import AIEngine
from .config import Config
from .models import (
    AnalysisResult, BatchAnalysisRequest, BatchAnalysisResponse,
    ScanRequest, ScanProgress, SystemStatus, ThreatIntelligence,
    FileInfo, Alert
)

log = structlog.get_logger()

# Security
security = HTTPBearer()

# Global variables
app = FastAPI(
    title="GhostAntivirus AI Engine API",
    description="AI-powered threat detection and analysis API",
    version="3.0.0",
    docs_url="/docs",
    redoc_url="/redoc"
)

ai_engine: Optional[AIEngine] = None
config: Optional[Config] = None
active_scans: Dict[str, ScanProgress] = {}
startup_time: float = time.time()
total_files_processed: int = 0
total_threats_detected: int = 0


class AnalysisRequest(BaseModel):
    """Request for single file analysis"""
    file_path: str
    include_features: bool = False


class HealthResponse(BaseModel):
    """Health check response"""
    status: str
    timestamp: str
    engine_ready: bool
    model_loaded: bool


def get_current_user(credentials: HTTPAuthorizationCredentials = Security(security)):
    """Simple authentication - in production, use proper JWT or API key validation"""
    # For now, accept any bearer token
    # In production, validate JWT tokens or API keys against a database
    if credentials.credentials:
        return {"user": "authenticated", "token": credentials.credentials}
    raise HTTPException(status_code=401, detail="Invalid authentication credentials")


@app.on_event("startup")
async def startup_event():
    """Initialize the AI engine on startup"""
    global ai_engine, config
    
    try:
        # Load configuration
        config = Config.default()
        config = config.apply_environment_overrides()
        
        # Initialize AI engine
        ai_engine = AIEngine(config)
        
        log.info("GhostAntivirus AI Engine API started successfully")
        
    except Exception as e:
        log.error(f"Failed to start AI Engine: {e}")
        raise


@app.on_event("shutdown")
async def shutdown_event():
    """Cleanup on shutdown"""
    log.info("GhostAntivirus AI Engine API shutting down")


@app.get("/", response_model=Dict[str, Any])
async def root():
    """Root endpoint"""
    return {
        "name": "GhostAntivirus AI Engine API",
        "version": "3.0.0",
        "status": "running",
        "endpoints": {
            "health": "/health",
            "analyze": "/analyze",
            "analyze_batch": "/analyze/batch",
            "scan": "/scan",
            "status": "/status",
            "docs": "/docs"
        }
    }


@app.get("/health", response_model=HealthResponse)
async def health_check():
    """Health check endpoint"""
    
    if ai_engine is None:
        raise HTTPException(status_code=503, detail="AI Engine not initialized")
    
    try:
        is_healthy = await ai_engine.health_check()
        status = "healthy" if is_healthy else "unhealthy"
        
        return HealthResponse(
            status=status,
            timestamp=time.strftime("%Y-%m-%d %H:%M:%S"),
            engine_ready=is_healthy,
            model_loaded=ai_engine.is_model_loaded()
        )
        
    except Exception as e:
        log.error(f"Health check failed: {e}")
        raise HTTPException(status_code=503, detail=f"Health check failed: {str(e)}")


@app.post("/analyze", response_model=AnalysisResult)
async def analyze_file(
    request: AnalysisRequest,
    user: dict = Depends(get_current_user)
):
    """Analyze a single file for threats"""
    
    global total_files_processed, total_threats_detected
    
    if ai_engine is None:
        raise HTTPException(status_code=503, detail="AI Engine not initialized")
    
    try:
        file_path = Path(request.file_path)
        result = await ai_engine.analyze_file(file_path)
        
        # Update statistics
        total_files_processed += 1
        if result.prediction == "malicious" or result.threat_score > 0.5:
            total_threats_detected += 1
        
        # Optionally include features based on request
        if not request.include_features:
            result.features = None
        
        return result
        
    except FileNotFoundError:
        raise HTTPException(status_code=404, detail=f"File not found: {request.file_path}")
    except Exception as e:
        log.error(f"Analysis failed for {request.file_path}: {e}")
        raise HTTPException(status_code=500, detail=f"Analysis failed: {str(e)}")


@app.post("/analyze/upload", response_model=AnalysisResult)
async def analyze_uploaded_file(
    file: UploadFile = File(...),
    include_features: bool = False,
    user: dict = Depends(get_current_user)
):
    """Analyze an uploaded file for threats"""
    
    if ai_engine is None:
        raise HTTPException(status_code=503, detail="AI Engine not initialized")
    
    try:
        # Save uploaded file temporarily
        temp_dir = Path(config.api.temp_dir if config else "/tmp")
        temp_dir.mkdir(exist_ok=True)
        
        temp_file_path = temp_dir / f"upload_{uuid.uuid4().hex}_{file.filename}"
        
        # Save file
        with open(temp_file_path, "wb") as temp_file:
            content = await file.read()
            temp_file.write(content)
        
        try:
            # Analyze the file
            result = await ai_engine.analyze_file(temp_file_path)
            
            # Remove temporary file
            temp_file_path.unlink(missing_ok=True)
            
            # Optionally include features
            if not include_features:
                result.features = None
            
            return result
            
        except Exception as e:
            # Clean up temp file on error
            temp_file_path.unlink(missing_ok=True)
            raise e
            
    except Exception as e:
        log.error(f"Upload analysis failed: {e}")
        raise HTTPException(status_code=500, detail=f"Upload analysis failed: {str(e)}")


@app.post("/analyze/batch", response_model=BatchAnalysisResponse)
async def analyze_batch(
    request: BatchAnalysisRequest,
    background_tasks: BackgroundTasks,
    user: dict = Depends(get_current_user)
):
    """Analyze multiple files in batch"""
    
    if ai_engine is None:
        raise HTTPException(status_code=503, detail="AI Engine not initialized")
    
    try:
        request_id = str(uuid.uuid4())
        
        # Create scan progress
        progress = ScanProgress(
            scan_id=request_id,
            status="queued",
            progress=0.0,
            files_scanned=0,
            files_total=len(request.file_paths),
            start_time=time.time()
        )
        
        active_scans[request_id] = progress
        
        # Start background task
        background_tasks.add_task(
            process_batch_analysis,
            request_id,
            request.file_paths,
            request.callback_url
        )
        
        return BatchAnalysisResponse(
            request_id=request_id,
            status="queued",
            total_files=len(request.file_paths)
        )
        
    except Exception as e:
        log.error(f"Batch analysis setup failed: {e}")
        raise HTTPException(status_code=500, detail=f"Batch analysis setup failed: {str(e)}")


async def process_batch_analysis(
    request_id: str,
    file_paths: List[str],
    callback_url: Optional[str] = None
):
    """Process batch analysis in background"""
    
    global total_files_processed, total_threats_detected
    
    if request_id not in active_scans:
        return
    
    progress = active_scans[request_id]
    progress.status = "running"
    
    try:
        results = []
        
        for i, file_path_str in enumerate(file_paths):
            try:
                file_path = Path(file_path_str)
                result = await ai_engine.analyze_file(file_path)
                results.append(result)
                
                # Update statistics
                total_files_processed += 1
                if result.prediction == "malicious" or result.threat_score > 0.5:
                    total_threats_detected += 1
                
            except Exception as e:
                log.error(f"Failed to analyze {file_path_str}: {e}")
                # Create error result
                error_result = AnalysisResult(
                    file_path=file_path_str,
                    threat_score=0.0,
                    prediction="error",
                    confidence=0.0,
                    error=str(e)
                )
                results.append(error_result)
            
            # Update progress
            progress.files_scanned = i + 1
            progress.progress = (progress.files_scanned / progress.files_total) * 100
            
            # Small delay to prevent overwhelming the system
            await asyncio.sleep(0.1)
        
        # Update final status
        progress.status = "completed"
        progress.progress = 100.0
        active_scans[request_id].results = results
        
        # Send callback if provided
        if callback_url:
            await send_callback(callback_url, request_id, results)
        
    except Exception as e:
        log.error(f"Batch analysis {request_id} failed: {e}")
        progress.status = "failed"
        progress.errors.append(str(e))


@app.get("/analyze/batch/{request_id}", response_model=ScanProgress)
async def get_batch_progress(
    request_id: str,
    user: dict = Depends(get_current_user)
):
    """Get batch analysis progress"""
    
    if request_id not in active_scans:
        raise HTTPException(status_code=404, detail="Batch request not found")
    
    return active_scans[request_id]


@app.post("/scan", response_model=ScanProgress)
async def start_scan(
    request: ScanRequest,
    background_tasks: BackgroundTasks,
    user: dict = Depends(get_current_user)
):
    """Start a scan operation"""
    
    if ai_engine is None:
        raise HTTPException(status_code=503, detail="AI Engine not initialized")
    
    try:
        scan_id = str(uuid.uuid4())
        
        # Create scan progress
        progress = ScanProgress(
            scan_id=scan_id,
            status="queued",
            progress=0.0,
            files_scanned=0,
            files_total=0,  # Will be determined when scan starts
            start_time=time.time()
        )
        
        active_scans[scan_id] = progress
        
        # Start background scan
        background_tasks.add_task(
            process_scan,
            scan_id,
            request.target,
            request.scan_type,
            request.recursive
        )
        
        return progress
        
    except Exception as e:
        log.error(f"Scan setup failed: {e}")
        raise HTTPException(status_code=500, detail=f"Scan setup failed: {str(e)}")


async def process_scan(
    scan_id: str,
    target: str,
    scan_type: str,
    recursive: bool
):
    """Process scan operation"""
    
    # TODO: Implement scan logic
    # This would integrate with the Core Engine's scanning capabilities
    
    if scan_id not in active_scans:
        return
    
    progress = active_scans[scan_id]
    progress.status = "running"
    
    try:
        # Simulate scan progress
        target_path = Path(target)
        
        # Count files (simple implementation)
        if target_path.is_file():
            total_files = 1
        else:
            total_files = sum(1 for _ in target_path.rglob("*") if _.is_file())
        
        progress.files_total = total_files
        
        # Simulate scanning
        for i in range(total_files):
            # Simulate processing time
            await asyncio.sleep(0.01)
            
            progress.files_scanned = i + 1
            progress.progress = (progress.files_scanned / progress.files_total) * 100
        
        progress.status = "completed"
        
    except Exception as e:
        log.error(f"Scan {scan_id} failed: {e}")
        progress.status = "failed"
        progress.errors.append(str(e))


@app.get("/scan/{scan_id}", response_model=ScanProgress)
async def get_scan_progress(
    scan_id: str,
    user: dict = Depends(get_current_user)
):
    """Get scan progress"""
    
    if scan_id not in active_scans:
        raise HTTPException(status_code=404, detail="Scan not found")
    
    return active_scans[scan_id]


@app.get("/status", response_model=SystemStatus)
async def get_system_status(user: dict = Depends(get_current_user)):
    """Get system status"""
    
    if ai_engine is None:
        raise HTTPException(status_code=503, detail="AI Engine not initialized")
    
    try:
        engine_status = ai_engine.get_engine_status()
        
        # Get real system metrics
        cpu_percent = psutil.cpu_percent(interval=0.1)
        memory = psutil.virtual_memory()
        disk = psutil.disk_usage('/')
        
        # Calculate uptime
        uptime_seconds = time.time() - startup_time
        
        return SystemStatus(
            status="running" if engine_status["model_loaded"] else "degraded",
            version="3.0.0",
            uptime=uptime_seconds,
            cpu_usage=cpu_percent,
            memory_usage=memory.percent,
            disk_usage=disk.percent,
            active_scans=len([s for s in active_scans.values() if s.status == "running"]),
            files_processed=total_files_processed,
            threats_detected=total_threats_detected,
            model_loaded=engine_status["model_loaded"],
            database_status="connected"
        )
        
    except Exception as e:
        log.error(f"Status check failed: {e}")
        raise HTTPException(status_code=500, detail=f"Status check failed: {str(e)}")


@app.get("/threat-intelligence/{file_hash}")
async def get_threat_intelligence(
    file_hash: str,
    user: dict = Depends(get_current_user)
):
    """Get threat intelligence for a file hash"""
    
    if ai_engine is None:
        raise HTTPException(status_code=503, detail="AI Engine not initialized")
    
    try:
        intel = await ai_engine.get_threat_intelligence(file_hash)
        return intel
        
    except Exception as e:
        log.error(f"Threat intelligence lookup failed: {e}")
        raise HTTPException(status_code=500, detail=f"Threat intelligence lookup failed: {str(e)}")


async def send_callback(callback_url: str, request_id: str, results: List[AnalysisResult]):
    """Send callback notification"""
    
    try:
        import httpx
        
        payload = {
            "request_id": request_id,
            "status": "completed",
            "results": [result.dict() for result in results]
        }
        
        async with httpx.AsyncClient() as client:
            response = await client.post(callback_url, json=payload, timeout=30.0)
            response.raise_for_status()
            
        log.info(f"Callback sent successfully to {callback_url}")
        
    except Exception as e:
        log.error(f"Failed to send callback to {callback_url}: {e}")


# Add CORS middleware
app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],  # Configure appropriately for production
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)


# Exception handlers
@app.exception_handler(404)
async def not_found_handler(request, exc):
    return JSONResponse(
        status_code=404,
        content={"error": "Endpoint not found", "detail": str(exc.detail)}
    )


@app.exception_handler(500)
async def internal_error_handler(request, exc):
    log.error(f"Internal server error: {exc}")
    return JSONResponse(
        status_code=500,
        content={"error": "Internal server error", "detail": "An unexpected error occurred"}
    )


def create_app(config_path: Optional[str] = None) -> FastAPI:
    """Create and configure FastAPI app"""
    
    # Load configuration if provided
    if config_path:
        global config
        config = Config.load(Path(config_path))
    
    return app


if __name__ == "__main__":
    uvicorn.run(
        "ai_engine.api:app",
        host="0.0.0.0",
        port=8000,
        reload=True,
        log_level="info"
    )