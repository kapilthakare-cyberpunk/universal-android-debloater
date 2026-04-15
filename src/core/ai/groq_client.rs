use crate::core::ai::types::*;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct GroqClient {
    client: Client,
    api_key: String,
    model: String,
    base_url: String,
}

#[derive(Debug, Serialize)]
struct GroqRequest {
    model: String,
    messages: Vec<GroqMessage>,
    temperature: f32,
    max_tokens: u32,
}

#[derive(Debug, Serialize, Deserialize)]
struct GroqMessage {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct GroqResponse {
    choices: Vec<GroqChoice>,
    usage: Option<GroqUsage>,
}

#[derive(Debug, Deserialize)]
struct GroqChoice {
    message: GroqMessage,
    finish_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
struct GroqUsage {
    prompt_tokens: u32,
    completion_tokens: u32,
    total_tokens: u32,
}

#[derive(Debug, thiserror::Error)]
pub enum GroqError {
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),
    #[error("API error: {0}")]
    Api(String),
    #[error("JSON parsing error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("Configuration error: {0}")]
    Config(String),
}

impl GroqClient {
    pub fn new(api_key: String) -> Result<Self, GroqError> {
        if api_key.trim().is_empty() {
            return Err(GroqError::Config("API key cannot be empty".to_string()));
        }

        let client = Client::builder().timeout(Duration::from_secs(30)).build()?;

        Ok(Self {
            client,
            api_key,
            model: "mixtral-8x7b-32768".to_string(), // Good balance of speed and intelligence
            base_url: "https://api.groq.com".to_string(),
        })
    }

    pub fn with_model(mut self, model: String) -> Self {
        self.model = model;
        self
    }

    pub async fn analyze_device(
        &self,
        device_profile: &DeviceProfile,
        packages: &[PackageInfo],
    ) -> Result<AiAnalysis, GroqError> {
        let prompt = self.build_analysis_prompt(device_profile, packages);

        let response = self.make_request(prompt).await?;
        let suggestions = self.parse_analysis_response(&response, packages)?;

        Ok(AiAnalysis {
            device_profile: device_profile.clone(),
            suggested_actions: suggestions,
            confidence_score: 0.85, // Base confidence, could be refined
            risk_assessment: RiskLevel::Medium,
            analysis_timestamp: chrono::Utc::now(),
        })
    }

    pub async fn validate_suggestions(&self, suggestions: &[SuggestedAction]) -> Result<ValidationResult, GroqError> {
        let prompt = self.build_validation_prompt(suggestions);
        let response = self.make_request(prompt).await?;
        self.parse_validation_response(&response)
    }

    async fn make_request(&self, prompt: String) -> Result<String, GroqError> {
        let request = GroqRequest {
            model: self.model.clone(),
            messages: vec![GroqMessage {
                role: "user".to_string(),
                content: prompt,
            }],
            temperature: 0.3, // Lower temperature for more consistent analysis
            max_tokens: 2048,
        };

        let response = self
            .client
            .post(format!("{}/openai/v1/chat/completions", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            return Err(GroqError::Api(format!(
                "API request failed with status {}: {}",
                status, error_text
            )));
        }

        let groq_response: GroqResponse = response.json().await?;
        let content = groq_response
            .choices
            .first()
            .and_then(|choice| Some(choice.message.content.clone()))
            .unwrap_or_default();

        Ok(content)
    }

    fn build_analysis_prompt(&self, device: &DeviceProfile, packages: &[PackageInfo]) -> String {
        let package_summary = packages
            .iter()
            .take(50) // Limit to prevent token overflow
            .map(|p| {
                format!(
                    "- {} ({}): {}MB, system: {}, category: {:?}",
                    p.package_name,
                    p.app_name,
                    p.size_bytes / 1_000_000,
                    p.is_system_app,
                    p.category
                )
            })
            .collect::<Vec<_>>()
            .join("\n");

        format!(
            r#"You are an expert Android debloating assistant. Analyze this device and suggest optimization actions.

Device Profile:
- Model: {}
- Android Version: {} (SDK {})
- Total Packages: {}
- System Apps: {}
- User Apps: {}
- Storage: {:.1}GB used of {:.1}GB total

Installed Packages (sample):
{}

Provide suggestions for debloating in this JSON format:
{{
  "suggestions": [
    {{
      "package": "com.example.app",
      "action": "uninstall|disable|archive",
      "reasoning": "Brief explanation",
      "risk_level": "safe|medium|high|critical",
      "confidence": 0.0-1.0,
      "category": "bloatware|unused|large|suspicious|other"
    }}
  ],
  "overall_assessment": "Brief summary"
}}

Focus on:
- Bloatware and pre-installed apps
- Unused apps (not used in 6+ months)
- Large apps with little value
- Suspicious or data-collecting apps
- System apps that can be safely disabled

Prioritize safety - never suggest removing critical system components!"#,
            device.model,
            device.android_version,
            device.android_sdk,
            device.total_packages,
            device.system_packages,
            device.user_packages,
            device.storage_used_gb,
            device.storage_total_gb,
            package_summary
        )
    }

    fn build_validation_prompt(&self, suggestions: &[SuggestedAction]) -> String {
        let suggestions_text = suggestions
            .iter()
            .map(|s| {
                format!(
                    "- {}: {} (risk: {:?}, confidence: {:.2})",
                    s.package, s.reasoning, s.risk_level, s.confidence_score
                )
            })
            .collect::<Vec<_>>()
            .join("\n");

        format!(
            r#"Review these debloating suggestions for safety and validity:

{}

For each suggestion, check:
1. Is this package safe to remove/disable?
2. Are there any dependencies that could break the system?
3. Is the reasoning sound?
4. Should the risk level be adjusted?

Provide validation results in JSON format:
{{
  "validations": [
    {{
      "package": "com.example.app",
      "is_safe": true|false,
      "warnings": ["warning1", "warning2"],
      "blocking_issues": ["issue1"],
      "adjusted_risk": "safe|medium|high|critical|null"
    }}
  ]
}}"#,
            suggestions_text
        )
    }

    fn parse_analysis_response(
        &self,
        response: &str,
        packages: &[PackageInfo],
    ) -> Result<Vec<SuggestedAction>, GroqError> {
        // Extract JSON from response (AI might add extra text)
        let json_start = response.find('{').unwrap_or(0);
        let json_end = response.rfind('}').unwrap_or(response.len()) + 1;
        let json_str = &response[json_start..json_end];

        #[derive(Deserialize)]
        struct AnalysisResponse {
            suggestions: Vec<SuggestionData>,
        }

        #[derive(Deserialize)]
        struct SuggestionData {
            package: String,
            action: String,
            reasoning: String,
            risk_level: String,
            confidence: f32,
            category: String,
        }

        let parsed: AnalysisResponse = serde_json::from_str(json_str)?;
        let package_map: std::collections::HashMap<_, _> =
            packages.iter().map(|p| (p.package_name.clone(), p.clone())).collect();

        let suggestions = parsed
            .suggestions
            .into_iter()
            .filter_map(|s| {
                let package_info = package_map.get(&s.package)?;
                let action_type = match s.action.to_lowercase().as_str() {
                    "uninstall" => ActionType::Uninstall,
                    "disable" => ActionType::Disable,
                    "archive" => ActionType::Archive,
                    _ => return None,
                };
                let risk_level = match s.risk_level.to_lowercase().as_str() {
                    "safe" => RiskLevel::Safe,
                    "medium" => RiskLevel::Medium,
                    "high" => RiskLevel::High,
                    "critical" => RiskLevel::Critical,
                    _ => RiskLevel::High,
                };
                let category = match s.category.to_lowercase().as_str() {
                    "bloatware" => PackageCategory::Bloatware,
                    "unused" => PackageCategory::UnusedApp,
                    "large" => PackageCategory::LargeApp,
                    "suspicious" => PackageCategory::Suspicious,
                    _ => PackageCategory::Other,
                };

                Some(SuggestedAction {
                    package: s.package,
                    action_type,
                    reasoning: s.reasoning,
                    risk_level,
                    confidence_score: s.confidence.min(1.0).max(0.0),
                    dependencies: vec![], // Could be populated by additional analysis
                    estimated_savings_mb: Some((package_info.size_bytes / 1_000_000) as u64),
                    category,
                })
            })
            .collect();

        Ok(suggestions)
    }

    fn parse_validation_response(&self, response: &str) -> Result<ValidationResult, GroqError> {
        let json_start = response.find('{').unwrap_or(0);
        let json_end = response.rfind('}').unwrap_or(response.len()) + 1;
        let json_str = &response[json_start..json_end];

        #[derive(Deserialize)]
        struct ValidationResponse {
            validations: Vec<ValidationData>,
        }

        #[derive(Deserialize)]
        struct ValidationData {
            package: String,
            is_safe: bool,
            warnings: Vec<String>,
            blocking_issues: Vec<String>,
            adjusted_risk: Option<String>,
        }

        let parsed: ValidationResponse = serde_json::from_str(json_str)?;

        let (warnings, blocking_issues, adjusted_risk) = parsed.validations.into_iter().fold(
            (Vec::new(), Vec::new(), None),
            |(mut warnings_acc, mut blocking_acc, adjusted), validation| {
                warnings_acc.extend(validation.warnings);
                blocking_acc.extend(validation.blocking_issues);

                let new_adjusted = validation.adjusted_risk.and_then(|r| match r.to_lowercase().as_str() {
                    "safe" => Some(RiskLevel::Safe),
                    "medium" => Some(RiskLevel::Medium),
                    "high" => Some(RiskLevel::High),
                    "critical" => Some(RiskLevel::Critical),
                    _ => None,
                });

                (warnings_acc, blocking_acc, adjusted.or(new_adjusted))
            },
        );

        Ok(ValidationResult {
            is_valid: blocking_issues.is_empty(),
            warnings,
            blocking_issues,
            adjusted_risk_level: adjusted_risk,
        })
    }
}
