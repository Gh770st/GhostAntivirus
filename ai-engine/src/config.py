"""
AI Engine Configuration Management
"""

import os
from pathlib import Path
from typing import Optional, List, Dict, Any
import toml
from pydantic import BaseModel, Field
import structlog

log = structlog.get_logger()


class ModelConfig(BaseModel):
    """Model configuration settings"""
    path: str = Field(default="models/threat_classifier.h5", description="Path to trained model")
    type: str = Field(default="neural_network", description="Model type")
    input_features: int = Field(default=300, description="Number of input features")
    hidden_layers: List[int] = Field(default=[256, 128, 64], description="Hidden layer sizes")
    dropout_rate: float = Field(default=0.3, description="Dropout rate")
    batch_size: int = Field(default=32, description="Batch size for predictions")
    confidence_threshold: float = Field(default=0.7, description="Minimum confidence for positive prediction")


class APIConfig(BaseModel):
    """API server configuration"""
    host: str = Field(default="0.0.0.0", description="API server host")
    port: int = Field(default=8000, description="API server port")
    workers: int = Field(default=1, description="Number of worker processes")
    reload: bool = Field(default=False, description="Enable auto-reload for development")
    access_log: bool = Field(default=True, description="Enable access logging")
    max_request_size: int = Field(default=10 * 1024 * 1024, description="Max request size in bytes")
    timeout: int = Field(default=30, description="Request timeout in seconds")


class DatabaseConfig(BaseModel):
    """Database configuration"""
    url: str = Field(default="sqlite:///ai_engine.db", description="Database connection URL")
    echo: bool = Field(default=False, description="Enable query logging")
    pool_size: int = Field(default=5, description="Connection pool size")
    max_overflow: int = Field(default=10, description="Max overflow connections")


class SecurityConfig(BaseModel):
    """Security configuration"""
    api_key: Optional[str] = Field(default=None, description="API authentication key")
    rate_limit: int = Field(default=100, description="Rate limit requests per minute")
    enable_cors: bool = Field(default=True, description="Enable CORS")
    allowed_origins: List[str] = Field(default=["*"], description="Allowed CORS origins")
    max_file_size: int = Field(default=100 * 1024 * 1024, description="Max file size for upload")


class LoggingConfig(BaseModel):
    """Logging configuration"""
    level: str = Field(default="INFO", description="Log level")
    format: str = Field(default="rich", description="Log format")
    file: Optional[str] = Field(default=None, description="Log file path")
    max_size: int = Field(default=10 * 1024 * 1024, description="Max log file size")
    backup_count: int = Field(default=5, description="Number of log backups")


class TrainingConfig(BaseModel):
    """Training configuration"""
    data_dir: str = Field(default="data/training", description="Training data directory")
    model_output_dir: str = Field(default="models", description="Model output directory")
    epochs: int = Field(default=10, description="Number of training epochs")
    batch_size: int = Field(default=32, description="Training batch size")
    validation_split: float = Field(default=0.2, description="Validation data split")
    learning_rate: float = Field(default=0.001, description="Learning rate")
    early_stopping_patience: int = Field(default=5, description="Early stopping patience")


class MonitoringConfig(BaseModel):
    """Monitoring configuration"""
    enable_metrics: bool = Field(default=True, description="Enable metrics collection")
    metrics_port: int = Field(default=9090, description="Metrics server port")
    health_check_interval: int = Field(default=30, description="Health check interval in seconds")


class Config(BaseModel):
    """Main AI Engine configuration"""
    model: ModelConfig = Field(default_factory=ModelConfig)
    api: APIConfig = Field(default_factory=APIConfig)
    database: DatabaseConfig = Field(default_factory=DatabaseConfig)
    security: SecurityConfig = Field(default_factory=SecurityConfig)
    logging: LoggingConfig = Field(default_factory=LoggingConfig)
    training: TrainingConfig = Field(default_factory=TrainingConfig)
    monitoring: MonitoringConfig = Field(default_factory=MonitoringConfig)
    
    @classmethod
    def load(cls, config_path: Path) -> "Config":
        """Load configuration from file"""
        try:
            if config_path.exists():
                with open(config_path, 'r') as f:
                    config_data = toml.load(f)
                return cls(**config_data)
            else:
                log.warning(f"Config file not found: {config_path}, using defaults")
                return cls.default()
        except Exception as e:
            log.error(f"Failed to load config from {config_path}: {e}")
            return cls.default()
    
    @classmethod
    def default(cls) -> "Config":
        """Create default configuration"""
        return cls()
    
    def save(self, config_path: Path) -> None:
        """Save configuration to file"""
        try:
            # Create directory if it doesn't exist
            config_path.parent.mkdir(parents=True, exist_ok=True)
            
            with open(config_path, 'w') as f:
                toml.dump(self.dict(), f)
            
            log.info(f"Configuration saved to: {config_path}")
        except Exception as e:
            log.error(f"Failed to save config to {config_path}: {e}")
            raise
    
    def get_environment_overrides(self) -> Dict[str, Any]:
        """Get configuration overrides from environment variables"""
        overrides = {}
        
        # Model overrides
        if os.getenv("AI_MODEL_PATH"):
            overrides["model"]["path"] = os.getenv("AI_MODEL_PATH")
        
        # API overrides
        if os.getenv("AI_API_HOST"):
            overrides["api"]["host"] = os.getenv("AI_API_HOST")
        if os.getenv("AI_API_PORT"):
            overrides["api"]["port"] = int(os.getenv("AI_API_PORT"))
        
        # Database overrides
        if os.getenv("AI_DATABASE_URL"):
            overrides["database"]["url"] = os.getenv("AI_DATABASE_URL")
        
        # Security overrides
        if os.getenv("AI_API_KEY"):
            overrides["security"]["api_key"] = os.getenv("AI_API_KEY")
        
        return overrides
    
    def apply_environment_overrides(self) -> "Config":
        """Apply environment variable overrides"""
        overrides = self.get_environment_overrides()
        if overrides:
            config_dict = self.dict()
            self._deep_update(config_dict, overrides)
            return Config(**config_dict)
        return self
    
    def _deep_update(self, base_dict: Dict, update_dict: Dict) -> None:
        """Deep update dictionary"""
        for key, value in update_dict.items():
            if key in base_dict and isinstance(base_dict[key], dict) and isinstance(value, dict):
                self._deep_update(base_dict[key], value)
            else:
                base_dict[key] = value
    
    def validate(self) -> bool:
        """Validate configuration"""
        try:
            # Check model path
            model_path = Path(self.model.path)
            if not model_path.parent.exists():
                log.warning(f"Model directory does not exist: {model_path.parent}")
            
            # Check API configuration
            if not (1 <= self.api.port <= 65535):
                log.error(f"Invalid API port: {self.api.port}")
                return False
            
            # Check database configuration
            if not self.database.url:
                log.error("Database URL is required")
                return False
            
            # Check training configuration
            training_path = Path(self.training.data_dir)
            if not training_path.exists():
                log.warning(f"Training data directory does not exist: {training_path}")
            
            return True
        except Exception as e:
            log.error(f"Configuration validation failed: {e}")
            return False


def create_default_config(config_path: Path) -> Config:
    """Create and save default configuration"""
    config = Config.default()
    config.save(config_path)
    log.info(f"Default configuration created at: {config_path}")
    return config


# Example usage and testing
if __name__ == "__main__":
    # Test configuration creation
    config = Config.default()
    print("Default configuration:")
    print(config.json(indent=2))
    
    # Test environment overrides
    os.environ["AI_API_PORT"] = "9000"
    os.environ["AI_MODEL_PATH"] = "/custom/model/path"
    
    config_with_overrides = config.apply_environment_overrides()
    print("\nConfiguration with environment overrides:")
    print(f"API Port: {config_with_overrides.api.port}")
    print(f"Model Path: {config_with_overrides.model.path}")