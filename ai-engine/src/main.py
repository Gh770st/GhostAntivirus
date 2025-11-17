"""
GhostAntivirus AI Engine
Main entry point for the AI-powered threat detection system
"""

import asyncio
import logging
from pathlib import Path
from typing import Optional

import click
import structlog
from rich.console import Console
from rich.logging import RichHandler

from .engine import AIEngine
from .config import Config

# Setup rich console and logging
console = Console()
logging.basicConfig(
    level=logging.INFO,
    format="%(message)s",
    datefmt="[%X]",
    handlers=[RichHandler(console=console, rich_tracebacks=True)]
)
log = structlog.get_logger()


@click.group()
@click.option('--config', '-c', default='config.toml', help='Configuration file path')
@click.option('--log-level', default='INFO', help='Logging level')
@click.pass_context
def cli(ctx, config: str, log_level: str):
    """GhostAntivirus AI Engine - AI-powered threat detection system"""
    
    # Set logging level
    logging.getLogger().setLevel(getattr(logging, log_level.upper()))
    
    # Load configuration
    try:
        config_path = Path(config)
        ai_config = Config.load(config_path)
        ctx.ensure_object(dict)
        ctx.obj['config'] = ai_config
        ctx.obj['config_path'] = config_path
    except Exception as e:
        console.print(f"[red]Error loading configuration: {e}[/red]")
        ctx.exit(1)


@cli.command()
@click.option('--host', default='0.0.0.0', help='Host to bind to')
@click.option('--port', default=8000, help='Port to bind to')
@click.option('--workers', default=1, help='Number of worker processes')
@click.pass_context
def serve(ctx, host: str, port: int, workers: int):
    """Start the AI engine web server"""
    
    config: Config = ctx.obj['config']
    
    console.print(f"[green]Starting GhostAntivirus AI Engine Server[/green]")
    console.print(f"[blue]Host: {host}[/blue]")
    console.print(f"[blue]Port: {port}[/blue]")
    console.print(f"[blue]Workers: {workers}[/blue]")
    
    try:
        engine = AIEngine(config)
        
        # Run the server
        import uvicorn
        uvicorn.run(
            "ai_engine.api:app",
            host=host,
            port=port,
            workers=workers,
            log_level="info"
        )
        
    except Exception as e:
        console.print(f"[red]Failed to start server: {e}[/red]")
        raise click.ClickException(str(e))


@cli.command()
@click.option('--model-path', required=True, help='Path to the model file')
@click.option('--data-path', required=True, help='Path to training data')
@click.option('--epochs', default=10, help='Number of training epochs')
@click.option('--batch-size', default=32, help='Batch size for training')
@click.pass_context
def train(ctx, model_path: str, data_path: str, epochs: int, batch_size: int):
    """Train a new AI model"""
    
    config: Config = ctx.obj['config']
    
    console.print(f"[green]Training AI Model[/green]")
    console.print(f"[blue]Model Path: {model_path}[/blue]")
    console.print(f"[blue]Data Path: {data_path}[/blue]")
    console.print(f"[blue]Epochs: {epochs}[/blue]")
    console.print(f"[blue]Batch Size: {batch_size}[/blue]")
    
    try:
        from .training import ModelTrainer
        
        trainer = ModelTrainer(config)
        model = trainer.train(
            data_path=Path(data_path),
            output_path=Path(model_path),
            epochs=epochs,
            batch_size=batch_size
        )
        
        console.print(f"[green]✓ Model training completed[/green]")
        console.print(f"[blue]Model saved to: {model_path}[/blue]")
        
    except Exception as e:
        console.print(f"[red]Training failed: {e}[/red]")
        raise click.ClickException(str(e))


@cli.command()
@click.argument('file_path')
@click.option('--model-path', help='Path to model file (optional)')
@click.pass_context
def analyze(ctx, file_path: str, model_path: Optional[str]):
    """Analyze a single file for threats"""
    
    config: Config = ctx.obj['config']
    
    console.print(f"[green]Analyzing file: {file_path}[/green]")
    
    try:
        engine = AIEngine(config)
        
        # Load model if specified
        if model_path:
            engine.load_model(Path(model_path))
        
        # Analyze file
        result = asyncio.run(engine.analyze_file(Path(file_path)))
        
        console.print(f"\n[green]Analysis Results:[/green]")
        console.print(f"[blue]File: {result.file_path}[/blue]")
        console.print(f"[blue]Threat Score: {result.threat_score:.2f}[/blue]")
        console.print(f"[blue]Prediction: {result.prediction}[/blue]")
        console.print(f"[blue]Confidence: {result.confidence:.2f}[/blue]")
        
        if result.features:
            console.print(f"\n[blue]Top Features:[/blue]")
            for feature, value in list(result.features.items())[:5]:
                console.print(f"  {feature}: {value:.4f}")
        
    except Exception as e:
        console.print(f"[red]Analysis failed: {e}[/red]")
        raise click.ClickException(str(e))


@cli.command()
@click.pass_context
def status(ctx):
    """Show AI engine status"""
    
    config: Config = ctx.obj['config']
    
    console.print("[green]GhostAntivirus AI Engine Status[/green]")
    console.print(f"[blue]Version: 3.0.0[/blue]")
    console.print(f"[blue]Config File: {ctx.obj['config_path']}[/blue]")
    console.print(f"[blue]Model Path: {config.model.path}[/blue]")
    console.print(f"[blue]API Host: {config.api.host}[/blue]")
    console.print(f"[blue]API Port: {config.api.port}[/blue]")
    
    # Check if model exists
    model_path = Path(config.model.path)
    if model_path.exists():
        console.print(f"[green]✓ Model loaded[/green]")
    else:
        console.print(f"[yellow]⚠ Model not found at {config.model.path}[/yellow]")


@cli.command()
@click.option('--output', '-o', default='config.toml', help='Output configuration file')
@click.pass_context
def init_config(ctx, output: str):
    """Initialize default configuration file"""
    
    console.print(f"[green]Creating default configuration: {output}[/green]")
    
    try:
        config = Config.default()
        config.save(Path(output))
        console.print(f"[green]✓ Configuration file created[/green]")
        
    except Exception as e:
        console.print(f"[red]Failed to create config: {e}[/red]")
        raise click.ClickException(str(e))


def main():
    """Main entry point"""
    try:
        cli()
    except KeyboardInterrupt:
        console.print("\n[yellow]Operation cancelled by user[/yellow]")
    except Exception as e:
        console.print(f"\n[red]Fatal error: {e}[/red]")
        raise


if __name__ == "__main__":
    main()