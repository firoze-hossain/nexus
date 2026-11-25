// nexus-ui: lib.rs
// Interactive state machine and event dispatcher for Nexus

use nexus_core::*;
use nexus_render::NexusRenderer;
use nexus_storage::StorageManager;

#[derive(Debug, Clone)]
pub enum UiEvent {
    SwitchTab(NexusActiveTab),
    SwitchMode(NexusWindowMode),
    ToggleChannel(String),
    OpenMirrorDialog,
    CloseMirrorDialog,
    SelectMirror(String),
    RunBenchmark,
    AddFeed(String),
    RemoveSelectedFeed,
    ToggleFeed(String),
    SelectFeed(String),
    SetCheckFrequency(CheckFrequency),
    SetSecurityPolicy(SecurityUpdatePolicy),
    ImportKey(String),
    RemoveSelectedKey,
    RestoreDefaultKeys,
    SelectDriver { device_id: String, driver_id: String },
    ApplyDriverChanges,
    RevertDriverChanges,
    ToggleProposedChannel,
    ToggleDebugLogging,
    TogglePackage(String),
    SelectAllPackages(bool),
    StepUpdateTransaction,
    Click { x: usize, y: usize },
}

pub struct NexusUiController {
    pub studio: NexusStudio,
    pub storage: StorageManager,
    pub renderer: NexusRenderer,
}

impl NexusUiController {
    pub fn new(width: usize, height: usize, config_dir: &str) -> Self {
        Self {
            studio: NexusStudio::new(),
            storage: StorageManager::new(config_dir),
            renderer: NexusRenderer::new(width, height),
        }
    }

    pub fn dispatch(&mut self, event: UiEvent) {
        match event {
            UiEvent::SwitchTab(tab) => {
                self.studio.set_tab(tab);
            }
            UiEvent::SwitchMode(mode) => {
                self.studio.set_mode(mode);
            }
            UiEvent::ToggleChannel(id) => {
                self.studio.channels.toggle_channel(&id);
                self.studio.status_message = format!("Toggled channel '{}'", id);
            }
            UiEvent::OpenMirrorDialog => {
                self.studio.set_tab(NexusActiveTab::MirrorSpeedTest);
            }
            UiEvent::CloseMirrorDialog => {
                self.studio.set_tab(NexusActiveTab::UbuntuSoftware);
            }
            UiEvent::SelectMirror(id) => {
                if self.studio.mirrors.select_mirror(&id) {
                    if let Some(m) = self.studio.mirrors.get_mirror(&id) {
                        self.studio.channels.set_active_server(&m.url);
                        self.studio.status_message = format!("Selected mirror '{}'", m.name);
                    }
                }
            }
            UiEvent::RunBenchmark => {
                let best = self.studio.run_mirror_speed_test();
                self.studio.status_message = format!("Benchmark finished. Fastest: {}", best);
            }
            UiEvent::AddFeed(line) => {
                match self.studio.feeds.add_feed_from_line(&line) {
                    Ok(id) => self.studio.status_message = format!("Added repository feed '{}'", id),
                    Err(e) => self.studio.status_message = format!("Error: {}", e),
                }
            }
            UiEvent::RemoveSelectedFeed => {
                if let Some(ref id) = self.studio.feeds.selected_feed_id.clone() {
                    self.studio.feeds.remove_feed(id);
                    self.studio.status_message = format!("Removed repository feed '{}'", id);
                }
            }
            UiEvent::ToggleFeed(id) => {
                self.studio.feeds.toggle_feed(&id);
                self.studio.status_message = format!("Toggled repository feed '{}'", id);
            }
            UiEvent::SelectFeed(id) => {
                self.studio.feeds.selected_feed_id = Some(id);
            }
            UiEvent::SetCheckFrequency(freq) => {
                self.studio.policy.set_check_frequency(freq);
                self.studio.status_message = format!("Update check frequency set to: {}", freq.as_str());
            }
            UiEvent::SetSecurityPolicy(sec) => {
                self.studio.policy.set_security_policy(sec);
                self.studio.status_message = format!("Security updates policy set to: {}", sec.as_str());
            }
            UiEvent::ImportKey(content) => {
                match self.studio.auth_keys.import_key_file(&content) {
                    Ok(kid) => self.studio.status_message = format!("Imported signing key '{}'", kid),
                    Err(e) => self.studio.status_message = format!("Key import error: {}", e),
                }
            }
            UiEvent::RemoveSelectedKey => {
                if let Some(ref kid) = self.studio.auth_keys.selected_key_id.clone() {
                    self.studio.auth_keys.remove_key(kid);
                    self.studio.status_message = format!("Removed signing key '{}'", kid);
                }
            }
            UiEvent::RestoreDefaultKeys => {
                self.studio.auth_keys.restore_defaults();
                self.studio.status_message = "Restored default cryptographic signing keys".to_string();
            }
            UiEvent::SelectDriver { device_id, driver_id } => {
                self.studio.drivers.select_device_driver(&device_id, &driver_id);
                self.studio.status_message = format!("Selected driver '{}' for device '{}'", driver_id, device_id);
            }
            UiEvent::ApplyDriverChanges => {
                let count = self.studio.drivers.apply_all_driver_changes();
                self.studio.status_message = format!("Applied {} driver change(s)", count);
            }
            UiEvent::RevertDriverChanges => {
                self.studio.drivers.revert_all_driver_changes();
                self.studio.status_message = "Reverted driver changes".to_string();
            }
            UiEvent::ToggleProposedChannel => {
                self.studio.developer.toggle_proposed_channel();
                self.studio.status_message = format!(
                    "Pre-release proposed channel is now {}",
                    if self.studio.developer.proposed_channel_enabled { "ENABLED" } else { "DISABLED" }
                );
            }
            UiEvent::ToggleDebugLogging => {
                self.studio.developer.toggle_debug_logging();
            }
            UiEvent::TogglePackage(id) => {
                self.studio.updater.toggle_package_selection(&id);
                self.studio.status_message = self.studio.updater.total_download_display();
            }
            UiEvent::SelectAllPackages(select) => {
                self.studio.updater.select_all(select);
                self.studio.status_message = self.studio.updater.total_download_display();
            }
            UiEvent::StepUpdateTransaction => {
                let stage = self.studio.updater.step_update_transaction();
                self.studio.status_message = format!("Update execution: {:?}", stage);
            }
            UiEvent::Click { x, y } => {
                self.handle_click(x, y);
            }
        }

        self.renderer.render(&self.studio);
    }

    fn handle_click(&mut self, x: usize, y: usize) {
        // Mode switch button in title bar (top right)
        if y < 36 && x > self.renderer.buffer.width - 180 {
            if self.studio.mode == NexusWindowMode::SoftwareAndUpdates {
                self.studio.set_mode(NexusWindowMode::SoftwareUpdater);
            } else {
                self.studio.set_mode(NexusWindowMode::SoftwareAndUpdates);
            }
            return;
        }

        if self.studio.mode == NexusWindowMode::SoftwareAndUpdates {
            // Check Tab bar clicks (y between 44 and 72)
            if (44..=72).contains(&y) {
                let tab_w = 124;
                let rel_x = x.saturating_sub(16);
                let tab_idx = rel_x / tab_w;
                match tab_idx {
                    0 => self.studio.set_tab(NexusActiveTab::UbuntuSoftware),
                    1 => self.studio.set_tab(NexusActiveTab::OtherSoftware),
                    2 => self.studio.set_tab(NexusActiveTab::UpdatesPolicy),
                    3 => self.studio.set_tab(NexusActiveTab::Authentication),
                    4 => self.studio.set_tab(NexusActiveTab::AdditionalDrivers),
                    5 => self.studio.set_tab(NexusActiveTab::DeveloperOptions),
                    _ => {}
                }
            } else if self.studio.active_tab == NexusActiveTab::UbuntuSoftware {
                // Check if clicked "Select Mirror..." button
                if (230..=265).contains(&y) && (510..=670).contains(&x) {
                    self.studio.set_tab(NexusActiveTab::MirrorSpeedTest);
                }
            } else if self.studio.active_tab == NexusActiveTab::MirrorSpeedTest {
                // Inside mirror dialog: "Select Best Server"
                let mh = 480;
                let my = (self.renderer.buffer.height - mh) / 2;
                let btn_y = my + mh - 44;
                if (btn_y..=btn_y + 30).contains(&y) {
                    let mw = 680;
                    let mx = (self.renderer.buffer.width - mw) / 2;
                    if (mx + 20..=mx + 190).contains(&x) {
                        self.dispatch(UiEvent::RunBenchmark);
                    } else if (mx + mw - 230..=mx + mw - 120).contains(&x) || (mx + mw - 110..=mx + mw - 20).contains(&x) {
                        self.studio.set_tab(NexusActiveTab::UbuntuSoftware);
                    }
                }
            }
        } else if self.studio.mode == NexusWindowMode::SoftwareUpdater {
            // Software Updater mode clicks: "Settings..." button
            let btn_y = self.renderer.buffer.height - 44;
            if (btn_y..=btn_y + 30).contains(&y) {
                if (20..=130).contains(&x) {
                    self.studio.set_mode(NexusWindowMode::SoftwareAndUpdates);
                } else if x > self.renderer.buffer.width - 140 {
                    self.dispatch(UiEvent::StepUpdateTransaction);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ui_controller_dispatch_and_flow() {
        let mut controller = NexusUiController::new(800, 600, "/tmp/nexus");

        // Verify initial state
        assert_eq!(controller.studio.active_tab, NexusActiveTab::UbuntuSoftware);
        assert_eq!(controller.studio.mode, NexusWindowMode::SoftwareAndUpdates);

        // Switch tab
        controller.dispatch(UiEvent::SwitchTab(NexusActiveTab::UpdatesPolicy));
        assert_eq!(controller.studio.active_tab, NexusActiveTab::UpdatesPolicy);

        // Benchmark mirrors
        controller.dispatch(UiEvent::RunBenchmark);
        assert!(controller.studio.mirrors.get_selected_mirror().latency_ms.is_some());

        // Switch mode to Software Updater
        controller.dispatch(UiEvent::SwitchMode(NexusWindowMode::SoftwareUpdater));
        assert_eq!(controller.studio.mode, NexusWindowMode::SoftwareUpdater);

        // Step update transaction
        controller.dispatch(UiEvent::StepUpdateTransaction);
        assert_ne!(controller.studio.updater.current_stage, UpdateStage::Idle);

        // Switch back to Settings
        controller.dispatch(UiEvent::SwitchMode(NexusWindowMode::SoftwareAndUpdates));
        assert_eq!(controller.studio.mode, NexusWindowMode::SoftwareAndUpdates);
    }
}
