use crate::core::ai::types::*;
use std::collections::{HashMap, HashSet};

/// AI suggestion processing and refinement engine

pub struct SuggestionProcessor;

impl SuggestionProcessor {
    pub fn new() -> Self {
        Self
    }

    /// Process and refine AI suggestions with safety checks
    pub fn process_suggestions(
        &self,
        raw_suggestions: Vec<SuggestedAction>,
        config: &AiConfig,
        packages: &[PackageInfo],
    ) -> Vec<SuggestedAction> {
        let mut processed = Vec::new();

        for suggestion in raw_suggestions {
            // Apply safety filters
            if !self.passes_safety_checks(&suggestion, config, packages) {
                continue;
            }

            // Adjust risk level based on configuration
            let adjusted_suggestion = self.adjust_for_risk_tolerance(suggestion, config);

            // Enrich with additional data
            let enriched_suggestion = self.enrich_suggestion(adjusted_suggestion, packages);

            processed.push(enriched_suggestion);
        }

        // Sort by priority (safety first, then confidence)
        processed.sort_by(|a, b| {
            // Sort by risk level first (safe first)
            let risk_order = self
                .risk_level_priority(&a.risk_level)
                .cmp(&self.risk_level_priority(&b.risk_level));

            if risk_order == std::cmp::Ordering::Equal {
                // Then by confidence (highest first)
                b.confidence_score
                    .partial_cmp(&a.confidence_score)
                    .unwrap_or(std::cmp::Ordering::Equal)
            } else {
                risk_order
            }
        });

        // Limit number of suggestions
        processed.truncate(config.max_operations_per_session);

        processed
    }

    /// Apply comprehensive safety checks
    fn passes_safety_checks(&self, suggestion: &SuggestedAction, config: &AiConfig, packages: &[PackageInfo]) -> bool {
        // Check blacklist
        if config.package_blacklist.contains(&suggestion.package) {
            return false;
        }

        // Check risk tolerance
        if self.risk_level_priority(&suggestion.risk_level) < self.risk_level_priority(&config.risk_tolerance) {
            return false;
        }

        // Critical system apps check
        if self.is_critical_system_app(&suggestion.package) {
            return false;
        }

        // Dependency check - ensure we don't break dependencies
        if self.has_critical_dependencies(&suggestion.package, packages) {
            return false;
        }

        // Size sanity check - don't suggest unreasonably large operations
        if let Some(savings) = suggestion.estimated_savings_mb {
            if savings > 1000 {
                // 1GB limit per suggestion
                return false;
            }
        }

        true
    }

    /// Adjust suggestion based on user risk tolerance
    fn adjust_for_risk_tolerance(&self, mut suggestion: SuggestedAction, config: &AiConfig) -> SuggestedAction {
        // If user has conservative settings, upgrade some actions to be safer
        if config.risk_tolerance == RiskLevel::Safe {
            match suggestion.action_type {
                ActionType::Uninstall => {
                    suggestion.action_type = ActionType::Disable;
                    suggestion.risk_level = RiskLevel::Safe;
                }
                ActionType::Disable => {
                    suggestion.risk_level = RiskLevel::Safe;
                }
                _ => {}
            }
        }

        suggestion
    }

    /// Enrich suggestion with additional package information
    fn enrich_suggestion(&self, mut suggestion: SuggestedAction, packages: &[PackageInfo]) -> SuggestedAction {
        if let Some(package_info) = packages.iter().find(|p| p.package_name == suggestion.package) {
            // Update size estimate if not already set
            if suggestion.estimated_savings_mb.is_none() {
                suggestion.estimated_savings_mb = Some((package_info.size_bytes / 1_000_000) as u64);
            }

            // Enhance reasoning with package details
            let enhanced_reasoning = format!(
                "{} (Size: {}MB, Category: {:?}, Last used: {})",
                suggestion.reasoning,
                package_info.size_bytes / 1_000_000,
                package_info.category,
                package_info
                    .last_used
                    .as_ref()
                    .map(|dt| format!("{} ago", Self::time_ago(dt)))
                    .unwrap_or_else(|| "Never".to_string())
            );
            suggestion.reasoning = enhanced_reasoning;
        }

        suggestion
    }

    /// Check if package is a critical system component
    fn is_critical_system_app(&self, package_name: &str) -> bool {
        let critical_packages = [
            "com.android.systemui",
            "com.android.settings",
            "com.android.packageinstaller",
            "com.android.launcher",
            "android",
            "com.android.providers.settings",
            "com.android.providers.media",
            "com.android.server.telecom",
            "com.android.phone",
            "com.android.dialer",
        ];

        critical_packages.contains(&package_name)
    }

    /// Check if package has critical dependencies
    fn has_critical_dependencies(&self, package_name: &str, packages: &[PackageInfo]) -> bool {
        // Simple dependency checking - in a real implementation this would
        // query the system for actual dependencies
        let package_lower = package_name.to_lowercase();

        // Some known dependency patterns
        if package_lower.contains("google") && package_lower.contains("play") {
            // Play Store dependencies
            return packages.iter().any(|p| {
                p.package_name.contains("google") && (p.package_name.contains("gms") || p.package_name.contains("play"))
            });
        }

        false
    }

    /// Get priority value for risk levels (lower number = higher priority)
    fn risk_level_priority(&self, risk: &RiskLevel) -> i32 {
        match risk {
            RiskLevel::Safe => 1,
            RiskLevel::Medium => 2,
            RiskLevel::High => 3,
            RiskLevel::Critical => 4,
        }
    }

    /// Create batch groups for safer execution
    pub fn create_execution_batches(&self, suggestions: &[SuggestedAction]) -> Vec<Vec<SuggestedAction>> {
        let mut safe_batch = Vec::new();
        let mut medium_batch = Vec::new();
        let mut high_batch = Vec::new();

        for suggestion in suggestions {
            match suggestion.risk_level {
                RiskLevel::Safe => safe_batch.push(suggestion.clone()),
                RiskLevel::Medium => medium_batch.push(suggestion.clone()),
                RiskLevel::High | RiskLevel::Critical => high_batch.push(suggestion.clone()),
            }
        }

        vec![safe_batch, medium_batch, high_batch]
            .into_iter()
            .filter(|batch| !batch.is_empty())
            .collect()
    }

    /// Generate summary statistics for suggestions
    pub fn generate_summary(&self, suggestions: &[SuggestedAction]) -> SuggestionSummary {
        let total_suggestions = suggestions.len();
        let by_risk: HashSet<_> = suggestions.iter().map(|s| &s.risk_level).collect();
        let risk_levels: Vec<_> = by_risk.into_iter().cloned().collect();

        let total_estimated_savings: u64 = suggestions.iter().filter_map(|s| s.estimated_savings_mb).sum();

        let by_action: HashMap<_, usize> = suggestions.iter().fold(HashMap::new(), |mut acc, s| {
            *acc.entry(s.action_type.clone()).or_insert(0) += 1;
            acc
        });

        SuggestionSummary {
            total_suggestions,
            risk_levels,
            total_estimated_savings_mb: total_estimated_savings,
            action_breakdown: by_action,
        }
    }

    /// Helper function to format time ago
    fn time_ago(datetime: &chrono::DateTime<chrono::Utc>) -> String {
        let now = chrono::Utc::now();
        let duration = now.signed_duration_since(*datetime);

        if duration.num_days() > 365 {
            format!("{} years", duration.num_days() / 365)
        } else if duration.num_days() > 30 {
            format!("{} months", duration.num_days() / 30)
        } else if duration.num_days() > 0 {
            format!("{} days", duration.num_days())
        } else if duration.num_hours() > 0 {
            format!("{} hours", duration.num_hours())
        } else {
            format!("{} minutes", duration.num_minutes().max(1))
        }
    }
}

#[derive(Debug)]
pub struct SuggestionSummary {
    pub total_suggestions: usize,
    pub risk_levels: Vec<RiskLevel>,
    pub total_estimated_savings_mb: u64,
    pub action_breakdown: HashMap<ActionType, usize>,
}
