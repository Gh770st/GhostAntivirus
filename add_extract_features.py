#!/usr/bin/env python3

with open('core/src/scanner.rs', 'r') as f:
    content = f.read()

# Add the extract_file_features method before the calculate_file_hash method
# Find the location to insert (before calculate_file_hash)
insert_point = content.find('    /// Calculate file hash')

if insert_point != -1:
    # Insert the new method
    new_method = '''    /// Extract features from file for AI analysis
    fn extract_file_features(&self, file_path: &Path, content: &[u8]) -> Result<ai::FileFeatures> {
        let file_size = content.len();
        let file_hash = self.calculate_hash(content);
        let file_extension = file_path
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("");
        
        // Extract PE features for executables
        let mut pe_features = None;
        if file_extension == "exe" || file_extension == "dll" {
            pe_features = Some(self.extract_pe_features(content));
        }
        
        // Extract string patterns
        let string_patterns = self.extract_string_patterns(content);
        
        // Calculate entropy
        let entropy = self.calculate_entropy(content);
        
        Ok(ai::FileFeatures {
            file_hash,
            file_size: file_size as u64,
            file_extension: file_extension.to_string(),
            pe_features,
            string_patterns,
            entropy,
        })
    }
    
    /// Extract PE header features for Windows executables
    fn extract_pe_features(&self, content: &[u8]) -> ai::PEFeatures {
        // Simple PE parsing - in production, use a proper PE library
        let mut features = ai::PEFeatures::default();
        
        if content.len() > 64 {
            // Check for MZ header
            if content[0] == b'M' && content[1] == b'Z' {
                features.is_pe = true;
                
                // Extract some basic PE info
                if content.len() > 60 {
                    let pe_offset = u32::from_le_bytes([content[60], content[61], content[62], content[63]]) as usize;
                    if pe_offset < content.len() - 4 {
                        features.has_pe_header = content[pe_offset] == b'P' && content[pe_offset + 1] == b'E';
                    }
                }
            }
        }
        
        features
    }
    
    /// Extract string patterns from file content
    fn extract_string_patterns(&self, content: &[u8]) -> Vec<String> {
        let mut patterns = Vec::new();
        let mut current_string = String::new();
        
        for &byte in content {
            if byte.is_ascii_graphic() {
                current_string.push(byte as char);
            } else {
                if current_string.len() >= 4 {
                    patterns.push(current_string.clone());
                }
                current_string.clear();
            }
        }
        
        // Collect common suspicious patterns
        patterns.retain(|s| {
            s.len() >= 4 && (
                s.to_lowercase().contains("http") ||
                s.to_lowercase().contains("ftp") ||
                s.to_lowercase().contains("password") ||
                s.to_lowercase().contains("key") ||
                s.to_lowercase().contains("encrypt") ||
                s.to_lowercase().contains("decode")
            )
        });
        
        patterns.truncate(10); // Limit to top 10 patterns
        patterns
    }
    
    /// Calculate Shannon entropy of data
    fn calculate_entropy(&self, data: &[u8]) -> f64 {
        use std::collections::HashMap;
        
        let mut freq = HashMap::new();
        let len = data.len() as f64;
        
        // Count frequency of each byte
        for &byte in data {
            *freq.entry(byte).or_insert(0) += 1;
        }
        
        // Calculate entropy
        let mut entropy = 0.0;
        for &count in freq.values() {
            let p = count as f64 / len;
            entropy -= p * p.log2();
        }
        
        entropy
    }
    
'''
    
    # Insert the method
    content = content[:insert_point] + new_method + content[insert_point:]
    
    with open('core/src/scanner.rs', 'w') as f:
        f.write(content)
    
    print("✅ Added extract_file_features method and supporting functions")
else:
    print("❌ Could not find insertion point for extract_file_features")