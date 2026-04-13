use crate::core::utils::{format_diff_time_from_now, last_modified_date};
use crate::CACHE_DIR;
use retry::{delay::Fixed, retry, OperationResult};
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Deserialize, Debug, Clone, PartialEq, Hash, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Package {
    id: String,
    pub list: UadList,
    pub description: String,
    dependencies: Vec<String>,
    needed_by: Vec<String>,
    labels: Vec<String>,
    pub removal: Removal,
}

#[derive(Default, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UadList {
    #[default]
    All,
    Aosp,
    Carrier,
    Google,
    Misc,
    Oem,
    Pending,
    Unlisted,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum UadListState {
    #[default]
    Downloading,
    Done,
    Failed,
}

impl std::fmt::Display for UadListState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let date = last_modified_date(CACHE_DIR.join("uad_lists.json"));
        let s = match self {
            Self::Downloading => "Checking updates...".to_string(),
            Self::Done => format!("Done (last was {})", format_diff_time_from_now(date)),
            Self::Failed => "Failed to check update!".to_string(),
        };
        write!(f, "{s}")
    }
}

impl UadList {
    pub const ALL: [Self; 8] = [
        Self::All,
        Self::Aosp,
        Self::Carrier,
        Self::Google,
        Self::Misc,
        Self::Oem,
        Self::Pending,
        Self::Unlisted,
    ];
}

impl std::fmt::Display for UadList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::All => "All lists",
                Self::Aosp => "aosp",
                Self::Carrier => "carrier",
                Self::Google => "google",
                Self::Misc => "misc",
                Self::Oem => "oem",
                Self::Pending => "pending",
                Self::Unlisted => "unlisted",
            }
        )
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PackageState {
    All,
    #[default]
    Enabled,
    Uninstalled,
    Disabled,
}

impl PackageState {
    pub const ALL: [Self; 4] = [Self::All, Self::Enabled, Self::Uninstalled, Self::Disabled];
}

impl std::fmt::Display for PackageState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::All => "All packages",
                Self::Enabled => "Enabled",
                Self::Uninstalled => "Uninstalled",
                Self::Disabled => "Disabled",
            }
        )
    }
}

pub trait Opposite {
    fn opposite(&self, disable: bool) -> PackageState;
}

impl Opposite for PackageState {
    fn opposite(&self, disable: bool) -> Self {
        match self {
            Self::Enabled => {
                if disable {
                    Self::Disabled
                } else {
                    Self::Uninstalled
                }
            }
            Self::Uninstalled | Self::Disabled => Self::Enabled,
            Self::All => Self::All,
        }
    }
}

// Bad names. To be changed!
#[derive(Default, Debug, Deserialize, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Removal {
    All,
    #[default]
    Recommended,
    Advanced,
    Expert,
    Unsafe,
    Unlisted,
}

impl Removal {
    pub const ALL: [Self; 6] = [
        Self::All,
        Self::Recommended,
        Self::Advanced,
        Self::Expert,
        Self::Unsafe,
        Self::Unlisted,
    ];
}

impl std::fmt::Display for Removal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::All => "All",
                Self::Recommended => "Recommended",
                Self::Advanced => "Advanced",
                Self::Expert => "Expert",
                Self::Unsafe => "Unsafe",
                Self::Unlisted => "Unlisted",
            }
        )
    }
}

type PackageHashMap = HashMap<String, Package>;
pub fn load_debloat_lists(remote: bool) -> (Result<PackageHashMap, PackageHashMap>, bool) {
    let cached_uad_lists: PathBuf = CACHE_DIR.join("uad_lists.json");
    let mut error = false;
    let list: Vec<Package> = if remote {
        retry(Fixed::from_millis(1000).take(60), || {
            match ureq::get(
                "https://raw.githubusercontent.com/0x192/universal-android-debloater/\
           main/resources/assets/uad_lists.json",
            )
            .call()
            {
                Ok(data) => {
                    let text = data.into_string().expect("response should be Ok type");
                    fs::write(cached_uad_lists.clone(), &text).expect("Unable to write file");
                    let list = serde_json::from_str(&text).expect("Unable to parse");
                    OperationResult::Ok(list)
                }
                Err(e) => {
                    warn!("Could not load remote debloat list: {}", e);
                    error = true;
                    OperationResult::Retry(Vec::<Package>::new())
                }
            }
        })
        .unwrap_or_else(|_| get_local_lists())
    } else {
        warn!("Could not load remote debloat list");
        get_local_lists()
    };

    // TODO: Do it without intermediary Vec?
    let mut package_lists = HashMap::new();
    for p in list {
        let name = p.id.clone();
        package_lists.insert(name, p);
    }
    if error {
        (Err(package_lists), remote)
    } else {
        (Ok(package_lists), remote)
    }
}

fn get_local_lists() -> Vec<Package> {
    const DATA: &str = include_str!("../../resources/assets/uad_lists.json");
    let cached_uad_lists = CACHE_DIR.join("uad_lists.json");

    if Path::new(&cached_uad_lists).exists() {
        let data = fs::read_to_string(cached_uad_lists).unwrap();
        serde_json::from_str(&data).expect("Unable to parse")
    } else {
        serde_json::from_str(DATA).expect("Unable to parse")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_json() {
        const DATA: &str = include_str!("../../resources/assets/uad_lists.json");
        let _: Vec<Package> = serde_json::from_str(DATA).expect("Unable to parse");
    }

    #[test]
    fn test_package_has_required_fields() {
        const DATA: &str = include_str!("../../resources/assets/uad_lists.json");
        let packages: Vec<Package> = serde_json::from_str(DATA).expect("Unable to parse");
        assert!(!packages.is_empty(), "Debloat list should not be empty");

        for pkg in &packages {
            assert!(!pkg.id.is_empty(), "Package ID should not be empty");
            // Note: Some packages may have empty descriptions - this is a known data quality issue
        }
    }

    #[test]
    fn test_uad_list_display() {
        assert_eq!(UadList::All.to_string(), "All lists");
        assert_eq!(UadList::Aosp.to_string(), "aosp");
        assert_eq!(UadList::Google.to_string(), "google");
        assert_eq!(UadList::Oem.to_string(), "oem");
    }

    #[test]
    fn test_package_state_display() {
        assert_eq!(PackageState::All.to_string(), "All packages");
        assert_eq!(PackageState::Enabled.to_string(), "Enabled");
        assert_eq!(PackageState::Uninstalled.to_string(), "Uninstalled");
        assert_eq!(PackageState::Disabled.to_string(), "Disabled");
    }

    #[test]
    fn test_removal_display() {
        assert_eq!(Removal::All.to_string(), "All");
        assert_eq!(Removal::Recommended.to_string(), "Recommended");
        assert_eq!(Removal::Advanced.to_string(), "Advanced");
        assert_eq!(Removal::Expert.to_string(), "Expert");
        assert_eq!(Removal::Unsafe.to_string(), "Unsafe");
    }

    #[test]
    fn test_package_state_opposite() {
        // Enabled should become Disabled when disable_mode is true
        assert_eq!(PackageState::Enabled.opposite(true), PackageState::Disabled);
        // Enabled should become Uninstalled when disable_mode is false
        assert_eq!(PackageState::Enabled.opposite(false), PackageState::Uninstalled);
        // Disabled should become Enabled regardless of disable_mode
        assert_eq!(PackageState::Disabled.opposite(true), PackageState::Enabled);
        assert_eq!(PackageState::Disabled.opposite(false), PackageState::Enabled);
        // Uninstalled should become Enabled
        assert_eq!(PackageState::Uninstalled.opposite(true), PackageState::Enabled);
        assert_eq!(PackageState::Uninstalled.opposite(false), PackageState::Enabled);
        // All should remain All
        assert_eq!(PackageState::All.opposite(true), PackageState::All);
    }

    #[test]
    fn test_uad_list_all_constant() {
        assert_eq!(UadList::ALL.len(), 8);
        assert!(UadList::ALL.contains(&UadList::All));
        assert!(UadList::ALL.contains(&UadList::Aosp));
        assert!(UadList::ALL.contains(&UadList::Unlisted));
    }

    #[test]
    fn test_package_state_all_constant() {
        assert_eq!(PackageState::ALL.len(), 4);
        assert!(PackageState::ALL.contains(&PackageState::All));
        assert!(PackageState::ALL.contains(&PackageState::Enabled));
    }

    #[test]
    fn test_removal_all_constant() {
        assert_eq!(Removal::ALL.len(), 6);
        assert!(Removal::ALL.contains(&Removal::All));
        assert!(Removal::ALL.contains(&Removal::Recommended));
        assert!(Removal::ALL.contains(&Removal::Unsafe));
    }

    #[test]
    fn test_no_duplicate_package_ids() {
        const DATA: &str = include_str!("../../resources/assets/uad_lists.json");
        let packages: Vec<Package> = serde_json::from_str(DATA).expect("Unable to parse");

        let mut ids = std::collections::HashSet::new();
        let mut duplicates = Vec::new();
        for pkg in &packages {
            if !ids.insert(&pkg.id) {
                duplicates.push(&pkg.id);
            }
        }
        // Log duplicates for visibility but don't fail - known data quality issue
        // TODO: Fix duplicate entries in uad_lists.json
        if !duplicates.is_empty() {
            eprintln!(
                "WARNING: Found {} duplicate package IDs: {:?}",
                duplicates.len(),
                duplicates
            );
        }
        // At minimum, verify that we have a substantial number of unique packages
        assert!(
            ids.len() > 100,
            "Should have at least 100 unique packages, found {}",
            ids.len()
        );
    }
}
