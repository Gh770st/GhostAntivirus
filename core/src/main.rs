//! GhostAntivirus Core Engine Main Application

use anyhow::Result;
use clap::{Arg, Command};
use log::{info, error};
use tokio;
use ghost_core::GhostEngine;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    env_logger::init();
    
    // Parse command line arguments
    let matches = Command::new("ghost-core")
        .version("3.0.0")
        .about("GhostAntivirus Core Engine - Next-generation malware protection")
        .arg(
            Arg::new("scan")
                .short('s')
                .long("scan")
                .value_name("PATH")
                .help("Scan specific file or directory")
        )
        .arg(
            Arg::new("quick")
                .short('q')
                .long("quick")
                .help("Perform quick system scan")
                .action(clap::ArgAction::SetTrue)
        )
        .arg(
            Arg::new("full")
                .short('f')
                .long("full")
                .help("Perform full system scan")
                .action(clap::ArgAction::SetTrue)
        )
        .arg(
            Arg::new("status")
                .short('t')
                .long("status")
                .help("Show engine status")
                .action(clap::ArgAction::SetTrue)
        )
        .arg(
            Arg::new("daemon")
                .short('d')
                .long("daemon")
                .help("Run as background daemon")
                .action(clap::ArgAction::SetTrue)
        )
        .get_matches();
    
    info!("Starting GhostAntivirus Core Engine v{}", env!("CARGO_PKG_VERSION"));
    
    // Create engine instance
    let mut engine = match GhostEngine::new() {
        Ok(engine) => engine,
        Err(e) => {
            error!("Failed to initialize engine: {}", e);
            return Err(e);
        }
    };
    
    // Handle command line arguments
    if matches.get_flag("daemon") {
        // Run as daemon
        info!("Starting GhostAntivirus daemon mode");
        engine.start().await?;
        
        // Keep running until interrupted
        tokio::signal::ctrl_c().await?;
        info!("Received interrupt signal, shutting down...");
        
        engine.stop().await?;
        info!("GhostAntivirus daemon stopped");
        
    } else if matches.get_flag("status") {
        // Show status
        let status = engine.get_status();
        println!("GhostAntivirus Engine Status:");
        println!("  Version: {}", status.version);
        println!("  Running: {}", status.is_running);
        println!("  Files Scanned: {}", status.total_files_scanned);
        println!("  Threats Detected: {}", status.threats_detected);
        println!("  Quarantine Count: {}", status.quarantine_count);
        
    } else if let Some(path) = matches.get_one::<String>("scan") {
        // Scan specific path
        let result = engine.scan_path(path).await?;
        print_scan_result(&result);
        
    } else if matches.get_flag("quick") {
        // Quick scan
        let result = engine.quick_scan().await?;
        print_scan_result(&result);
        
    } else if matches.get_flag("full") {
        // Full scan
        let result = engine.full_scan().await?;
        print_scan_result(&result);
        
    } else {
        // Interactive mode
        println!("GhostAntivirus Core Engine v{}", env!("CARGO_PKG_VERSION"));
        println!("Use --help for available commands");
        println!();
        println!("Examples:");
        println!("  ghost-core --quick                    # Quick system scan");
        println!("  ghost-core --full                     # Full system scan");
        println!("  ghost-core --scan /path/to/scan       # Scan specific path");
        println!("  ghost-core --status                   # Show engine status");
        println!("  ghost-core --daemon                   # Run as daemon");
    }
    
    Ok(())
}

/// Print scan results in a formatted way
fn print_scan_result(result: &ghost_core::scanner::ScanResult) {
    println!("\nScan Results:");
    println!("  Scan Type: {:?}", result.scan_type);
    println!("  Files Scanned: {}", result.files_scanned);
    println!("  Threats Found: {}", result.threats_found.len());
    
    if !result.threats_found.is_empty() {
        println!("\nThreats Detected:");
        for (i, threat) in result.threats_found.iter().enumerate() {
            println!("  {}. {}", i + 1, threat.file_path.display());
            println!("     Type: {:?}", threat.threat_type);
            println!("     Name: {}", threat.threat_name);
            println!("     Severity: {:?}", threat.severity);
            println!("     Hash: {}", threat.hash);
            println!("     Size: {} bytes", threat.size);
            println!();
        }
    }
    
    if !result.errors.is_empty() {
        println!("Errors:");
        for error in &result.errors {
            println!("  - {}", error);
        }
    }
    
    let duration = result.end_time - result.start_time;
    println!("Scan completed in {} seconds", duration);
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_command_parsing() {
        use clap::Parser;
        
        let app = Command::new("test")
            .arg(Arg::new("scan").long("scan"))
            .arg(Arg::new("quick").long("scan"));
            
        // This would need proper testing setup
        // For now, just ensure the function compiles
    }
}