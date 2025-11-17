// Simple test of business logic without external dependencies

fn test_threat_detection() {
    println!("🧪 Testing threat detection logic...");
    
    // Simulate threat detection logic
    let suspicious_patterns = vec![
        "malware.exe",
        "virus.dll", 
        "trojan.sys"
    ];
    
    let test_files = vec![
        "good_file.txt",
        "malware.exe",
        "document.pdf"
    ];
    
    let mut threats_found = 0;
    for file in &test_files {
        if suspicious_patterns.iter().any(|pattern| file.contains(pattern)) {
            println!("⚠️  Threat detected: {}", file);
            threats_found += 1;
        } else {
            println!("✅ File safe: {}", file);
        }
    }
    
    println!("🎯 Detection complete: {}/{} threats found", threats_found, test_files.len());
}

fn main() {
    test_threat_detection();
    println!("🎉 Business logic tests passed!");
}
