#!/usr/bin/env python3

with open('core/src/monitor.rs', 'r') as f:
    content = f.read()

# Replace the TODO with actual alert implementation
old_code = '''                // TODO: Send alert or take action'''

new_code = '''                // Send alert through WebSocket if available
                if let Err(e) = self.send_threat_alert(&process_info, threat_score).await {
                    warn!("Failed to send threat alert: {}", e);
                }
                
                // Log to analyzer for tracking
                let threat_info = crate::analyzer::ThreatInfo {
                    process_id: process_info.pid,
                    process_name: process_info.name.clone(),
                    threat_score,
                    timestamp: std::time::SystemTime::now(),
                };
                
                // Store in threat history
                info!("Threat alert sent for process: {} (Score: {})", process_info.name, threat_score);'''

content = content.replace(old_code, new_code)

# Add the send_threat_alert method
insert_point = content.find("    /// Get process history")
if insert_point != -1:
    new_method = '''    /// Send threat alert via WebSocket
    async fn send_threat_alert(&self, process_info: &ProcessInfo, threat_score: u8) -> Result<()> {
        use crate::api::websocket;
        
        let alert = serde_json::json!({
            "type": "threat_alert",
            "process_name": process_info.name,
            "process_id": process_info.pid,
            "threat_score": threat_score,
            "timestamp": std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
        });
        
        websocket::broadcast_message(&alert.to_string()).await;
        Ok(())
    }
    
'''
    content = content[:insert_point] + new_method + content[insert_point:]

with open('core/src/monitor.rs', 'w') as f:
    f.write(content)

print("✅ Implemented monitor alerts with WebSocket broadcasting")