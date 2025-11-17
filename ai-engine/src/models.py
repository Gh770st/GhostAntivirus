"""
Data models and types for GhostAntivirus AI Engine
"""

from enum import Enum
from typing import Dict, List, Optional, Any
from pydantic import BaseModel, Field
from datetime import datetime


class ThreatType(str, Enum):
    """Threat classification types"""
    VIRUS = "virus"
    TROJAN = "trojan"
    WORM = "worm"
    RANSOMWARE = "ransomware"
    SPYWARE = "spyware"
    ADWARE = "adware"
    ROOTKIT = "rootkit"
    BACKDOOR = "backdoor"
    MALWARE = "malware"
    SUSPICIOUS = "suspicious"
    UNKNOWN = "unknown"


class Severity(str, Enum):
    """Threat severity levels"""
    LOW = "low"
    MEDIUM = "medium"
    HIGH = "high"
    CRITICAL = "critical"


class PredictionResult(BaseModel):
    """Prediction result from ML model"""
    prediction: str = Field(..., description="Predicted class (benign/malicious)")
    confidence: float = Field(..., ge=0.0, le=1.0, description="Confidence score")
    probability: float = Field(..., ge=0.0, le=1.0, description="Probability of malicious")
    threat_score: float = Field(..., ge=0.0, le=100.0, description="Threat score (0-100)")


class AnalysisResult(BaseModel):
    """Complete analysis result for a file"""
    file_path: str = Field(..., description="Path to analyzed file")
    file_hash: Optional[str] = Field(None, description="SHA256 hash of the file")
    file_size: Optional[int] = Field(None, description="File size in bytes")
    threat_score: float = Field(..., ge=0.0, le=100.0, description="Threat score (0-100)")
    prediction: str = Field(..., description="Predicted classification")
    confidence: float = Field(..., ge=0.0, le=1.0, description="Confidence in prediction")
    threat_type: ThreatType = Field(..., description="Type of threat detected")
    severity: Severity = Field(..., description="Severity level of threat")
    features: Optional[Dict[str, Any]] = Field(None, description="Extracted features")
    analysis_time: float = Field(..., description="Time taken for analysis in seconds")
    model_version: str = Field(..., description="Version of model used")
    error: Optional[str] = Field(None, description="Error message if analysis failed")
    timestamp: datetime = Field(default_factory=datetime.utcnow, description="Analysis timestamp")
    
    class Config:
        use_enum_values = True


class BatchAnalysisRequest(BaseModel):
    """Request for batch file analysis"""
    file_paths: List[str] = Field(..., description="List of file paths to analyze")
    priority: str = Field(default="normal", description="Processing priority")
    callback_url: Optional[str] = Field(None, description="URL to call when complete")


class BatchAnalysisResponse(BaseModel):
    """Response for batch analysis"""
    request_id: str = Field(..., description="Unique request identifier")
    status: str = Field(..., description="Processing status")
    total_files: int = Field(..., description="Total files to process")
    completed_files: int = Field(default=0, description="Files processed so far")
    results: Optional[List[AnalysisResult]] = Field(None, description="Analysis results")
    error: Optional[str] = Field(None, description="Error message if failed")


class FileInfo(BaseModel):
    """Basic file information"""
    path: str = Field(..., description="File path")
    name: str = Field(..., description="File name")
    extension: str = Field(..., description="File extension")
    size: int = Field(..., description="File size in bytes")
    created_time: Optional[datetime] = Field(None, description="File creation time")
    modified_time: Optional[datetime] = Field(None, description="File modification time")
    accessed_time: Optional[datetime] = Field(None, description="File access time")
    is_executable: bool = Field(default=False, description="Whether file is executable")
    is_hidden: bool = Field(default=False, description="Whether file is hidden")
    permissions: Optional[str] = Field(None, description="File permissions")


class FeatureInfo(BaseModel):
    """Information about extracted features"""
    name: str = Field(..., description="Feature name")
    value: Any = Field(..., description="Feature value")
    description: Optional[str] = Field(None, description="Feature description")
    importance: Optional[float] = Field(None, ge=0.0, le=1.0, description="Feature importance")


class ThreatIntelligence(BaseModel):
    """Threat intelligence information"""
    hash: str = Field(..., description="File hash")
    known_malicious: bool = Field(default=False, description="Whether file is known malicious")
    reputation_score: float = Field(default=0.0, ge=0.0, le=100.0, description="Reputation score")
    first_seen: Optional[datetime] = Field(None, description="First time file was seen")
    last_seen: Optional[datetime] = Field(None, description="Last time file was seen")
    file_type: str = Field(default="unknown", description="File type")
    sources: List[str] = Field(default_factory=list, description="Threat intel sources")
    tags: List[str] = Field(default_factory=list, description="Threat tags")
    family: Optional[str] = Field(None, description="Malware family")
    campaign: Optional[str] = Field(None, description="Campaign name")


class QuarantineAction(BaseModel):
    """Quarantine action details"""
    action_id: str = Field(..., description="Unique action identifier")
    file_path: str = Field(..., description="Path to file")
    action_type: str = Field(..., description="Type of action (quarantine/delete/restore)")
    reason: str = Field(..., description="Reason for action")
    timestamp: datetime = Field(default_factory=datetime.utcnow, description="Action timestamp")
    performed_by: str = Field(..., description="Who performed the action")
    success: bool = Field(default=False, description="Whether action was successful")
    error: Optional[str] = Field(None, description="Error message if failed")


class SystemStatus(BaseModel):
    """System status information"""
    status: str = Field(..., description="Overall system status")
    version: str = Field(..., description="Engine version")
    uptime: float = Field(..., description="System uptime in seconds")
    cpu_usage: float = Field(..., ge=0.0, le=100.0, description="CPU usage percentage")
    memory_usage: float = Field(..., ge=0.0, le=100.0, description="Memory usage percentage")
    disk_usage: float = Field(..., ge=0.0, le=100.0, description="Disk usage percentage")
    active_scans: int = Field(default=0, description="Number of active scans")
    files_processed: int = Field(default=0, description="Total files processed")
    threats_detected: int = Field(default=0, description="Total threats detected")
    last_update: Optional[datetime] = Field(None, description="Last system update time")
    model_loaded: bool = Field(default=False, description="Whether model is loaded")
    database_status: str = Field(default="unknown", description="Database connection status")


class APIKey(BaseModel):
    """API key information"""
    key_id: str = Field(..., description="API key identifier")
    key_hash: str = Field(..., description="Hashed API key")
    name: str = Field(..., description="API key name/description")
    permissions: List[str] = Field(default_factory=list, description="API permissions")
    rate_limit: int = Field(default=1000, description="Rate limit per hour")
    created_at: datetime = Field(default_factory=datetime.utcnow, description="Creation time")
    last_used: Optional[datetime] = Field(None, description="Last usage time")
    is_active: bool = Field(default=True, description="Whether key is active")
    expires_at: Optional[datetime] = Field(None, description="Expiration time")


class ScanRequest(BaseModel):
    """Scan request model"""
    target: str = Field(..., description="Target to scan (file/directory)")
    scan_type: str = Field(default="quick", description="Scan type (quick/full/custom)")
    recursive: bool = Field(default=True, description="Whether to scan recursively")
    max_depth: Optional[int] = Field(None, description="Maximum scan depth")
    file_types: Optional[List[str]] = Field(None, description="File types to scan")
    exclude_patterns: Optional[List[str]] = Field(None, description="Patterns to exclude")
    priority: str = Field(default="normal", description="Scan priority")


class ScanProgress(BaseModel):
    """Scan progress information"""
    scan_id: str = Field(..., description="Unique scan identifier")
    status: str = Field(..., description="Scan status")
    progress: float = Field(..., ge=0.0, le=100.0, description="Progress percentage")
    files_scanned: int = Field(..., description="Number of files scanned")
    files_total: int = Field(..., description="Total files to scan")
    threats_found: int = Field(default=0, description="Number of threats found")
    threats_detected: int = Field(default=0, description="Number of threats detected")
    errors: List[str] = Field(default_factory=list, description="Scan errors")
    start_time: datetime = Field(..., description="Scan start time")
    estimated_completion: Optional[datetime] = Field(None, description="Estimated completion time")
    current_file: Optional[str] = Field(None, description="Currently scanning file")
    results: Optional[List[Dict[str, Any]]] = Field(None, description="Scan results details")


class Alert(BaseModel):
    """Security alert model"""
    alert_id: str = Field(..., description="Unique alert identifier")
    severity: Severity = Field(..., description="Alert severity")
    title: str = Field(..., description="Alert title")
    description: str = Field(..., description="Alert description")
    file_path: Optional[str] = Field(None, description="Related file path")
    threat_type: Optional[ThreatType] = Field(None, description="Related threat type")
    timestamp: datetime = Field(default_factory=datetime.utcnow, description="Alert timestamp")
    acknowledged: bool = Field(default=False, description="Whether alert was acknowledged")
    acknowledged_by: Optional[str] = Field(None, description="Who acknowledged the alert")
    resolved: bool = Field(default=False, description="Whether alert was resolved")
    resolved_by: Optional[str] = Field(None, description="Who resolved the alert")
    metadata: Dict[str, Any] = Field(default_factory=dict, description="Additional metadata")


class ModelMetrics(BaseModel):
    """ML model performance metrics"""
    model_version: str = Field(..., description="Model version")
    accuracy: float = Field(..., ge=0.0, le=1.0, description="Model accuracy")
    precision: float = Field(..., ge=0.0, le=1.0, description="Model precision")
    recall: float = Field(..., ge=0.0, le=1.0, description="Model recall")
    f1_score: float = Field(..., ge=0.0, le=1.0, description="F1 score")
    false_positive_rate: float = Field(..., ge=0.0, le=1.0, description="False positive rate")
    false_negative_rate: float = Field(..., ge=0.0, le=1.0, description="False negative rate")
    roc_auc: float = Field(..., ge=0.0, le=1.0, description="ROC AUC score")
    training_samples: int = Field(..., description="Number of training samples")
    test_samples: int = Field(..., description="Number of test samples")
    last_trained: datetime = Field(..., description="Last training date")
    inference_time_avg: float = Field(..., description="Average inference time in seconds")
    memory_usage_mb: float = Field(..., description="Model memory usage in MB")