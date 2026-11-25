// nexus-core: developer.rs
// Developer Options: Proposed release channels, debug logs, staging branches

#[derive(Debug, Clone)]
pub struct DeveloperOptions {
    pub proposed_channel_enabled: bool,
    pub proposed_channel_name: String,
    pub debug_logging_enabled: bool,
    pub staging_server_url: Option<String>,
}

impl Default for DeveloperOptions {
    fn default() -> Self {
        Self::new()
    }
}

impl DeveloperOptions {
    pub fn new() -> Self {
        Self {
            proposed_channel_enabled: false,
            proposed_channel_name: "noble-proposed (Pre-release testing)".to_string(),
            debug_logging_enabled: false,
            staging_server_url: None,
        }
    }

    pub fn toggle_proposed_channel(&mut self) {
        self.proposed_channel_enabled = !self.proposed_channel_enabled;
    }

    pub fn toggle_debug_logging(&mut self) {
        self.debug_logging_enabled = !self.debug_logging_enabled;
    }

    pub fn set_staging_server(&mut self, url: Option<&str>) {
        self.staging_server_url = url.map(|s| s.to_string());
    }
}
