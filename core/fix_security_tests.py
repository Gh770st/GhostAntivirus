import re

def fix_security_tests():
    file_path = "tests/security_validation.rs"
    
    with open(file_path, 'r') as f:
        content = f.read()
    
    # Fix Scanner::new() calls - remove .await
    content = re.sub(r'Scanner::new\(config\)\.await', 'Scanner::new(config)', content)
    content = re.sub(r'Scanner::new\(config\.clone\(\)\)\.await', 'Scanner::new(config.clone())', content)
    
    # Fix ScanType::Deep to Full
    content = content.replace('ScanType::Deep', 'ScanType::Full')
    
    with open(file_path, 'w') as f:
        f.write(content)
    
    print("Fixed security tests")

if __name__ == "__main__":
    fix_security_tests()