use crate::core::ai::types::*;
use std::collections::HashSet;

/// Safety validation and guardrails for AI operations

pub struct SafetyValidator;

impl SafetyValidator {
    pub fn new() -> Self {
        Self
    }

    /// Comprehensive pre-execution safety validation
    pub fn validate_operation_batch(
        &self,
        suggestions: &[SuggestedAction],
        device_profile: &DeviceProfile,
    ) -> SafetyValidationResult {
        let mut warnings = Vec::new();
        let mut blocking_issues = Vec::new();
        let mut adjusted_suggestions = Vec::new();

        // System stability checks
        self.check_system_stability(suggestions, &mut warnings, &mut blocking_issues);

        // Dependency analysis
        self.check_dependencies(suggestions, &mut warnings, &mut blocking_issues);

        // Resource impact assessment
        self.check_resource_impact(suggestions, device_profile, &mut warnings);

        // Boot safety checks
        self.check_boot_safety(suggestions, &mut warnings, &mut blocking_issues);

        // Adjust suggestions based on validation
        for suggestion in suggestions {
            let mut adjusted = suggestion.clone();
            self.adjust_suggestion_safety(&mut adjusted, &warnings, &blocking_issues);
            adjusted_suggestions.push(adjusted);
        }

        let is_safe = blocking_issues.is_empty();
        let safety_score = self.calculate_safety_score(&adjusted_suggestions, &warnings);

        SafetyValidationResult {
            is_safe,
            warnings,
            blocking_issues,
            adjusted_suggestions,
            safety_score,
        }
    }

    /// Check for system stability risks
    fn check_system_stability(
        &self,
        suggestions: &[SuggestedAction],
        warnings: &mut Vec<String>,
        blocking_issues: &mut Vec<String>,
    ) {
        // Count high-risk operations
        let high_risk_count = suggestions
            .iter()
            .filter(|s| matches!(s.risk_level, RiskLevel::High | RiskLevel::Critical))
            .count();

        if high_risk_count > 3 {
            warnings.push(format!("High number of high-risk operations ({})", high_risk_count));
        }

        // Check for conflicting operations
        let uninstall_packages: HashSet<_> = suggestions
            .iter()
            .filter(|s| s.action_type == ActionType::Uninstall)
            .map(|s| &s.package)
            .collect();

        let disable_packages: HashSet<_> = suggestions
            .iter()
            .filter(|s| s.action_type == ActionType::Disable)
            .map(|s| &s.package)
            .collect();

        let conflicts: Vec<_> = uninstall_packages.intersection(&disable_packages).collect();
        if !conflicts.is_empty() {
            blocking_issues.push(format!(
                "Conflicting operations for packages: {:?}",
                conflicts.into_iter().collect::<Vec<_>>()
            ));
        }

        // Check for core system components
        for suggestion in suggestions {
            if self.is_core_system_component(&suggestion.package) {
                blocking_issues.push(format!("Cannot modify core system component: {}", suggestion.package));
            }
        }
    }

    /// Analyze package dependencies
    fn check_dependencies(
        &self,
        suggestions: &[SuggestedAction],
        warnings: &mut Vec<String>,
        blocking_issues: &mut Vec<String>,
    ) {
        // This is a simplified dependency check
        // In a real implementation, this would query the system package manager

        let suggested_packages: HashSet<_> = suggestions.iter().map(|s| &s.package).collect();

        // Check for known dependency chains
        for suggestion in suggestions {
            if suggestion.package.contains("google") && suggestion.package.contains("play") {
                // Play Store being removed - check for dependent apps
                let dependent_count = suggestions
                    .iter()
                    .filter(|s| s.package.contains("google") || s.package.contains("play"))
                    .count();

                if dependent_count > 1 {
                    warnings.push(format!(
                        "Removing Play Store may affect {} other Google apps",
                        dependent_count - 1
                    ));
                }
            }
        }

        // Check for launcher removal
        if suggested_packages.iter().any(|pkg| pkg.contains("launcher")) {
            warnings.push("Removing launcher may affect home screen functionality".to_string());
        }
    }

    /// Assess resource impact
    fn check_resource_impact(
        &self,
        suggestions: &[SuggestedAction],
        device_profile: &DeviceProfile,
        warnings: &mut Vec<String>,
    ) {
        let total_savings: u64 = suggestions.iter().filter_map(|s| s.estimated_savings_mb).sum();

        let total_storage_gb = device_profile.storage_total_gb;
        let savings_percentage = if total_storage_gb > 0.0 {
            (total_savings as f64 / 1_000_000.0) / total_storage_gb * 100.0
        } else {
            0.0
        };

        if savings_percentage > 50.0 {
            warnings.push(format!(
                "Operations would free {:.1}% of storage - consider doing this in batches",
                savings_percentage
            ));
        }

        // Check for battery-intensive operations
        let battery_intensive_count = suggestions
            .iter()
            .filter(|s| self.is_battery_intensive_operation(s))
            .count();

        if battery_intensive_count > 5 {
            warnings.push(format!(
                "{} operations may be battery-intensive",
                battery_intensive_count
            ));
        }
    }

    /// Check for boot safety
    fn check_boot_safety(
        &self,
        suggestions: &[SuggestedAction],
        warnings: &mut Vec<String>,
        blocking_issues: &mut Vec<String>,
    ) {
        // Essential boot components
        let essential_packages = [
            "android",
            "com.android.systemui",
            "com.android.settings",
            "com.android.server",
        ];

        for suggestion in suggestions {
            if essential_packages.contains(&suggestion.package.as_str()) {
                blocking_issues.push(format!(
                    "Cannot modify essential boot component: {}",
                    suggestion.package
                ));
            }
        }

        // Check for too many system modifications
        let system_modifications = suggestions
            .iter()
            .filter(|s| s.category == PackageCategory::SystemApp)
            .count();

        if system_modifications > 2 {
            warnings.push(format!(
                "Multiple system app modifications ({}) may affect boot stability",
                system_modifications
            ));
        }
    }

    /// Adjust suggestion safety based on validation results
    fn adjust_suggestion_safety(
        &self,
        suggestion: &mut SuggestedAction,
        warnings: &[String],
        blocking_issues: &[String],
    ) {
        // If there are warnings about this package, increase risk level
        let package_warnings = warnings.iter().filter(|w| w.contains(&suggestion.package)).count();

        if package_warnings > 0 {
            match suggestion.risk_level {
                RiskLevel::Safe => suggestion.risk_level = RiskLevel::Medium,
                RiskLevel::Medium => suggestion.risk_level = RiskLevel::High,
                _ => {} // Don't increase beyond High
            }
        }

        // If this package has blocking issues, mark as critical
        let has_blocking_issues = blocking_issues.iter().any(|issue| issue.contains(&suggestion.package));

        if has_blocking_issues {
            suggestion.risk_level = RiskLevel::Critical;
        }
    }

    /// Calculate overall safety score (0.0 = unsafe, 1.0 = completely safe)
    fn calculate_safety_score(&self, suggestions: &[SuggestedAction], warnings: &[String]) -> f32 {
        let base_score = 1.0;

        // Deduct for high-risk operations
        let high_risk_penalty = suggestions
            .iter()
            .filter(|s| matches!(s.risk_level, RiskLevel::High | RiskLevel::Critical))
            .count() as f32
            * 0.1;

        // Deduct for warnings
        let warning_penalty = warnings.len() as f32 * 0.05;

        // Deduct for system app modifications
        let system_penalty = suggestions
            .iter()
            .filter(|s| s.category == PackageCategory::SystemApp)
            .count() as f32
            * 0.15;

        (base_score - high_risk_penalty - warning_penalty - system_penalty).max(0.0)
    }

    /// Check if package is a core system component
    fn is_core_system_component(&self, package_name: &str) -> bool {
        let core_packages = [
            "android",
            "com.android.systemui",
            "com.android.settings",
            "com.android.providers.settings",
            "com.android.server.telecom",
            "com.android.phone",
            "com.android.providers.contacts",
            "com.android.providers.media",
        ];

        core_packages.contains(&package_name)
    }

    /// Check if operation is battery-intensive
    fn is_battery_intensive_operation(&self, suggestion: &SuggestedAction) -> bool {
        // Large uninstalls can be battery intensive
        if let Some(size_mb) = suggestion.estimated_savings_mb {
            size_mb > 100 // 100MB threshold
        } else {
            false
        }
    }

    /// Generate safety recommendations
    pub fn generate_safety_recommendations(&self, validation_result: &SafetyValidationResult) -> Vec<String> {
        let mut recommendations = Vec::new();

        if !validation_result.is_safe {
            recommendations.push("⚠️ Blocking issues detected - review and resolve before proceeding".to_string());
        }

        if !validation_result.warnings.is_empty() {
            recommendations.push(format!(
                "⚡ {} warnings identified - consider addressing them",
                validation_result.warnings.len()
            ));
        }

        if validation_result.safety_score < 0.7 {
            recommendations.push("🔴 Low safety score - consider reducing scope or reviewing suggestions".to_string());
        }

        // Specific recommendations based on validation
        if validation_result
            .adjusted_suggestions
            .iter()
            .any(|s| s.category == PackageCategory::SystemApp)
        {
            recommendations.push("💡 System app modifications detected - create backup before proceeding".to_string());
        }

        recommendations
    }
}

#[derive(Debug)]
pub struct SafetyValidationResult {
    pub is_safe: bool,
    pub warnings: Vec<String>,
    pub blocking_issues: Vec<String>,
    pub adjusted_suggestions: Vec<SuggestedAction>,
    pub safety_score: f32,
}
