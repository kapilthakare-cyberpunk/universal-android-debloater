use serde::{Deserialize, Serialize};

/// Core data structures for AI-powered debloating

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiAnalysis {
    pub device_profile: DeviceProfile,
    pub suggested_actions: Vec<SuggestedAction>,
    pub confidence_score: f32,
    pub risk_assessment: RiskLevel,
    pub analysis_timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceProfile {
    pub model: String,
    pub android_version: String,
    pub android_sdk: u8,
    pub total_packages: usize,
    pub system_packages: usize,
    pub user_packages: usize,
    pub storage_used_gb: f64,
    pub storage_total_gb: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuggestedAction {
    pub package: String,
    pub action_type: ActionType,
    pub reasoning: String,
    pub risk_level: RiskLevel,
    pub confidence_score: f32,
    pub dependencies: Vec<String>,
    pub estimated_savings_mb: Option<u64>,
    pub category: PackageCategory,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ActionType {
    Uninstall,
    Disable,
    Archive,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RiskLevel {
    Safe,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PackageCategory {
    SystemApp,
    Bloatware,
    UnusedApp,
    LargeApp,
    Suspicious,
    Gaming,
    Social,
    Advertising,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageInfo {
    pub package_name: String,
    pub app_name: String,
    pub version: String,
    pub size_bytes: u64,
    pub install_time: Option<chrono::DateTime<chrono::Utc>>,
    pub last_used: Option<chrono::DateTime<chrono::Utc>>,
    pub is_system_app: bool,
    pub category: PackageCategory,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    pub is_valid: bool,
    pub warnings: Vec<String>,
    pub blocking_issues: Vec<String>,
    pub adjusted_risk_level: Option<RiskLevel>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiOperation {
    pub id: String,
    pub action: SuggestedAction,
    pub status: OperationStatus,
    pub started_at: Option<chrono::DateTime<chrono::Utc>>,
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum OperationStatus {
    Pending,
    InProgress,
    Completed,
    Failed,
    RolledBack,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiConfig {
    pub groq_api_key: Option<String>,
    pub groq_model: String,
    pub risk_tolerance: RiskLevel,
    pub max_operations_per_session: usize,
    pub enable_ai_suggestions: bool,
    pub enable_automated_execution: bool,
    pub package_blacklist: Vec<String>,
}
