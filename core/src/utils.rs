//! Utility Functions Module
//! 
//! Common utility functions for file operations, formatting, and validation.

use std::path::{Path, PathBuf};
use std::fs::{self, File};
use std::io::Read;
use std::time::{Duration, SystemTime};
use anyhow::{Result, Context};
use sha2::{Sha256, Digest};
use log::debug;

/// File type classification
#[derive(Debug, Clone, PartialEq)]
pub enum FileType {
    Executable,
    Document,
    Archive,
    Image,
    Video,
    Audio,
    Script,
    Unknown,
}

/// Calculate SHA256 hash of a file
pub fn calculate_file_hash(path: &Path) -> Result<String> {
    debug!("Calculating hash for: {:?}", path);
    
    let mut file = File::open(path)
        .context("Failed to open file for hashing")?;
    
    let mut hasher = Sha256::new();
    let mut buffer = [0u8; 8192];
    
    loop {
        let bytes_read = file.read(&mut buffer)
            .context("Failed to read file")?;
        
        if bytes_read == 0 {
            break;
        }
        
        hasher.update(&buffer[..bytes_read]);
    }
    
    Ok(format!("{:x}", hasher.finalize()))
}

/// Get file size in bytes
pub fn get_file_size(path: &Path) -> Result<u64> {
    let metadata = fs::metadata(path)
        .context("Failed to get file metadata")?;
    
    Ok(metadata.len())
}

/// Check if file is executable
pub fn is_executable(path: &Path) -> bool {
    if let Some(extension) = path.extension() {
        let ext = extension.to_string_lossy().to_lowercase();
        matches!(ext.as_str(), 
            "exe" | "dll" | "sys" | "com" | "bat" | "cmd" | 
            "scr" | "pif" | "msi" | "app" | "deb" | "rpm" |
            "sh" | "bin" | "run" | "elf"
        )
    } else {
        false
    }
}

/// Determine file type based on extension
pub fn get_file_type(path: &Path) -> FileType {
    if let Some(extension) = path.extension() {
        let ext = extension.to_string_lossy().to_lowercase();
        
        match ext.as_str() {
            // Executables
            "exe" | "dll" | "sys" | "com" | "bat" | "cmd" | "scr" | "pif" | 
            "msi" | "app" | "deb" | "rpm" | "sh" | "bin" | "run" | "elf" => {
                FileType::Executable
            },
            
            // Documents
            "doc" | "docx" | "pdf" | "txt" | "rtf" | "odt" | "xls" | "xlsx" | 
            "ppt" | "pptx" | "csv" | "xml" | "json" | "md" => {
                FileType::Document
            },
            
            // Archives
            "zip" | "rar" | "7z" | "tar" | "gz" | "bz2" | "xz" | "iso" | 
            "dmg" | "pkg" => {
                FileType::Archive
            },
            
            // Images
            "jpg" | "jpeg" | "png" | "gif" | "bmp" | "svg" | "ico" | "webp" => {
                FileType::Image
            },
            
            // Videos
            "mp4" | "avi" | "mkv" | "mov" | "wmv" | "flv" | "webm" => {
                FileType::Video
            },
            
            // Audio
            "mp3" | "wav" | "flac" | "aac" | "ogg" | "wma" | "m4a" => {
                FileType::Audio
            },
            
            // Scripts
            "js" | "py" | "rb" | "pl" | "php" | "vbs" | "ps1" => {
                FileType::Script
            },
            
            _ => FileType::Unknown,
        }
    } else {
        FileType::Unknown
    }
}

/// Format bytes to human-readable string
pub fn format_bytes(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    
    if bytes == 0 {
        return "0 B".to_string();
    }
    
    let mut size = bytes as f64;
    let mut unit_index = 0;
    
    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }
    
    if unit_index == 0 {
        format!("{} {}", bytes, UNITS[unit_index])
    } else {
        format!("{:.2} {}", size, UNITS[unit_index])
    }
}

/// Format duration to human-readable string
pub fn format_duration(duration: Duration) -> String {
    let total_seconds = duration.as_secs();
    
    if total_seconds < 60 {
        format!("{}s", total_seconds)
    } else if total_seconds < 3600 {
        let minutes = total_seconds / 60;
        let seconds = total_seconds % 60;
        format!("{}m {}s", minutes, seconds)
    } else if total_seconds < 86400 {
        let hours = total_seconds / 3600;
        let minutes = (total_seconds % 3600) / 60;
        format!("{}h {}m", hours, minutes)
    } else {
        let days = total_seconds / 86400;
        let hours = (total_seconds % 86400) / 3600;
        format!("{}d {}h", days, hours)
    }
}

/// Sanitize filename by removing invalid characters
pub fn sanitize_filename(name: &str) -> String {
    name.chars()
        .map(|c| match c {
            '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|' => '_',
            c if c.is_control() => '_',
            c => c,
        })
        .collect()
}

/// Create backup of a file
pub fn create_backup(path: &Path) -> Result<PathBuf> {
    debug!("Creating backup of: {:?}", path);
    
    if !path.exists() {
        anyhow::bail!("File does not exist: {:?}", path);
    }
    
    // Generate backup filename with timestamp
    let timestamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    
    let backup_name = format!(
        "{}.backup.{}",
        path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("file"),
        timestamp
    );
    
    let backup_path = path.parent()
        .unwrap_or_else(|| Path::new("."))
        .join(backup_name);
    
    fs::copy(path, &backup_path)
        .context("Failed to create backup")?;
    
    debug!("Backup created: {:?}", backup_path);
    Ok(backup_path)
}

/// Check if path is safe (not system directory)
pub fn is_safe_path(path: &Path) -> bool {
    let path_str = path.to_string_lossy().to_lowercase();
    
    // Dangerous paths to avoid
    let dangerous_paths = [
        "/system",
        "/windows",
        "/boot",
        "/etc",
        "/usr/bin",
        "/usr/sbin",
        "/sbin",
        "c:\\windows",
        "c:\\program files",
        "c:\\program files (x86)",
    ];
    
    !dangerous_paths.iter().any(|dp| path_str.starts_with(dp))
}

/// Get file extension
pub fn get_extension(path: &Path) -> Option<String> {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|s| s.to_lowercase())
}

/// Check if file is hidden
pub fn is_hidden(path: &Path) -> bool {
    path.file_name()
        .and_then(|name| name.to_str())
        .map(|name| name.starts_with('.'))
        .unwrap_or(false)
}

/// Get file age (time since last modification)
pub fn get_file_age(path: &Path) -> Result<Duration> {
    let metadata = fs::metadata(path)
        .context("Failed to get file metadata")?;
    
    let modified = metadata.modified()
        .context("Failed to get modification time")?;
    
    SystemTime::now()
        .duration_since(modified)
        .context("Invalid modification time")
}

/// Check if file is recently modified (within last N seconds)
pub fn is_recently_modified(path: &Path, seconds: u64) -> Result<bool> {
    let age = get_file_age(path)?;
    Ok(age.as_secs() <= seconds)
}

/// Validate file path
pub fn validate_path(path: &Path) -> Result<()> {
    if !path.exists() {
        anyhow::bail!("Path does not exist: {:?}", path);
    }
    
    if !path.is_file() {
        anyhow::bail!("Path is not a file: {:?}", path);
    }
    
    Ok(())
}

/// Get parent directory
pub fn get_parent_dir(path: &Path) -> Option<PathBuf> {
    path.parent().map(|p| p.to_path_buf())
}

/// Join paths safely
pub fn join_paths(base: &Path, relative: &str) -> PathBuf {
    base.join(sanitize_filename(relative))
}

/// Count files in directory
pub fn count_files_in_dir(dir: &Path) -> Result<usize> {
    if !dir.is_dir() {
        anyhow::bail!("Not a directory: {:?}", dir);
    }
    
    let count = fs::read_dir(dir)
        .context("Failed to read directory")?
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.path().is_file())
        .count();
    
    Ok(count)
}

/// Get total size of directory
pub fn get_dir_size(dir: &Path) -> Result<u64> {
    if !dir.is_dir() {
        anyhow::bail!("Not a directory: {:?}", dir);
    }
    
    let mut total_size = 0u64;
    
    for entry in fs::read_dir(dir).context("Failed to read directory")? {
        let entry = entry.context("Failed to read entry")?;
        let path = entry.path();
        
        if path.is_file() {
            total_size += get_file_size(&path)?;
        } else if path.is_dir() {
            total_size += get_dir_size(&path)?;
        }
    }
    
    Ok(total_size)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::TempDir;
    
    fn create_test_file(dir: &Path, name: &str, content: &[u8]) -> PathBuf {
        let path = dir.join(name);
        let mut file = File::create(&path).unwrap();
        file.write_all(content).unwrap();
        path
    }
    
    #[test]
    fn test_calculate_file_hash() {
        let temp_dir = TempDir::new().unwrap();
        let test_file = create_test_file(temp_dir.path(), "test.txt", b"Hello, World!");
        
        let hash = calculate_file_hash(&test_file).unwrap();
        assert_eq!(hash.len(), 64); // SHA256 produces 64 hex characters
    }
    
    #[test]
    fn test_get_file_size() {
        let temp_dir = TempDir::new().unwrap();
        let content = b"Hello, World!";
        let test_file = create_test_file(temp_dir.path(), "test.txt", content);
        
        let size = get_file_size(&test_file).unwrap();
        assert_eq!(size, content.len() as u64);
    }
    
    #[test]
    fn test_is_executable() {
        assert!(is_executable(Path::new("test.exe")));
        assert!(is_executable(Path::new("test.dll")));
        assert!(!is_executable(Path::new("test.txt")));
        assert!(!is_executable(Path::new("test.pdf")));
    }
    
    #[test]
    fn test_get_file_type() {
        assert_eq!(get_file_type(Path::new("test.exe")), FileType::Executable);
        assert_eq!(get_file_type(Path::new("test.pdf")), FileType::Document);
        assert_eq!(get_file_type(Path::new("test.zip")), FileType::Archive);
        assert_eq!(get_file_type(Path::new("test.jpg")), FileType::Image);
        assert_eq!(get_file_type(Path::new("test.mp4")), FileType::Video);
        assert_eq!(get_file_type(Path::new("test.mp3")), FileType::Audio);
        assert_eq!(get_file_type(Path::new("test.js")), FileType::Script);
    }
    
    #[test]
    fn test_format_bytes() {
        assert_eq!(format_bytes(0), "0 B");
        assert_eq!(format_bytes(512), "512 B");
        assert_eq!(format_bytes(1024), "1.00 KB");
        assert_eq!(format_bytes(1024 * 1024), "1.00 MB");
        assert_eq!(format_bytes(1024 * 1024 * 1024), "1.00 GB");
    }
    
    #[test]
    fn test_format_duration() {
        assert_eq!(format_duration(Duration::from_secs(30)), "30s");
        assert_eq!(format_duration(Duration::from_secs(90)), "1m 30s");
        assert_eq!(format_duration(Duration::from_secs(3661)), "1h 1m");
        assert_eq!(format_duration(Duration::from_secs(86400)), "1d 0h");
    }
    
    #[test]
    fn test_sanitize_filename() {
        assert_eq!(sanitize_filename("test/file.txt"), "test_file.txt");
        assert_eq!(sanitize_filename("test:file.txt"), "test_file.txt");
        assert_eq!(sanitize_filename("test*file.txt"), "test_file.txt");
        assert_eq!(sanitize_filename("normal.txt"), "normal.txt");
    }
    
    #[test]
    fn test_create_backup() {
        let temp_dir = TempDir::new().unwrap();
        let test_file = create_test_file(temp_dir.path(), "test.txt", b"Test content");
        
        let backup_path = create_backup(&test_file).unwrap();
        assert!(backup_path.exists());
        assert!(backup_path.to_string_lossy().contains("backup"));
    }
    
    #[test]
    fn test_is_safe_path() {
        assert!(is_safe_path(Path::new("/home/user/documents")));
        assert!(is_safe_path(Path::new("/tmp/test")));
        assert!(!is_safe_path(Path::new("/system/bin")));
        assert!(!is_safe_path(Path::new("/windows/system32")));
    }
    
    #[test]
    fn test_get_extension() {
        assert_eq!(get_extension(Path::new("test.txt")), Some("txt".to_string()));
        assert_eq!(get_extension(Path::new("test.TAR.GZ")), Some("gz".to_string()));
        assert_eq!(get_extension(Path::new("test")), None);
    }
    
    #[test]
    fn test_is_hidden() {
        assert!(is_hidden(Path::new(".hidden")));
        assert!(!is_hidden(Path::new("visible.txt")));
    }
}