#!/usr/bin/env python3
import re

with open('core/src/updater.rs', 'r') as f:
    content = f.read()

# Find and replace the problematic section
old_section = '''           let mut downloaded: u64 = 0;
           let mut buffer = [0u8; 8192];
           
           loop {
               let bytes_read = response.copy_to(&amp;mut buffer[..])
                   .context("Failed to read update data")? as usize;
               
               if bytes_read == 0 {
                   break;
               }
               
               file.write_all(&amp;buffer[..bytes_read])
                   .context("Failed to write update data")?;
               
               downloaded += bytes_read as u64;
               
               // Log progress
               if downloaded % (1024 * 1024) == 0 {
                   debug!("Downloaded {} MB / {} MB", 
                         downloaded / (1024 * 1024),
                         update.file_size / (1024 * 1024));
               }
           }'''

new_section = '''           use std::io::Write;
           
           // Read entire response body
           let content = response.bytes()
               .context("Failed to read update data")?;
           
           file.write_all(&amp;content)
               .context("Failed to write update data")?;
           
           let downloaded = content.len() as u64;
           
           // Log completion
           debug!("Downloaded {} MB", 
                 downloaded / (1024 * 1024));'''

content = content.replace(old_section, new_section)

with open('core/src/updater.rs', 'w') as f:
    f.write(content)

print("Fixed updater.rs")