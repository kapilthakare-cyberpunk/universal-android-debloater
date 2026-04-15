use crate::core::ai::types::*;
use crate::core::sync::{get_devices_list, Phone};
use std::collections::HashMap;

/// Device analysis engine for AI-powered debloating

pub struct DeviceAnalyzer;

impl DeviceAnalyzer {
    pub fn new() -> Self {
        Self
    }

    /// Analyze a connected device and generate AI-ready data
    pub async fn analyze_device(&self, device_id: &str) -> Result<DeviceAnalysisData, AnalysisError> {
        // Get device information
        let devices = get_devices_list().await;
        let device = devices
            .into_iter()
            .find(|d| d.adb_id == device_id)
            .ok_or_else(|| AnalysisError::DeviceNotFound(device_id.to_string()))?;

        if device.state != "device" {
            return Err(AnalysisError::DeviceNotAuthorized(device.adb_id.clone()));
        }

        // Build device profile
        let device_profile = self.build_device_profile(&device)?;

        // Get package information
        let packages = self.get_package_list(&device)?;

        // Analyze usage patterns (if available)
        let usage_data = self.analyze_usage_patterns(&device).ok();

        Ok(DeviceAnalysisData {
            device_profile,
            packages,
            usage_data,
        })
    }

    /// Build comprehensive device profile
    fn build_device_profile(&self, device: &Phone) -> Result<DeviceProfile, AnalysisError> {
        // This would normally query the device for detailed specs
        // For now, use available information
        Ok(DeviceProfile {
            model: device.model.clone(),
            android_version: "Unknown".to_string(), // Would need to query device
            android_sdk: device.android_sdk,
            total_packages: 0, // Will be populated from package list
            system_packages: 0,
            user_packages: 0,
            storage_used_gb: 0.0, // Would need to query device
            storage_total_gb: 0.0,
        })
    }

    /// Get comprehensive package list from device
    fn get_package_list(&self, device: &Phone) -> Result<Vec<PackageInfo>, AnalysisError> {
        // This would use ADB to get package information
        // For now, return mock data - in real implementation this would:
        // 1. Run 'pm list packages -f' to get all packages
        // 2. Run 'dumpsys package <pkg>' for detailed info
        // 3. Parse output to build PackageInfo structs

        // Mock implementation for development
        Ok(vec![
            PackageInfo {
                package_name: "com.android.chrome".to_string(),
                app_name: "Chrome".to_string(),
                version: "1.0.0".to_string(),
                size_bytes: 100_000_000,
                install_time: Some(chrono::Utc::now() - chrono::Duration::days(30)),
                last_used: Some(chrono::Utc::now() - chrono::Duration::hours(2)),
                is_system_app: false,
                category: PackageCategory::Other,
            },
            PackageInfo {
                package_name: "com.facebook.katana".to_string(),
                app_name: "Facebook".to_string(),
                version: "1.0.0".to_string(),
                size_bytes: 200_000_000,
                install_time: Some(chrono::Utc::now() - chrono::Duration::days(60)),
                last_used: Some(chrono::Utc::now() - chrono::Duration::days(7)),
                is_system_app: false,
                category: PackageCategory::Social,
            },
        ])
    }

    /// Analyze app usage patterns
    fn analyze_usage_patterns(&self, device: &Phone) -> Result<UsageData, AnalysisError> {
        // Would analyze usage stats, battery usage, etc.
        // For now, return basic structure
        Ok(UsageData {
            total_apps_launched: 0,
            most_used_apps: vec![],
            battery_drainers: vec![],
            storage_hogs: vec![],
        })
    }

    /// Categorize packages based on heuristics
    pub fn categorize_package(package_name: &str, app_name: &str) -> PackageCategory {
        let name_lower = package_name.to_lowercase();
        let app_lower = app_name.to_lowercase();

        // System apps
        if name_lower.starts_with("com.android.")
            || name_lower.starts_with("android.")
            || name_lower.contains("system")
            || name_lower.contains("framework")
        {
            return PackageCategory::SystemApp;
        }

        // Bloatware patterns
        if name_lower.contains("facebook")
            || name_lower.contains("google") && (name_lower.contains("ads") || name_lower.contains("analytics"))
            || name_lower.contains("samsung") && name_lower.contains("edge")
            || name_lower.contains("huawei") && name_lower.contains("hiapp")
        {
            return PackageCategory::Bloatware;
        }

        // Gaming apps
        if name_lower.contains("game") || name_lower.contains("gaming") {
            return PackageCategory::Gaming;
        }

        // Social media
        if name_lower.contains("facebook")
            || name_lower.contains("instagram")
            || name_lower.contains("twitter")
            || name_lower.contains("tiktok")
            || name_lower.contains("snapchat")
        {
            return PackageCategory::Social;
        }

        // Advertising/tracking
        if name_lower.contains("ad") || name_lower.contains("analytics") || name_lower.contains("tracking") {
            return PackageCategory::Advertising;
        }

        PackageCategory::Other
    }
}

#[derive(Debug)]
pub struct DeviceAnalysisData {
    pub device_profile: DeviceProfile,
    pub packages: Vec<PackageInfo>,
    pub usage_data: Option<UsageData>,
}

#[derive(Debug)]
pub struct UsageData {
    pub total_apps_launched: u32,
    pub most_used_apps: Vec<String>,
    pub battery_drainers: Vec<String>,
    pub storage_hogs: Vec<String>,
}

#[derive(Debug, thiserror::Error)]
pub enum AnalysisError {
    #[error("Device not found: {0}")]
    DeviceNotFound(String),
    #[error("Device not authorized: {0}")]
    DeviceNotAuthorized(String),
    #[error("ADB command failed: {0}")]
    AdbError(String),
    #[error("Failed to parse package data: {0}")]
    ParseError(String),
}
