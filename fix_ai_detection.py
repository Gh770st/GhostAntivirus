#!/usr/bin/env python3

with open('core/src/scanner.rs', 'r') as f:
    content = f.read()

# Replace the AI detection section
old_code = '''           // AI-based detection
           if self.config.scanner.enable_ai {
               // TODO: Implement AI detection
               debug!("AI detection not yet implemented for: {}", file_path.display());
           }'''

new_code = '''           // AI-based detection
           if self.config.scanner.enable_ai {
               // Extract file features for AI analysis
               let file_features = self.extract_file_features(file_path, &content)?;
               
               match self.ai_engine.analyze_file(&file_features).await {
                   Ok(Some(analysis)) => {
                       if analysis.is_threat && analysis.confidence > 0.7 {
                           info!("AI detected threat in {}: {} (confidence: {:.2})", 
                                file_path.display(), analysis.threat_type, analysis.confidence);
                           return Ok(Some(ThreatInfo {
                               threat_id: uuid::Uuid::new_v4().to_string(),
                               threat_name: format!("AI-{}", analysis.threat_type),
                               threat_type: analysis.threat_type,
                               severity: analysis.severity,
                               file_path: file_path.to_path_buf(),
                               file_hash: self.calculate_file_hash(file_path)?,
                               detected_at: std::time::SystemTime::now(),
                               confidence: analysis.confidence,
                           }));
                       }
                   }
                   Ok(None) => {
                       debug!("AI analysis complete: no threat in {}", file_path.display());
                   }
                   Err(e) => {
                       warn!("AI analysis failed for {}: {}", file_path.display(), e);
                   }
               }
           }'''

content = content.replace(old_code, new_code)

with open('core/src/scanner.rs', 'w') as f:
    f.write(content)

print("✅ Fixed AI detection - implemented real AI analysis")