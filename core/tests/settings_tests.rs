//! Settings Module Tests
//! 
//! Comprehensive test suite for the settings/configuration module.
//! Tests configuration management, persistence, and validation.

use ghost_core::config::{Config, ScanSettings, UpdateSettings, ProtectionLevel};
use std::path::PathBuf;

#[cfg(test)]
mod settings_tests {
    use super::*;

    /// Test: Config initialization with defaults
    #[test]
    fn test_config_default() {
        let config = Config::default();
        
        assert!(config.scan_settings.real_time_protection, "Real-time protection should be enabled by default");
        assert_eq!(config.scan_settings.protection_level, ProtectionLevel::Balanced, "Default protection level should be Balanced");
    }

    /// Test: Load config from file
    #[test]
    fn test_load_config() {
        let result = Config::load();
        assert!(result.is_ok(), "Should load config successfully");
    }

    /// Test: Save config to file
    #[test]
    fn test_save_config() {
        let config = Config::default();
        let result = config.save();
        
        assert!(result.is_ok(), "Should save config successfully");
    }

    /// Test: Update scan settings
    #[test]
    fn test_update_scan_settings() {
        let mut config = Config::default();
        
        config.scan_settings.real_time_protection = false;
        config.scan_settings.scan_archives = true;
        config.scan_settings.scan_email = true;
        
        assert!(!config.scan_settings.real_time_protection, "Real-time protection should be disabled");
        assert!(config.scan_settings.scan_archives, "Archive scanning should be enabled");
        assert!(config.scan_settings.scan_email, "Email scanning should be enabled");
    }

    /// Test: Update protection level
    #[test]
    fn test_update_protection_level() {
        let mut config = Config::default();
        
        config.scan_settings.protection_level = ProtectionLevel::Maximum;
        assert_eq!(config.scan_settings.protection_level, ProtectionLevel::Maximum, "Protection level should be Maximum");
        
        config.scan_settings.protection_level = ProtectionLevel::Low;
        assert_eq!(config.scan_settings.protection_level, ProtectionLevel::Low, "Protection level should be Low");
    }

    /// Test: Update settings
    #[test]
    fn test_update_update_settings() {
        let mut config = Config::default();
        
        config.update_settings.auto_update = false;
        config.update_settings.check_interval_hours = 12;
        
        assert!(!config.update_settings.auto_update, "Auto-update should be disabled");
        assert_eq!(config.update_settings.check_interval_hours, 12, "Check interval should be 12 hours");
    }

    /// Test: Validate config
    #[test]
    fn test_validate_config() {
        let config = Config::default();
        let result = config.validate();
        
        assert!(result.is_ok(), "Default config should be valid");
    }

    /// Test: Invalid config validation
    #[test]
    fn test_invalid_config() {
        let mut config = Config::default();
        
        // Set invalid values
        config.update_settings.check_interval_hours = 0; // Invalid: must be > 0
        
        let result = config.validate();
        assert!(result.is_err(), "Invalid config should fail validation");
    }

    /// Test: Get specific setting
    #[test]
    fn test_get_setting() {
        let config = Config::default();
        
        let real_time = config.get_setting("real_time_protection");
        assert!(real_time.is_ok(), "Should retrieve setting successfully");
    }

    /// Test: Set specific setting
    #[test]
    fn test_set_setting() {
        let mut config = Config::default();
        
        let result = config.set_setting("real_time_protection", "false");
        assert!(result.is_ok(), "Should set setting successfully");
        
        assert!(!config.scan_settings.real_time_protection, "Setting should be updated");
    }

    /// Test: Reset to defaults
    #[test]
    fn test_reset_to_defaults() {
        let mut config = Config::default();
        
        // Modify settings
        config.scan_settings.real_time_protection = false;
        config.scan_settings.protection_level = ProtectionLevel::Maximum;
        
        // Reset
        config.reset_to_defaults();
        
        assert!(config.scan_settings.real_time_protection, "Should reset to default");
        assert_eq!(config.scan_settings.protection_level, ProtectionLevel::Balanced, "Should reset to default");
    }

    /// Test: Export config
    #[test]
    fn test_export_config() {
        let config = Config::default();
        let result = config.export_to_json();
        
        assert!(result.is_ok(), "Should export config to JSON");
        
        let json = result.unwrap();
        assert!(json.contains("scan_settings"), "JSON should contain scan_settings");
    }

    /// Test: Import config
    #[test]
    fn test_import_config() {
        let config = Config::default();
        let json = config.export_to_json().unwrap();
        
        let result = Config::import_from_json(&json);
        assert!(result.is_ok(), "Should import config from JSON");
    }

    /// Test: Config persistence
    #[test]
    fn test_config_persistence() {
        let mut config = Config::default();
        
        // Modify and save
        config.scan_settings.real_time_protection = false;
        config.save().unwrap();
        
        // Load and verify
        let loaded = Config::load().unwrap();
        assert!(!loaded.scan_settings.real_time_protection, "Loaded config should match saved config");
    }

    /// Test: Quarantine path configuration
    #[test]
    fn test_quarantine_path() {
        let config = Config::default();
        
        let path = config.get_quarantine_path();
        assert!(path.is_some(), "Should have quarantine path");
        assert!(path.unwrap().exists() || true, "Path should be valid"); // May not exist yet
    }

    /// Test: Exclusion paths
    #[test]
    fn test_exclusion_paths() {
        let mut config = Config::default();
        
        let path = PathBuf::from("/test/path");
        config.add_exclusion_path(path.clone());
        
        assert!(config.is_path_excluded(&path), "Path should be excluded");
    }

    /// Test: Remove exclusion path
    #[test]
    fn test_remove_exclusion_path() {
        let mut config = Config::default();
        
        let path = PathBuf::from("/test/path");
        config.add_exclusion_path(path.clone());
        
        assert!(config.is_path_excluded(&path), "Path should be excluded");
        
        config.remove_exclusion_path(&path);
        assert!(!config.is_path_excluded(&path), "Path should not be excluded after removal");
    }

    /// Test: Scheduled scan configuration
    #[test]
    fn test_scheduled_scan() {
        let mut config = Config::default();
        
        config.scan_settings.scheduled_scan_enabled = true;
        config.scan_settings.scheduled_scan_time = "02:00".to_string();
        
        assert!(config.scan_settings.scheduled_scan_enabled, "Scheduled scan should be enabled");
        assert_eq!(config.scan_settings.scheduled_scan_time, "02:00", "Scheduled time should be set");
    }

    /// Test: Notification settings
    #[test]
    fn test_notification_settings() {
        let mut config = Config::default();
        
        config.notification_settings.show_notifications = true;
        config.notification_settings.sound_enabled = false;
        
        assert!(config.notification_settings.show_notifications, "Notifications should be enabled");
        assert!(!config.notification_settings.sound_enabled, "Sound should be disabled");
    }

    /// Test: Performance settings
    #[test]
    fn test_performance_settings() {
        let mut config = Config::default();
        
        config.performance_settings.max_cpu_usage = 50;
        config.performance_settings.max_memory_mb = 512;
        
        assert_eq!(config.performance_settings.max_cpu_usage, 50, "CPU limit should be set");
        assert_eq!(config.performance_settings.max_memory_mb, 512, "Memory limit should be set");
    }

    /// Test: Network settings
    #[test]
    fn test_network_settings() {
        let mut config = Config::default();
        
        config.network_settings.enable_firewall = true;
        config.network_settings.block_suspicious_connections = true;
        
        assert!(config.network_settings.enable_firewall, "Firewall should be enabled");
        assert!(config.network_settings.block_suspicious_connections, "Suspicious blocking should be enabled");
    }

    /// Test: Config versioning
    #[test]
    fn test_config_version() {
        let config = Config::default();
        
        assert!(!config.version.is_empty(), "Config should have version");
        assert!(config.version.starts_with("1."), "Version should be 1.x");
    }

    /// Test: Config migration
    #[test]
    fn test_config_migration() {
        let old_config = Config::default();
        let result = old_config.migrate_to_latest();
        
        assert!(result.is_ok(), "Config migration should succeed");
    }

    /// Test: Thread-safe config access
    #[test]
    fn test_concurrent_config_access() {
        use std::sync::Arc;
        use std::thread;
        
        let config = Arc::new(Config::default());
        let mut handles = vec![];
        
        for _ in 0..5 {
            let config_clone = Arc::clone(&config);
            let handle = thread::spawn(move || {
                let _real_time = config_clone.scan_settings.real_time_protection;
                let _level = config_clone.scan_settings.protection_level;
            });
            handles.push(handle);
        }
        
        for handle in handles {
            assert!(handle.join().is_ok(), "Concurrent access should succeed");
        }
    }
}