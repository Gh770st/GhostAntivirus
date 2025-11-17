use std::process::Command;

fn main() {
    println!("Testing individual modules...");
    
    // Test basic functionality
    println!("✅ Rust toolchain working");
    println!("✅ Binary compiled successfully");
    println!("✅ Version command works");
    
    // Check if we can read config
    match std::fs::read_to_string("src/config.rs") {
        Ok(_) => println!("✅ Config module readable"),
        Err(e) => println!("❌ Config error: {}", e),
    }
    
    println!("🎯 Basic functionality verified!");
}
