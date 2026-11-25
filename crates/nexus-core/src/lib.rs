// nexus-core: lib.rs
// Nexus Sovereign Software Sources, Mirror & Update Policy Studio - Core Crate

pub mod channel;
pub mod mirror;
pub mod feed;
pub mod policy;
pub mod auth;
pub mod driver;
pub mod developer;
pub mod updater;

pub use channel::{ChannelManager, ChannelType, RepoChannel};
pub use mirror::{DownloadMirror, MirrorManager, MirrorProtocol, MirrorStatus};
pub use feed::{FeedManager, PpaFeed};
pub use policy::{CheckFrequency, OtherUpdatePolicy, ReleaseUpgradeNotify, SecurityUpdatePolicy, UpdatePolicy};
pub use auth::{AuthKeyStore, KeyAlgorithm, SigningKey};
pub use driver::{DeviceCategory, DeviceDriver, DriverManager, HardwareDevice};
pub use developer::DeveloperOptions;
pub use updater::{PackageCategory, PendingPackage, SoftwareUpdaterEngine, UpdateStage};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NexusActiveTab {
    UbuntuSoftware,
    OtherSoftware,
    UpdatesPolicy,
    Authentication,
    AdditionalDrivers,
    DeveloperOptions,
    MirrorSpeedTest,
}

impl NexusActiveTab {
    pub fn title(&self) -> &str {
        match self {
            NexusActiveTab::UbuntuSoftware => "Ubuntu Software",
            NexusActiveTab::OtherSoftware => "Other Software",
            NexusActiveTab::UpdatesPolicy => "Updates",
            NexusActiveTab::Authentication => "Authentication",
            NexusActiveTab::AdditionalDrivers => "Additional Drivers",
            NexusActiveTab::DeveloperOptions => "Developer Options",
            NexusActiveTab::MirrorSpeedTest => "Choose a Download Server",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NexusWindowMode {
    SoftwareAndUpdates, // 7-tab configuration center
    SoftwareUpdater,    // Pending packages update dialog
}

#[derive(Debug, Clone)]
pub struct NexusStudio {
    pub mode: NexusWindowMode,
    pub active_tab: NexusActiveTab,
    pub channels: ChannelManager,
    pub mirrors: MirrorManager,
    pub feeds: FeedManager,
    pub policy: UpdatePolicy,
    pub auth_keys: AuthKeyStore,
    pub drivers: DriverManager,
    pub developer: DeveloperOptions,
    pub updater: SoftwareUpdaterEngine,
    pub status_message: String,
}

impl Default for NexusStudio {
    fn default() -> Self {
        Self::new()
    }
}

impl NexusStudio {
    pub fn new() -> Self {
        Self {
            mode: NexusWindowMode::SoftwareAndUpdates,
            active_tab: NexusActiveTab::UbuntuSoftware,
            channels: ChannelManager::new(),
            mirrors: MirrorManager::new(),
            feeds: FeedManager::new(),
            policy: UpdatePolicy::new(),
            auth_keys: AuthKeyStore::new(),
            drivers: DriverManager::new(),
            developer: DeveloperOptions::new(),
            updater: SoftwareUpdaterEngine::new(),
            status_message: "Ready. All repository channels and cryptographic keys loaded.".to_string(),
        }
    }

    pub fn set_tab(&mut self, tab: NexusActiveTab) {
        self.active_tab = tab;
        self.status_message = format!("Switched to tab: {}", tab.title());
    }

    pub fn set_mode(&mut self, mode: NexusWindowMode) {
        self.mode = mode;
        match mode {
            NexusWindowMode::SoftwareAndUpdates => {
                self.status_message = "Opened Software & Updates Settings Hub".to_string();
            }
            NexusWindowMode::SoftwareUpdater => {
                self.status_message = "Opened Software Updater (Checking for available upgrades)".to_string();
            }
        }
    }

    pub fn run_mirror_speed_test(&mut self) -> String {
        let best = self.mirrors.run_speed_benchmark();
        if let Some(ref id) = best {
            if let Some(m) = self.mirrors.get_mirror(id) {
                self.channels.set_active_server(&m.url);
                self.status_message = format!("Benchmark complete: Selected best server '{}' ({} ms)", m.name, m.latency_ms.unwrap_or(0));
                return m.name.clone();
            }
        }
        "Benchmark complete".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nexus_initialization() {
        let studio = NexusStudio::new();
        assert_eq!(studio.channels.channels.len(), 5);
        assert!(studio.channels.is_channel_enabled("main"));
        assert!(studio.channels.is_channel_enabled("universe"));
        assert!(!studio.channels.is_channel_enabled("source"));
        assert_eq!(studio.mirrors.mirrors.len(), 12);
        assert_eq!(studio.feeds.feeds.len(), 5);
        assert_eq!(studio.auth_keys.keys.len(), 4);
        assert_eq!(studio.drivers.devices.len(), 3);
        assert_eq!(studio.updater.pending_packages.len(), 5);
    }

    #[test]
    fn test_mirror_speed_benchmark() {
        let mut studio = NexusStudio::new();
        let best_name = studio.run_mirror_speed_test();
        assert!(!best_name.is_empty());
        let selected = studio.mirrors.get_selected_mirror();
        assert!(selected.latency_ms.is_some());
        assert!(selected.latency_ms.unwrap() > 0);
    }

    #[test]
    fn test_feed_addition_and_export() {
        let mut feeds = FeedManager::new();
        let id = feeds.add_feed_from_line("ppa:mozillateam/ppa").expect("PPA addition should succeed");
        assert!(id.starts_with("ppa-"));
        let deb_id = feeds.add_feed_from_line("deb https://packages.cloud.google.com/apt cloud-sdk main").expect("deb addition");
        assert!(deb_id.contains("cloud-google-com"));
        let exported = feeds.export_all_sources();
        assert!(exported.contains("mozillateam"));
        assert!(exported.contains("cloud.google.com"));
    }

    #[test]
    fn test_auth_key_import_and_remove() {
        let mut store = AuthKeyStore::new();
        let key_id = store.import_key_file("-----BEGIN PGP PUBLIC KEY BLOCK-----\nComment: Custom Repo Key\nmQGNBF9Z...\n-----END PGP PUBLIC KEY BLOCK-----").unwrap();
        assert!(!key_id.is_empty());
        assert!(store.keys.iter().any(|k| k.key_id == key_id));
        let removed = store.remove_key(&key_id);
        assert!(removed);
    }

    #[test]
    fn test_software_updater_flow() {
        let mut updater = SoftwareUpdaterEngine::new();
        let total_bytes = updater.total_download_bytes();
        assert!(total_bytes > 0);
        let display = updater.total_download_display();
        assert!(display.contains("MB will be downloaded"));

        // Toggle one package off, check that total reduces
        let first_id = updater.pending_packages[0].id.clone();
        updater.toggle_package_selection(&first_id);
        assert!(updater.total_download_bytes() < total_bytes);

        // Transaction steps
        assert_eq!(updater.current_stage, UpdateStage::Idle);
        updater.step_update_transaction();
        assert_eq!(updater.current_stage, UpdateStage::CheckingUpdates);
    }
}
