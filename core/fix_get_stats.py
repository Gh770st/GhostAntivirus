import re

def fix_get_stats():
    file_path = "tests/security_validation.rs"
    
    with open(file_path, 'r') as f:
        content = f.read()
    
    # Fix get_stats() calls - remove .await
    content = re.sub(r'\.get_stats\(\)\.await', '.get_stats()', content)
    
    with open(file_path, 'w') as f:
        f.write(content)
    
    print("Fixed get_stats calls")

if __name__ == "__main__":
    fix_get_stats()