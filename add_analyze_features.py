#!/usr/bin/env python3

with open('core/src/ai.rs', 'r') as f:
    content = f.read()

# Find where to insert the new method (before analyze_batch)
insert_point = content.find("    /// Analyze multiple files in batch")

if insert_point != -1:
    # Insert the analyze_features method
    new_method = '''    /// Analyze file using pre-extracted features
    pub async fn analyze_features(&self, features: &FileFeatures) -> Result<Option<AnalysisResult>> {
        if !self.enabled {
            return Ok(None);
        }
        
        debug!("Analyzing features for file: {} (size: {})", features.file_hash, features.file_size);
        
        // Simple heuristic analysis (in production, use real ML models)
        let mut threat_score = 0.0;
        let mut reasons = Vec::new();
        
        // Check entropy (high entropy often indicates packing/encryption)
        if features.entropy > 7.5 {
            threat_score += 0.3;
            reasons.push("High entropy (possible packing)".to_string());
        }
        
        // Check PE features
        if let Some(pe) = &features.pe_features {
            if pe.is_pe && !pe.has_pe_header {
                threat_score += 0.4;
                reasons.push("Invalid PE header".to_string());
            }
        }
        
        // Check suspicious strings
        let suspicious_count = features.string_patterns.len();
        if suspicious_count > 5 {
            threat_score += 0.2 * (suspicious_count as f64 / 10.0);
            reasons.push(format!("{} suspicious strings", suspicious_count));
        }
        
        // Check file size (very small executables are suspicious)
        if features.file_extension == "exe" && features.file_size < 10240 {
            threat_score += 0.3;
            reasons.push("Very small executable".to_string());
        }
        
        // Determine if it's a threat
        if threat_score > 0.5 {
            Ok(Some(AnalysisResult {
                is_threat: true,
                threat_type: if features.pe_features.is_some() {
                    "Malware"
                } else {
                    "Suspicious"
                }.to_string(),
                severity: if threat_score > 0.8 {
                    "High".to_string()
                } else if threat_score > 0.6 {
                    "Medium".to_string()
                } else {
                    "Low".to_string()
                },
                confidence: threat_score.min(1.0),
                reason: reasons.join("; "),
            }))
        } else {
            Ok(None)
        }
    }
    
'''
    
    # Insert the method
    content = content[:insert_point] + new_method + content[insert_point:]
    
    with open('core/src/ai.rs', 'w') as f:
        f.write(content)
    
    print("✅ Added analyze_features method")
else:
    print("❌ Could not find insertion point")