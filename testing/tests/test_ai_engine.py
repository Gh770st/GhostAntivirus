"""
Test suite for GhostAntivirus AI Engine
"""

import pytest
import asyncio
from pathlib import Path
from unittest.mock import Mock, patch, AsyncMock
import tempfile
import shutil

# Import AI Engine modules
import sys
sys.path.append(str(Path(__file__).parent.parent.parent / "ai-engine" / "src"))

from ai_engine.engine import AIEngine
from ai_engine.config import Config
from ai_engine.models import AnalysisResult, ThreatType, Severity


class TestAIEngine:
    """Test cases for AI Engine"""
    
    @pytest.fixture
    def config(self):
        """Create test configuration"""
        return Config.default()
    
    @pytest.fixture
    def ai_engine(self, config):
        """Create AI Engine instance"""
        return AIEngine(config)
    
    @pytest.fixture
    def sample_text_file(self):
        """Create a sample text file for testing"""
        with tempfile.NamedTemporaryFile(mode='w', suffix='.txt', delete=False) as f:
            f.write("This is a harmless text file for testing.")
            temp_file = f.name
        
        yield Path(temp_file)
        Path(temp_file).unlink(missing_ok=True)
    
    @pytest.fixture
    def sample_exe_file(self):
        """Create a sample executable file for testing"""
        with tempfile.NamedTemporaryFile(mode='wb', suffix='.exe', delete=False) as f:
            # Write a minimal PE header
            f.write(b"MZ\x90\x00\x03\x00\x00\x00\x04\x00\x00\x00\xff\xff")
            f.write(b"\x00\x00\xb8\x00\x00\x00\x00\x00\x00\x00@\x00\x00\x00")
            f.write(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00")
            f.write(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x80\x00")
            f.write(b"\x00\x00\x0e\x1f\xba\x0e\x00\xb4\t\xcd!\xb8\x01L\xcd!")
            f.write(b"This program cannot be run in DOS mode.\r\r\n$")
            f.write(b"\x00\x00\x00\x00\x00\x00\x00PE\x00\x00L\x01\x03\x00")
            temp_file = f.name
        
        yield Path(temp_file)
        Path(temp_file).unlink(missing_ok=True)
    
    @pytest.fixture
    def eicar_file(self):
        """Create EICAR test file"""
        with tempfile.NamedTemporaryFile(mode='w', suffix='.txt', delete=False) as f:
            eicar_string = "X5O!P%@AP[4\\PZX54(P^)7CC)7}$EICAR-STANDARD-ANTIVIRUS-TEST-FILE!$H+H*"
            f.write(eicar_string)
            temp_file = f.name
        
        yield Path(temp_file)
        Path(temp_file).unlink(missing_ok=True)
    
    def test_engine_initialization(self, ai_engine):
        """Test AI Engine initialization"""
        assert ai_engine is not None
        assert ai_engine.config is not None
        assert ai_engine.feature_extractor is not None
        assert not ai_engine.is_model_loaded()  # Model not loaded by default
    
    def test_engine_status(self, ai_engine):
        """Test engine status information"""
        status = ai_engine.get_engine_status()
        
        assert isinstance(status, dict)
        assert "model_loaded" in status
        assert "model_type" in status
        assert "version" in status
        assert status["version"] == "3.0.0"
    
    @pytest.mark.asyncio
    async def test_analyze_text_file(self, ai_engine, sample_text_file):
        """Test analysis of a harmless text file"""
        result = await ai_engine.analyze_file(sample_text_file)
        
        assert isinstance(result, AnalysisResult)
        assert result.file_path == str(sample_text_file)
        assert result.threat_score >= 0.0 and result.threat_score <= 100.0
        assert result.prediction in ["benign", "malicious", "error"]
        assert result.confidence >= 0.0 and result.confidence <= 1.0
        assert result.model_version in ["3.0.0", "rule-based"]
        
        # Text file should be benign
        assert result.threat_score < 50.0
        assert result.prediction == "benign"
    
    @pytest.mark.asyncio
    async def test_analyze_executable_file(self, ai_engine, sample_exe_file):
        """Test analysis of an executable file"""
        result = await ai_engine.analyze_file(sample_exe_file)
        
        assert isinstance(result, AnalysisResult)
        assert result.file_path == str(sample_exe_file)
        
        # Executable should have some features extracted
        assert result.features is not None
        assert len(result.features) > 0
        
        # Should detect as executable
        assert result.features.get("is_executable", False)
    
    @pytest.mark.asyncio
    async def test_analyze_nonexistent_file(self, ai_engine):
        """Test analysis of nonexistent file"""
        nonexistent_path = Path("/nonexistent/file.txt")
        
        with pytest.raises(FileNotFoundError):
            await ai_engine.analyze_file(nonexistent_path)
    
    @pytest.mark.asyncio
    async def test_analyze_eicar_file(self, ai_engine, eicar_file):
        """Test analysis of EICAR test file"""
        result = await ai_engine.analyze_file(eicar_file)
        
        # EICAR should be detected (high threat score)
        assert result.threat_score > 50.0
        # Depending on the model, might be detected as malicious
        # but rule-based should give it a moderate score
    
    @pytest.mark.asyncio
    async def test_batch_analysis(self, ai_engine, sample_text_file, sample_exe_file):
        """Test batch file analysis"""
        file_paths = [sample_text_file, sample_exe_file]
        results = await ai_engine.analyze_batch(file_paths)
        
        assert len(results) == 2
        for result in results:
            assert isinstance(result, AnalysisResult)
            assert result.file_path in [str(sample_text_file), str(sample_exe_file)]
    
    @pytest.mark.asyncio
    async def test_health_check(self, ai_engine):
        """Test health check functionality"""
        # Health check should work even without model loaded
        is_healthy = await ai_engine.health_check()
        assert isinstance(is_healthy, bool)
    
    def test_features_to_array(self, ai_engine):
        """Test feature conversion to array"""
        features = {
            "file_size": 1024,
            "is_executable": True,
            "entropy": 7.5,
            "some_other_feature": "test"
        }
        
        array = ai_engine._features_to_array(features)
        
        assert isinstance(array, np.ndarray)
        assert len(array) == ai_engine.config.model.input_features
        assert array.dtype == np.float32
    
    def test_rule_based_analysis(self, ai_engine):
        """Test rule-based analysis fallback"""
        features = {
            "file_size": 100,
            "is_executable": True,
            "has_digital_signature": False,
            "entropy": 8.0,
            "suspicious_strings": 5
        }
        
        result = ai_engine._rule_based_analysis(Path("test.exe"), features)
        
        assert "threat_score" in result
        assert "prediction" in result
        assert "confidence" in result
        assert result["threat_score"] >= 0.0 and result["threat_score"] <= 100.0
    
    def test_classify_threat(self, ai_engine):
        """Test threat classification"""
        # Test low threat
        threat_type, severity = ai_engine._classify_threat(20.0, {})
        assert severity == Severity.LOW
        
        # Test high threat
        threat_type, severity = ai_engine._classify_threat(85.0, {})
        assert severity == Severity.CRITICAL
        
        # Test with high entropy
        threat_type, severity = ai_engine._classify_threat(60.0, {"entropy": 8.0})
        assert threat_type == ThreatType.TROJAN


class TestFeatureExtractor:
    """Test cases for Feature Extractor"""
    
    @pytest.fixture
    def config(self):
        """Create test configuration"""
        return Config.default()
    
    @pytest.fixture
    def feature_extractor(self, config):
        """Create feature extractor instance"""
        from ai_engine.features import FeatureExtractor
        return FeatureExtractor(config)
    
    @pytest.fixture
    def sample_file(self):
        """Create a sample file"""
        with tempfile.NamedTemporaryFile(mode='w', suffix='.txt', delete=False) as f:
            f.write("Sample file content for feature extraction testing.")
            temp_file = f.name
        
        yield Path(temp_file)
        Path(temp_file).unlink(missing_ok=True)
    
    def test_extractor_initialization(self, feature_extractor):
        """Test feature extractor initialization"""
        assert feature_extractor is not None
        assert feature_extractor.config is not None
        assert isinstance(feature_extractor.get_supported_features(), list)
    
    @pytest.mark.asyncio
    async def test_extract_basic_features(self, feature_extractor, sample_file):
        """Test basic feature extraction"""
        features = await feature_extractor._extract_basic_features(sample_file)
        
        assert isinstance(features, dict)
        assert "file_size" in features
        assert "file_age_days" in features
        assert "path_depth" in features
        assert "filename_length" in features
        assert "contains_spaces" in features
        assert features["file_size"] > 0
    
    @pytest.mark.asyncio
    async def test_extract_content_features(self, feature_extractor, sample_file):
        """Test content feature extraction"""
        features = await feature_extractor._extract_content_features(sample_file)
        
        assert isinstance(features, dict)
        assert "file_hash" in features
        assert "mime_type" in features
        assert "entropy" in features
        assert "is_executable" in features
        assert features["file_hash"] is not None
        assert len(features["file_hash"]) == 64  # SHA256 hex length
    
    @pytest.mark.asyncio
    async def test_entropy_calculation(self, feature_extractor):
        """Test entropy calculation"""
        # Test with known data
        data = b"AAAAAA"  # Low entropy
        entropy = feature_extractor._calculate_entropy(data)
        assert entropy == 0.0
        
        # Test with high entropy data
        data = bytes(range(256))  # Maximum entropy
        entropy = feature_extractor._calculate_entropy(data)
        assert entropy > 7.0
    
    def test_executable_detection(self, feature_extractor):
        """Test executable file detection"""
        # Test PE executable
        pe_data = b"MZ\x90\x00" + b"\x00" * 100
        assert feature_extractor._is_executable(pe_data, ".exe")
        
        # Test text file
        text_data = b"This is a text file"
        assert not feature_extractor._is_executable(text_data, ".txt")
    
    def test_extension_risk(self, feature_extractor):
        """Test extension risk scoring"""
        # High risk extensions
        assert feature_extractor._get_extension_risk(".exe") >= 7.0
        assert feature_extractor._get_extension_risk(".scr") >= 8.0
        
        # Low risk extensions
        assert feature_extractor._get_extension_risk(".txt") <= 2.0
        assert feature_extractor._get_extension_risk(".doc") <= 5.0


class TestAPI:
    """Test cases for API endpoints"""
    
    @pytest.fixture
    def config(self):
        """Create test configuration"""
        config = Config.default()
        config.api.port = 8001  # Use different port for testing
        return config
    
    @pytest.fixture
    def mock_ai_engine(self):
        """Create mock AI engine"""
        engine = Mock()
        engine.is_model_loaded.return_value = False
        engine.get_engine_status.return_value = {
            "model_loaded": False,
            "model_type": "neural_network",
            "version": "3.0.0"
        }
        engine.health_check = AsyncMock(return_value=True)
        return engine
    
    def test_health_endpoint(self, mock_ai_engine):
        """Test health check endpoint"""
        # Test with mock engine
        from ai_engine.api import HealthResponse
        
        response = HealthResponse(
            status="healthy",
            timestamp="2023-01-01 00:00:00",
            engine_ready=True,
            model_loaded=False
        )
        
        assert response.status == "healthy"
        assert response.engine_ready is True


if __name__ == "__main__":
    # Run tests
    pytest.main([__file__, "-v", "--tb=short"])