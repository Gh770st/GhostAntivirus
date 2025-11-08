"""
GhostAntivirus AI Engine Core
Machine learning powered threat detection and analysis
"""

import asyncio
import hashlib
import logging
import pickle
from pathlib import Path
from typing import Dict, List, Optional, Tuple, Any
import numpy as np
import pandas as pd
from pydantic import BaseModel
import structlog
from sklearn.preprocessing import StandardScaler
import tensorflow as tf
from joblib import load, dump

from .config import Config
from .features import FeatureExtractor
from .models import AnalysisResult, ThreatType, Severity

log = structlog.get_logger()


class AIEngine:
    """Main AI Engine for threat detection and analysis"""
    
    def __init__(self, config: Config):
        self.config = config
        self.model = None
        self.scaler = None
        self.feature_extractor = FeatureExtractor(config)
        self._setup_logging()
        
        # Try to load existing model
        self._load_model()
    
    def _setup_logging(self) -> None:
        """Setup logging configuration"""
        logging.basicConfig(
            level=getattr(logging, self.config.logging.level.upper()),
            format=self.config.logging.format
        )
    
    def _load_model(self) -> None:
        """Load the trained model if available"""
        model_path = Path(self.config.model.path)
        
        if not model_path.exists():
            log.warning(f"Model not found at {model_path}")
            return
        
        try:
            # Load model based on type
            if self.config.model.type == "neural_network":
                self.model = tf.keras.models.load_model(model_path)
                log.info(f"Loaded neural network model from {model_path}")
            elif self.config.model.type == "sklearn":
                self.model = load(model_path)
                log.info(f"Loaded scikit-learn model from {model_path}")
            else:
                log.error(f"Unknown model type: {self.config.model.type}")
                return
            
            # Load scaler if exists
            scaler_path = model_path.parent / "scaler.pkl"
            if scaler_path.exists():
                self.scaler = load(scaler_path)
                log.info("Loaded feature scaler")
            
            log.info("AI Engine model loaded successfully")
            
        except Exception as e:
            log.error(f"Failed to load model: {e}")
            self.model = None
    
    def load_model(self, model_path: Path) -> None:
        """Load model from specific path"""
        self.config.model.path = str(model_path)
        self._load_model()
    
    def is_model_loaded(self) -> bool:
        """Check if model is loaded"""
        return self.model is not None
    
    async def analyze_file(self, file_path: Path) -> AnalysisResult:
        """Analyze a single file for threats"""
        
        if not file_path.exists():
            raise FileNotFoundError(f"File not found: {file_path}")
        
        log.info(f"Analyzing file: {file_path}")
        
        try:
            # Extract features
            features = await self.feature_extractor.extract_features(file_path)
            
            # Perform analysis
            if self.model is not None:
                prediction_result = await self._predict_threat(features)
            else:
                # Fallback to rule-based analysis
                prediction_result = self._rule_based_analysis(file_path, features)
            
            # Create analysis result
            result = AnalysisResult(
                file_path=str(file_path),
                threat_score=prediction_result["threat_score"],
                prediction=prediction_result["prediction"],
                confidence=prediction_result["confidence"],
                threat_type=prediction_result.get("threat_type", ThreatType.UNKNOWN),
                severity=prediction_result.get("severity", Severity.LOW),
                features=features,
                analysis_time=prediction_result.get("analysis_time", 0.0),
                model_version="3.0.0" if self.model else "rule-based"
            )
            
            log.info(f"Analysis complete - Score: {result.threat_score:.2f}, "
                    f"Prediction: {result.prediction}, Confidence: {result.confidence:.2f}")
            
            return result
            
        except Exception as e:
            log.error(f"Error analyzing file {file_path}: {e}")
            raise
    
    async def analyze_batch(self, file_paths: List[Path]) -> List[AnalysisResult]:
        """Analyze multiple files in batch"""
        
        log.info(f"Analyzing batch of {len(file_paths)} files")
        
        results = []
        for file_path in file_paths:
            try:
                result = await self.analyze_file(file_path)
                results.append(result)
            except Exception as e:
                log.error(f"Failed to analyze {file_path}: {e}")
                # Add error result
                error_result = AnalysisResult(
                    file_path=str(file_path),
                    threat_score=0.0,
                    prediction="error",
                    confidence=0.0,
                    error=str(e)
                )
                results.append(error_result)
        
        log.info(f"Batch analysis complete - {len(results)} files processed")
        return results
    
    async def _predict_threat(self, features: Dict[str, Any]) -> Dict[str, Any]:
        """Use ML model to predict threat"""
        
        import time
        start_time = time.time()
        
        try:
            # Convert features to array
            feature_array = self._features_to_array(features)
            
            # Apply scaling if available
            if self.scaler is not None:
                feature_array = self.scaler.transform([feature_array])
            else:
                feature_array = [feature_array]
            
            # Make prediction
            if self.config.model.type == "neural_network":
                prediction_proba = self.model.predict(feature_array, verbose=0)[0][0]
            else:  # sklearn
                prediction_proba = self.model.predict_proba(feature_array)[0][1]
            
            # Determine prediction and confidence
            threshold = self.config.model.confidence_threshold
            prediction = "malicious" if prediction_proba >= threshold else "benign"
            confidence = max(prediction_proba, 1 - prediction_proba)
            
            # Calculate threat score (0-100)
            threat_score = prediction_proba * 100
            
            # Determine threat type and severity based on score
            threat_type, severity = self._classify_threat(threat_score, features)
            
            analysis_time = time.time() - start_time
            
            return {
                "threat_score": threat_score,
                "prediction": prediction,
                "confidence": confidence,
                "threat_type": threat_type,
                "severity": severity,
                "analysis_time": analysis_time
            }
            
        except Exception as e:
            log.error(f"ML prediction failed: {e}")
            # Fallback to rule-based
            return self._rule_based_analysis(Path("unknown"), features)
    
    def _features_to_array(self, features: Dict[str, Any]) -> np.ndarray:
        """Convert features dictionary to numpy array"""
        
        # Define feature order (must match training)
        feature_order = [
            'file_size', 'file_extension_type', 'is_executable', 'has_digital_signature',
            'entropy', 'imports_count', 'exports_count', 'sections_count', 'suspicious_strings',
            'pe_entropy', 'is_packed', 'compile_time', 'has_debug_info', 'has_tls',
            'resource_size', 'version_info', 'imported_dlls_count', 'suspicious_imports',
            'file_age_days', 'path_depth', 'filename_length', 'contains_spaces',
            'extension_risk', 'is_system_file', 'has_version_info', 'company_info_present'
        ]
        
        # Extract features in order
        feature_values = []
        for feature_name in feature_order:
            value = features.get(feature_name, 0)
            if isinstance(value, bool):
                value = int(value)
            feature_values.append(value)
        
        # Pad or truncate to match expected input size
        expected_size = self.config.model.input_features
        if len(feature_values) < expected_size:
            feature_values.extend([0] * (expected_size - len(feature_values)))
        elif len(feature_values) > expected_size:
            feature_values = feature_values[:expected_size]
        
        return np.array(feature_values, dtype=np.float32)
    
    def _rule_based_analysis(self, file_path: Path, features: Dict[str, Any]) -> Dict[str, Any]:
        """Fallback rule-based threat analysis"""
        
        threat_score = 0.0
        
        # File extension risk
        extension = file_path.suffix.lower()
        risky_extensions = {'.exe', '.scr', '.bat', '.cmd', '.com', '.pif', '.vbs', '.js'}
        if extension in risky_extensions:
            threat_score += 20
        
        # File size anomaly
        file_size = features.get('file_size', 0)
        if file_size == 0:
            threat_score += 10
        elif file_size < 1024:  # Very small files
            threat_score += 15
        
        # Entropy analysis
        entropy = features.get('entropy', 0)
        if entropy > 7.5:  # High entropy suggests packed/encrypted
            threat_score += 25
        
        # Suspicious strings
        suspicious_strings = features.get('suspicious_strings', 0)
        threat_score += min(suspicious_strings * 5, 30)
        
        # No digital signature for executables
        if features.get('is_executable', False) and not features.get('has_digital_signature', False):
            threat_score += 15
        
        # Determine prediction
        prediction = "malicious" if threat_score >= 50 else "benign"
        confidence = min(threat_score / 100, 0.9)
        
        threat_type, severity = self._classify_threat(threat_score, features)
        
        return {
            "threat_score": threat_score,
            "prediction": prediction,
            "confidence": confidence,
            "threat_type": threat_type,
            "severity": severity,
            "analysis_time": 0.0
        }
    
    def _classify_threat(self, threat_score: float, features: Dict[str, Any]) -> Tuple[ThreatType, Severity]:
        """Classify threat type and severity based on score and features"""
        
        # Determine severity
        if threat_score >= 80:
            severity = Severity.CRITICAL
        elif threat_score >= 60:
            severity = Severity.HIGH
        elif threat_score >= 40:
            severity = Severity.MEDIUM
        else:
            severity = Severity.LOW
        
        # Determine threat type based on characteristics
        file_extension = features.get('file_extension_type', '')
        entropy = features.get('entropy', 0)
        imports_count = features.get('imports_count', 0)
        
        if entropy > 7.5:
            threat_type = ThreatType.TROJAN  # Often packed/obfuscated
        elif imports_count > 100:
            threat_type = ThreatType.VIRUS  # Complex behavior
        elif 'script' in file_extension:
            threat_type = ThreatType.TROJAN
        elif threat_score > 70:
            threat_type = ThreatType.MALWARE
        else:
            threat_type = ThreatType.SUSPICIOUS
        
        return threat_type, severity
    
    async def get_threat_intelligence(self, file_hash: str) -> Dict[str, Any]:
        """Get threat intelligence for a file hash"""
        
        log.info(f"Looking up threat intelligence for hash: {file_hash}")
        
        # Check local threat database first
        local_threat = self._check_local_threat_db(file_hash)
        if local_threat:
            return local_threat
        
        # Check known malicious hashes (simple implementation)
        known_malicious_hashes = self._get_known_malicious_hashes()
        is_malicious = file_hash.lower() in known_malicious_hashes
        
        reputation_score = 0.0
        if is_malicious:
            reputation_score = 0.9  # High threat score for known malicious
        
        result = {
            "hash": file_hash,
            "known_malicious": is_malicious,
            "reputation_score": reputation_score,
            "first_seen": "2024-01-01T00:00:00Z" if is_malicious else None,
            "last_seen": "2024-11-08T00:00:00Z" if is_malicious else None,
            "file_type": "executable" if is_malicious else "unknown",
            "sources": ["local_db"] if is_malicious else [],
            "threat_name": "Generic.Malware" if is_malicious else None,
            "threat_family": "Generic" if is_malicious else None
        }
        
        log.info(f"Threat intelligence lookup complete: malicious={is_malicious}")
        return result
    
    def _check_local_threat_db(self, file_hash: str) -> Optional[Dict[str, Any]]:
        """Check local threat database for known threats"""
        # In production, this would query a real database
        # For now, return None to use the simple hash check
        return None
    
    def _get_known_malicious_hashes(self) -> set:
        """Get set of known malicious file hashes"""
        # EICAR test file hash and other known test malware hashes
        return {
            "275a021bbfb6489e54d471899f7db9d1663fc695ec2fe2a2c4538aabf651fd0f",  # EICAR
            "44d88612fea8a8f36de82e1278abb02f",  # Another test hash
            "3395856ce81f2b7382dee72602f798b642f14140",  # Test hash
        }
    
    def get_engine_status(self) -> Dict[str, Any]:
        """Get AI engine status information"""
        
        return {
            "model_loaded": self.is_model_loaded(),
            "model_type": self.config.model.type,
            "model_path": self.config.model.path,
            "feature_extractor_ready": self.feature_extractor.is_ready(),
            "supported_features": self.feature_extractor.get_supported_features(),
            "confidence_threshold": self.config.model.confidence_threshold,
            "batch_size": self.config.model.batch_size,
            "version": "3.0.0"
        }
    
    async def health_check(self) -> bool:
        """Perform health check"""
        try:
            # Check if model is loaded
            if not self.is_model_loaded():
                log.warning("Model not loaded during health check")
                return False
            
            # Test feature extraction
            test_features = await self.feature_extractor.extract_features(Path(__file__))
            if not test_features:
                log.warning("Feature extraction failed during health check")
                return False
            
            # Test prediction
            test_result = await self._predict_threat(test_features)
            if not test_result:
                log.warning("Prediction failed during health check")
                return False
            
            return True
            
        except Exception as e:
            log.error(f"Health check failed: {e}")
            return False