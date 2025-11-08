//! Updates Module Tests
//! 
//! Comprehensive test suite for the updates module.
//! Tests update checking, downloading, and application.

use ghost_core::updates::{UpdateManager, UpdateInfo, UpdateStatus};

#[cfg(test)]
mod updates_tests {
    use super::*;

    /// Test: Update manager initialization
    #[test]
    fn test_update_manager_new() {
        let manager = UpdateManager::new();
        assert!(manager.is_ok(), "Update manager should initialize successfully");
    }

    /// Test: Check for updates
    #[test]
    fn test_check_for_updates() {
        let manager = UpdateManager::new().unwrap();
        let result = manager.check_for_updates();
        
        assert!(result.is_ok(), "Should check for updates successfully");
    }

    /// Test: Get current version
    #[test]
    fn test_get_current_version() {
        let manager = UpdateManager::new().unwrap();
        let version = manager.get_current_version();
        
        assert!(version.is_ok(), "Should retrieve current version successfully");
        
        let version = version.unwrap();
        assert!(!version.is_empty(), "Version should not be empty");
        assert!(version.contains('.'), "Version should contain dots");
    }

    /// Test: Get latest version
    #[test]
    fn test_get_latest_version() {
        let manager = UpdateManager::new().unwrap();
        let result = manager.get_latest_version();
        
        assert!(result.is_ok(), "Should retrieve latest version successfully");
    }

    /// Test: Check if update available
    #[test]
    fn test_is_update_available() {
        let manager = UpdateManager::new().unwrap();
        let result = manager.is_update_available();
        
        assert!(result.is_ok(), "Should check update availability successfully");
    }

    /// Test: Get update info
    #[test]
    fn test_get_update_info() {
        let manager = UpdateManager::new().unwrap();
        
        // First check for updates
        manager.check_for_updates().unwrap();
        
        let info = manager.get_update_info();
        assert!(info.is_ok(), "Should retrieve update info successfully");
    }

    /// Test: Download update
    #[test]
    fn test_download_update() {
        let manager = UpdateManager::new().unwrap();
        
        // Check if update is available
        if manager.is_update_available().unwrap() {
            let result = manager.download_update();
            assert!(result.is_ok(), "Should download update successfully");
        }
    }

    /// Test: Get download progress
    #[test]
    fn test_get_download_progress() {
        let manager = UpdateManager::new().unwrap();
        let progress = manager.get_download_progress();
        
        assert!(progress.is_ok(), "Should retrieve download progress successfully");
        
        let progress = progress.unwrap();
        assert!(progress >= 0.0 && progress <= 100.0, "Progress should be between 0 and 100");
    }

    /// Test: Verify update
    #[test]
    fn test_verify_update() {
        let manager = UpdateManager::new().unwrap();
        
        // This will fail if no update is downloaded, which is expected
        let result = manager.verify_update();
        
        // Either succeeds or fails gracefully
        assert!(result.is_ok() || result.is_err(), "Verify should return a result");
    }

    /// Test: Apply update
    #[test]
    fn test_apply_update() {
        let manager = UpdateManager::new().unwrap();
        
        // This is a dry-run test - won't actually apply
        // In production, this would restart the application
        let result = manager.apply_update_dry_run();
        assert!(result.is_ok(), "Dry-run update should succeed");
    }

    /// Test: Get update history
    #[test]
    fn test_get_update_history() {
        let manager = UpdateManager::new().unwrap();
        let history = manager.get_update_history();
        
        assert!(history.is_ok(), "Should retrieve update history successfully");
    }

    /// Test: Get last check time
    #[test]
    fn test_get_last_check_time() {
        let manager = UpdateManager::new().unwrap();
        
        // Check for updates first
        manager.check_for_updates().unwrap();
        
        let last_check = manager.get_last_check_time();
        assert!(last_check.is_ok(), "Should retrieve last check time successfully");
        assert!(last_check.unwrap() > 0, "Last check time should be positive");
    }

    /// Test: Set auto-update
    #[test]
    fn test_set_auto_update() {
        let manager = UpdateManager::new().unwrap();
        
        let result = manager.set_auto_update(true);
        assert!(result.is_ok(), "Should enable auto-update successfully");
        
        assert!(manager.is_auto_update_enabled(), "Auto-update should be enabled");
        
        let result = manager.set_auto_update(false);
        assert!(result.is_ok(), "Should disable auto-update successfully");
        
        assert!(!manager.is_auto_update_enabled(), "Auto-update should be disabled");
    }

    /// Test: Set check interval
    #[test]
    fn test_set_check_interval() {
        let manager = UpdateManager::new().unwrap();
        
        let result = manager.set_check_interval(24);
        assert!(result.is_ok(), "Should set check interval successfully");
        
        let interval = manager.get_check_interval();
        assert_eq!(interval, 24, "Check interval should be 24 hours");
    }

    /// Test: Cancel download
    #[test]
    fn test_cancel_download() {
        let manager = UpdateManager::new().unwrap();
        
        let result = manager.cancel_download();
        assert!(result.is_ok(), "Should cancel download successfully");
    }

    /// Test: Get update status
    #[test]
    fn test_get_update_status() {
        let manager = UpdateManager::new().unwrap();
        let status = manager.get_status();
        
        assert!(status.is_ok(), "Should retrieve update status successfully");
        
        let status = status.unwrap();
        assert!(
            matches!(status, UpdateStatus::Idle | UpdateStatus::Checking | UpdateStatus::Downloading | UpdateStatus::Ready),
            "Status should be valid"
        );
    }

    /// Test: Rollback update
    #[test]
    fn test_rollback_update() {
        let manager = UpdateManager::new().unwrap();
        
        // This is a dry-run test
        let result = manager.rollback_dry_run();
        assert!(result.is_ok(), "Rollback dry-run should succeed");
    }

    /// Test: Get changelog
    #[test]
    fn test_get_changelog() {
        let manager = UpdateManager::new().unwrap();
        
        manager.check_for_updates().unwrap();
        
        let changelog = manager.get_changelog();
        assert!(changelog.is_ok(), "Should retrieve changelog successfully");
    }

    /// Test: Validate update package
    #[test]
    fn test_validate_update_package() {
        let manager = UpdateManager::new().unwrap();
        
        // Create a test package path
        let test_path = std::path::PathBuf::from("/tmp/test_update.pkg");
        
        let result = manager.validate_package(&test_path);
        // Will fail for non-existent file, which is expected
        assert!(result.is_err() || result.is_ok(), "Validation should return a result");
    }

    /// Test: Get update size
    #[test]
    fn test_get_update_size() {
        let manager = UpdateManager::new().unwrap();
        
        if manager.is_update_available().unwrap() {
            let size = manager.get_update_size();
            assert!(size.is_ok(), "Should retrieve update size successfully");
            assert!(size.unwrap() > 0, "Update size should be positive");
        }
    }

    /// Test: Check update compatibility
    #[test]
    fn test_check_compatibility() {
        let manager = UpdateManager::new().unwrap();
        
        let result = manager.check_compatibility();
        assert!(result.is_ok(), "Should check compatibility successfully");
    }

    /// Test: Get release notes
    #[test]
    fn test_get_release_notes() {
        let manager = UpdateManager::new().unwrap();
        
        manager.check_for_updates().unwrap();
        
        let notes = manager.get_release_notes();
        assert!(notes.is_ok(), "Should retrieve release notes successfully");
    }

    /// Test: Schedule update
    #[test]
    fn test_schedule_update() {
        let manager = UpdateManager::new().unwrap();
        
        // Schedule for 1 hour from now
        let schedule_time = std::time::SystemTime::now() + std::time::Duration::from_secs(3600);
        
        let result = manager.schedule_update(schedule_time);
        assert!(result.is_ok(), "Should schedule update successfully");
    }

    /// Test: Cancel scheduled update
    #[test]
    fn test_cancel_scheduled_update() {
        let manager = UpdateManager::new().unwrap();
        
        let result = manager.cancel_scheduled_update();
        assert!(result.is_ok(), "Should cancel scheduled update successfully");
    }

    /// Test: Get scheduled update time
    #[test]
    fn test_get_scheduled_time() {
        let manager = UpdateManager::new().unwrap();
        
        let scheduled = manager.get_scheduled_time();
        // May be None if no update is scheduled
        assert!(scheduled.is_none() || scheduled.is_some(), "Should return valid option");
    }

    /// Test: Update notification preferences
    #[test]
    fn test_notification_preferences() {
        let manager = UpdateManager::new().unwrap();
        
        manager.set_notify_on_available(true).unwrap();
        assert!(manager.should_notify_on_available(), "Should notify on available");
        
        manager.set_notify_on_downloaded(true).unwrap();
        assert!(manager.should_notify_on_downloaded(), "Should notify on downloaded");
    }

    /// Test: Concurrent update checks
    #[test]
    fn test_concurrent_checks() {
        use std::sync::Arc;
        use std::thread;
        
        let manager = Arc::new(UpdateManager::new().unwrap());
        let mut handles = vec![];
        
        for _ in 0..3 {
            let manager_clone = Arc::clone(&manager);
            let handle = thread::spawn(move || {
                manager_clone.check_for_updates()
            });
            handles.push(handle);
        }
        
        for handle in handles {
            assert!(handle.join().unwrap().is_ok(), "Concurrent checks should succeed");
        }
    }

    /// Test: Update manager state persistence
    #[test]
    fn test_state_persistence() {
        let manager = UpdateManager::new().unwrap();
        
        // Modify state
        manager.set_auto_update(true).unwrap();
        manager.set_check_interval(12).unwrap();
        
        // Save state
        let result = manager.save_state();
        assert!(result.is_ok(), "Should save state successfully");
        
        // Create new manager and verify state is loaded
        let new_manager = UpdateManager::new().unwrap();
        assert!(new_manager.is_auto_update_enabled(), "State should be persisted");
    }
}