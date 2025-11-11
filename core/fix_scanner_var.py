import re

def fix_scanner_var():
    file_path = "tests/security_validation.rs"
    
    with open(file_path, 'r') as f:
        content = f.read()
    
    # Fix all _scanner to scanner (except the one that's actually unused)
    content = re.sub(r'let _scanner = Scanner::new\(config\)\.expect\("Failed to create scanner"\);', 
                     'let scanner = Scanner::new(config).expect("Failed to create scanner");', content)
    
    with open(file_path, 'w') as f:
        f.write(content)
    
    print("Fixed scanner variable names")

if __name__ == "__main__":
    fix_scanner_var()