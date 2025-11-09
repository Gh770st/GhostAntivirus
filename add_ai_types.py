#!/usr/bin/env python3

with open('core/src/ai.rs', 'r') as f:
    content = f.read()

# Find where to insert the types (after the imports, before the impl block)
insert_point = content.find("impl AIIntegration")

if insert_point != -1:
    # Insert the type definitions
    new_types = '''/// Features extracted from a file for AI analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileFeatures {
    pub file_hash: String,
    pub file_size: u64,
    pub file_extension: String,
    pub pe_features: Option<PEFeatures>,
    pub string_patterns: Vec<String>,
    pub entropy: f64,
}

/// PE (Portable Executable) file features
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PEFeatures {
    pub is_pe: bool,
    pub has_pe_header: bool,
    pub is_dll: bool,
    pub is_exe: bool,
    pub has_imports: bool,
    pub has_exports: bool,
}

/// AI analysis result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisResult {
    pub is_threat: bool,
    pub threat_type: String,
    pub severity: String,
    pub confidence: f64,
    pub reason: String,
}

'''
    
    # Insert the types
    content = content[:insert_point] + new_types + content[insert_point:]
    
    with open('core/src/ai.rs', 'w') as f:
        f.write(content)
    
    print("✅ Added FileFeatures, PEFeatures, and AnalysisResult types")
else:
    print("❌ Could not find AIIntegration impl block")