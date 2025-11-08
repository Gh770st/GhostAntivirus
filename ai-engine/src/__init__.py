"""
GhostAntivirus AI Engine
AI-powered threat detection and analysis system
"""

__version__ = "3.0.0"
__author__ = "WiterDevelopment"
__email__ = "info@witerdev.com"

from .engine import AIEngine
from .config import Config
from .models import (
    AnalysisResult,
    ThreatType,
    Severity,
    BatchAnalysisRequest,
    BatchAnalysisResponse,
    SystemStatus,
    ScanRequest,
    ScanProgress,
    Alert,
    ModelMetrics
)

__all__ = [
    "AIEngine",
    "Config",
    "AnalysisResult",
    "ThreatType",
    "Severity",
    "BatchAnalysisRequest",
    "BatchAnalysisResponse",
    "SystemStatus",
    "ScanRequest",
    "ScanProgress",
    "Alert",
    "ModelMetrics",
]