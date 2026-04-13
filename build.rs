//! Build script for Universal Android Debloater
//!
//! This script:
//! - Embeds build metadata (commit hash, build timestamp, target)
//! - Validates required resources exist
//! - Sets up platform-specific configuration

use std::env;
use std::path::Path;
use std::process::Command;

fn main() {
    // Rebuild if these files change
    println!("cargo:rerun-if-changed=resources/assets/uad_lists.json");
    println!("cargo:rerun-if-changed=build.rs");

    // Validate required resources exist
    validate_resources();

    // Embed build metadata
    emit_build_metadata();
}

fn validate_resources() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    let required_files = ["resources/assets/uad_lists.json", "resources/assets/icons.ttf"];

    for file in &required_files {
        let path = Path::new(&manifest_dir).join(file);
        if !path.exists() {
            panic!(
                "Required resource file not found: {}\n\
                Please ensure all resources are present before building.",
                file
            );
        }
    }
}

fn emit_build_metadata() {
    // Get git commit hash (if available)
    let commit_hash = get_git_hash().unwrap_or_else(|| "unknown".to_string());
    println!("cargo:rustc-env=GIT_COMMIT_HASH={}", commit_hash);

    // Get build timestamp
    let timestamp = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC").to_string();
    println!("cargo:rustc-env=BUILD_TIMESTAMP={}", timestamp);

    // Get target triple
    let target = env::var("TARGET").unwrap_or_else(|_| "unknown".to_string());
    println!("cargo:rustc-env=BUILD_TARGET={}", target);

    // Get feature flags
    let features = env::var("CARGO_FEATURES").unwrap_or_default();
    if features.contains("self-update") {
        println!("cargo:rustc-cfg=feature=\"self-update\"");
    }
    if features.contains("wgpu") {
        println!("cargo:rustc-cfg=feature=\"wgpu\"");
    }
    if features.contains("glow") {
        println!("cargo:rustc-cfg=feature=\"glow\"");
    }
}

fn get_git_hash() -> Option<String> {
    // Try to get short commit hash from git
    Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .output()
        .ok()
        .filter(|output| output.status.success())
        .and_then(|output| String::from_utf8(output.stdout).ok())
        .map(|hash| hash.trim().to_string())
}
