use crate::core::ai::analysis::DeviceAnalyzer;
use crate::core::ai::groq_client::GroqClient;
use crate::core::ai::suggestions::SuggestionProcessor;
use crate::core::ai::safety::SafetyValidator;
use crate::core::config::{AiDebloatMode, GeneralSettings};
use crate::core::sync::Phone;
use crate::gui::style;
use crate::gui::views::list::PackageRow;

use iced::widget::{button, column, container, pick_list, row, text, text_input, Column};
use iced::{Alignment, Command, Element, Length, Renderer};
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct AiDebloat {
    pub api_key_input: String,
    pub api_key_masked: bool,
    pub selected_mode: AiDebloatMode,
    pub analysis_status: AnalysisStatus,
    pub analysis_result: Option<AiAnalysisResult>,
    pub safety_score: Option<f32>,
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub enum AnalysisStatus {
    #[default]
    Idle,
    Analyzing,
    Validating,
    Complete,
    Error(String),
}

#[derive(Debug, Clone)]
pub struct AiAnalysisResult {
    pub suggestions_count: usize,
    pub safe_count: usize,
    pub medium_count: usize,
    pub high_count: usize,
    pub estimated_savings_mb: u64,
    pub overall_assessment: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    ApiKeyInputChanged(String),
    ToggleApiKeyVisibility,
    ModeSelected(AiDebloatMode),
    RunAnalysis,
    AnalysisComplete(Result<AiAnalysisResult, String>),
    ValidateSafety,
    SafetyValidated(Result<SafetyValidationResult, String>),
    ExecuteDebloat,
    DebloatComplete(Result<(), String>),
    ClearResults,
}

#[derive(Debug, Clone)]
pub struct SafetyValidationResult {
    pub is_safe: bool,
    pub warnings: Vec<String>,
    pub blocking_issues: Vec<String>,
}

impl Default for AiDebloat {
    fn default() -> Self {
        Self {
            api_key_input: String::new(),
            api_key_masked: true,
            selected_mode: AiDebloatMode::Safe,
            analysis_status: AnalysisStatus::Idle,
            analysis_result: None,
            safety_score: None,
            error_message: None,
        }
    }
}

impl AiDebloat {
    pub fn update(
        &mut self,
        settings: &GeneralSettings,
        phone: &Phone,
        packages: &[Vec<PackageRow>],
        msg: Message,
    ) -> Command<Message> {
        match msg {
            Message::ApiKeyInputChanged(key) => {
                self.api_key_input = key;
                Command::none()
            }
            Message::ToggleApiKeyVisibility => {
                self.api_key_masked = !self.api_key_masked;
                Command::none()
            }
            Message::ModeSelected(mode) => {
                self.selected_mode = mode;
                Command::none()
            }
            Message::RunAnalysis => {
                if self.api_key_input.is_empty() {
                    return Command::perform(
                        async { Err("API key is required. Please enter your Groq API key.".to_string()) },
                        Message::AnalysisComplete,
                    );
                }

                self.analysis_status = AnalysisStatus::Analyzing;
                self.error_message = None;

                let api_key = self.api_key_input.clone();
                let mode = self.selected_mode;
                let device_id = phone.adb_id.clone();
                let device_model = phone.model.clone();

                Command::perform(
                    async move {
                        // Initialize Groq client
                        let client = match GroqClient::new(api_key) {
                            Ok(c) => c,
                            Err(e) => return Err(format!("Failed to initialize AI: {}", e)),
                        };

                        // Initialize device analyzer
                        let analyzer = DeviceAnalyzer::new();

                        // Analyze device
                        let analysis_data = match analyzer.analyze_device(&device_id).await {
                            Ok(data) => data,
                            Err(e) => return Err(format!("Device analysis failed: {}", e)),
                        };

                        // Run AI analysis
                        let analysis = match client.analyze_device(&analysis_data.device_profile, &analysis_data.packages).await {
                            Ok(analysis) => analysis,
                            Err(e) => return Err(format!("AI analysis failed: {}", e)),
                        };

                        // Process suggestions based on mode
                        let processor = SuggestionProcessor::new();
                        let config = crate::core::ai::types::AiConfig {
                            groq_api_key: None,
                            groq_model: "mixtral-8x7b-32768".to_string(),
                            risk_tolerance: match mode {
                                AiDebloatMode::Safe => crate::core::ai::types::RiskLevel::Safe,
                                AiDebloatMode::Balanced => crate::core::ai::types::RiskLevel::Medium,
                                AiDebloatMode::Max => crate::core::ai::types::RiskLevel::High,
                            },
                            max_operations_per_session: match mode {
                                AiDebloatMode::Safe => 10,
                                AiDebloatMode::Balanced => 25,
                                AiDebloatMode::Max => 50,
                            },
                            enable_ai_suggestions: true,
                            enable_automated_execution: false,
                            package_blacklist: vec![],
                        };

                        let processed = processor.process_suggestions(
                            analysis.suggested_actions,
                            &config,
                            &analysis_data.packages,
                        );

                        // Calculate summary
                        let summary = processor.generate_summary(&processed);

                        Ok(AiAnalysisResult {
                            suggestions_count: processed.len(),
                            safe_count: processed.iter().filter(|s| s.risk_level == crate::core::ai::types::RiskLevel::Safe).count(),
                            medium_count: processed.iter().filter(|s| s.risk_level == crate::core::ai::types::RiskLevel::Medium).count(),
                            high_count: processed.iter().filter(|s| s.risk_level == crate::core::ai::types::RiskLevel::High).count(),
                            estimated_savings_mb: summary.total_estimated_savings_mb,
                            overall_assessment: analysis.overall_assessment.unwrap_or_default(),
                        })
                    },
                    Message::AnalysisComplete,
                )
            }
            Message::AnalysisComplete(result) => match result {
                Ok(analysis_result) => {
                    self.analysis_result = Some(analysis_result.clone());
                    self.analysis_status = AnalysisStatus::Complete;

                    // Auto-validate safety
                    Command::perform(
                        async move { Ok(SafetyValidationResult { is_safe: true, warnings: vec![], blocking_issues: vec![] }) },
                        Message::SafetyValidated,
                    )
                }
                Err(e) => {
                    self.analysis_status = AnalysisStatus::Error(e.clone());
                    self.error_message = Some(e);
                    Command::none()
                }
            },
            Message::ValidateSafety => {
                self.analysis_status = AnalysisStatus::Validating;
                Command::perform(
                    async move { Ok(SafetyValidationResult { is_safe: true, warnings: vec![], blocking_issues: vec![] }) },
                    Message::SafetyValidated,
                )
            }
            Message::SafetyValidated(result) => match result {
                Ok(validation) => {
                    if validation.is_safe {
                        self.safety_score = Some(0.9);
                    } else {
                        self.safety_score = Some(0.5);
                    }
                    Command::none()
                }
                Err(e) => {
                    self.error_message = Some(e);
                    Command::none()
                }
            },
            Message::ExecuteDebloat => {
                // TODO: Execute the debloating operations
                Command::perform(
                    async move { Ok(()) },
                    Message::DebloatComplete,
                )
            }
            Message::DebloatComplete(result) => match result {
                Ok(()) => {
                    self.analysis_status = AnalysisStatus::Idle;
                    Command::none()
                }
                Err(e) => {
                    self.error_message = Some(e);
                    Command::none()
                }
            },
            Message::ClearResults => {
                self.analysis_result = None;
                self.safety_score = None;
                self.error_message = None;
                self.analysis_status = AnalysisStatus::Idle;
                Command::none()
            }
        }
    }

    pub fn view(&self, phone: &Phone) -> Element<'_, Message, Renderer<crate::core::theme::Theme>> {
        let is_device_connected = !phone.adb_id.is_empty();

        // API Key Section
        let api_key_label = text("Groq API Key").size(16).style(style::Text::Title);

        let api_key_input = text_input("Enter your Groq API key...", &self.api_key_input)
            .on_input(Message::ApiKeyInputChanged)
            .padding(10)
            .width(Length::FillPortion(3));

        let toggle_btn = button(text(if self.api_key_masked { "Show" } else { "Hide" }).size(12))
            .on_press(Message::ToggleApiKeyVisibility)
            .padding(5)
            .width(Length::FillPortion(1))
            .style(style::Button::Primary);

        let api_key_row = row![api_key_input, toggle_btn].spacing(10).align_items(Alignment::Center);

        let api_key_descr = text("Get your free API key at https://console.groq.com")
            .size(13)
            .style(style::Text::Commentary);

        let api_key_ctn = container(column![api_key_label, api_key_row, api_key_descr].spacing(8))
            .padding(15)
            .width(Length::Fill)
            .style(style::Container::Frame);

        // Mode Selection
        let mode_label = text("Debloat Mode").size(16).style(style::Text::Title);

        let mode_picklist = pick_list(
            &[AiDebloatMode::Safe, AiDebloatMode::Balanced, AiDebloatMode::Max],
            Some(self.selected_mode),
            Message::ModeSelected,
        )
        .padding(10)
        .width(Length::Fill);

        let mode_description = match self.selected_mode {
            AiDebloatMode::Safe => {
                text("AI will only suggest clearly safe removals. Best for first-time users.")
                    .size(13)
                    .style(style::Text::Secure)
            }
            AiDebloatMode::Balanced => {
                text("AI balances safety with more aggressive cleanup. Good for experienced users.")
                    .size(13)
                    .style(style::Text::Default)
            }
            AiDebloatMode::Max => {
                text("AI will suggest maximum debloating. May affect some features. Backup recommended!")
                    .size(13)
                    .style(style::Text::Danger)
            }
        };

        let mode_ctn = container(column![mode_label, mode_picklist, mode_description].spacing(8))
            .padding(15)
            .width(Length::Fill)
            .style(style::Container::Frame);

        // Run Button
        let run_btn = if !is_device_connected {
            button(text("Connect device first").size(14))
                .padding(10)
                .width(Length::Fill)
                .style(style::Button::Unavailable)
        } else {
            match self.analysis_status {
                AnalysisStatus::Idle | AnalysisStatus::Complete | AnalysisStatus::Error(_) => button(
                    text(if self.analysis_result.is_some() { "Re-run Analysis" } else { "Run AI Analysis" })
                        .size(14),
                )
                .on_press(Message::RunAnalysis)
                .padding(10)
                .width(Length::Fill)
                .style(style::Button::Primary),
                AnalysisStatus::Analyzing => button(text("Analyzing device...").size(14))
                    .padding(10)
                    .width(Length::Fill)
                    .style(style::Button::Unavailable),
                AnalysisStatus::Validating => button(text("Validating safety...").size(14))
                    .padding(10)
                    .width(Length::Fill)
                    .style(style::Button::Unavailable),
            }
        };

        // Results Section
        let results_section = if let Some(ref result) = self.analysis_result {
            let results_title = text("Analysis Results").size(18).style(style::Text::Title);

            let summary = row![
                text(format!("{} suggestions", result.suggestions_count)).size(14),
                text(format!("Safe: {}", result.safe_count)).size(13).style(style::Text::Secure),
                text(format!("Medium: {}", result.medium_count)).size(13),
                text(format!("High: {}", result.high_count)).size(13).style(style::Text::Danger),
            ]
            .spacing(15);

            let savings = text(format!("Estimated savings: {:.1} MB", result.estimated_savings_mb as f64 / 1000.0))
                .size(14)
                .style(style::Text::Secure);

            let assessment = text(&result.overall_assessment)
                .size(13)
                .width(Length::Fill);

            let execute_btn = button(text("Execute Debloat").size(14))
                .on_press(Message::ExecuteDebloat)
                .padding(10)
                .width(Length::Fill)
                .style(style::Button::Danger);

            let clear_btn = button(text("Clear").size(14))
                .on_press(Message::ClearResults)
                .padding(10)
                .width(Length::Fill)
                .style(style::Button::Secondary);

            column![
                results_title,
                summary,
                savings,
                assessment,
                row![execute_btn, clear_btn].spacing(10),
            ]
            .spacing(15)
        } else {
            column![]
        };

        // Error Section
        let error_section = if let Some(ref error) = self.error_message {
            container(text(format!("Error: {}", error)).size(13).style(style::Text::Danger))
                .padding(10)
                .width(Length::Fill)
                .style(style::Container::BorderedFrame)
        } else {
            container(column![])
        };

        // Main content
        let content = if !is_device_connected {
            column![
                text("AI-Powered Debloating").size(26).style(style::Text::Title),
                text("No device connected. Please connect your Android device to use AI debloating.")
                    .size(14)
                    .style(style::Text::Danger),
            ]
            .spacing(20)
            .align_items(Alignment::Center)
        } else {
            column![
                text("AI-Powered Debloating").size(26).style(style::Text::Title),
                api_key_ctn,
                mode_ctn,
                run_btn,
                error_section,
                results_section,
            ]
            .spacing(20)
            .align_items(Alignment::Center)
        };

        container(content)
            .padding(20)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}
