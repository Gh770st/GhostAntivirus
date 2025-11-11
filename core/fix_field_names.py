import re

def fix_field_names():
    file_path = "tests/security_validation.rs"
    
    with open(file_path, 'r') as f:
        content = f.read()
    
    # Fix field names to match actual ScanStats struct
    content = re.sub(r'threats_found', 'threats_detected', content)
    content = re.sub(r'files_scanned', 'total_files_scanned', content)
    
    # Fix unused variable
    content = re.sub(r'let scanner = Scanner::new\(config\)\.expect\("Failed to create scanner"\);', 
                     'let _scanner = Scanner::new(config).expect("Failed to create scanner");', content)
    
    with open(file_path, 'w') as f:
        f.write(content)
    
    print("Fixed field names")

if __name__ == "__main__":
    fix_field_names()