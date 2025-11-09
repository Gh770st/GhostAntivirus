#!/usr/bin/env python3

with open('core/src/quarantine.rs', 'r') as f:
    content = f.read()

# Fix the test to use different content for each file
old_code = '''        // Create and quarantine multiple files
        for i in 0..3 {
            let test_file = create_test_file(
                temp_dir.path(), 
                &format!("test{}.txt", i), 
                b"Test"
            );
            manager.quarantine_file(&test_file, "Test Threat").unwrap();
        }'''

new_code = '''        // Create and quarantine multiple files with DIFFERENT content
        for i in 0..3 {
            let content = format!("Test content {}", i);
            let test_file = create_test_file(
                temp_dir.path(), 
                &format!("test{}.txt", i), 
                content.as_bytes()
            );
            manager.quarantine_file(&test_file, "Test Threat").unwrap();
        }'''

content = content.replace(old_code, new_code)

with open('core/src/quarantine.rs', 'w') as f:
    f.write(content)

print("Fixed quarantine test - now uses unique content for each file")