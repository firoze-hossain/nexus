// nexus-core: policy.rs
// Automated updates schedules, security update automation, and LTS notifications

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CheckFrequency {
    Daily,
    EveryTwoDays,
    Weekly,
    EveryTwoWeeks,
    Never,
}

impl CheckFrequency {
    pub fn as_str(&self) -> &str {
        match self {
            CheckFrequency::Daily => "Daily",
            CheckFrequency::EveryTwoDays => "Every two days",
            CheckFrequency::Weekly => "Weekly",
            CheckFrequency::EveryTwoWeeks => "Every two weeks",
            CheckFrequency::Never => "Never",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SecurityUpdatePolicy {
    DownloadAndInstallAuto,
    DownloadAutoAndNotify,
    DisplayImmediately,
}

impl SecurityUpdatePolicy {
    pub fn as_str(&self) -> &str {
        match self {
            SecurityUpdatePolicy::DownloadAndInstallAuto => "Download and install automatically",
            SecurityUpdatePolicy::DownloadAutoAndNotify => "Download automatically and notify",
            SecurityUpdatePolicy::DisplayImmediately => "Display immediately",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OtherUpdatePolicy {
    DisplayImmediately,
    DisplayWeekly,
    DisplayEveryTwoWeeks,
}

impl OtherUpdatePolicy {
    pub fn as_str(&self) -> &str {
        match self {
            OtherUpdatePolicy::DisplayImmediately => "Display immediately",
            OtherUpdatePolicy::DisplayWeekly => "Display weekly",
            OtherUpdatePolicy::DisplayEveryTwoWeeks => "Display every two weeks",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReleaseUpgradeNotify {
    ForAnyNewVersion,
    ForLongTermSupportVersions,
    Never,
}

impl ReleaseUpgradeNotify {
    pub fn as_str(&self) -> &str {
        match self {
            ReleaseUpgradeNotify::ForAnyNewVersion => "For any new version",
            ReleaseUpgradeNotify::ForLongTermSupportVersions => "For long-term support versions",
            ReleaseUpgradeNotify::Never => "Never",
        }
    }
}

#[derive(Debug, Clone)]
pub struct UpdatePolicy {
    pub check_frequency: CheckFrequency,
    pub security_updates: SecurityUpdatePolicy,
    pub other_updates: OtherUpdatePolicy,
    pub release_upgrade: ReleaseUpgradeNotify,
    pub esm_pro_active: bool,
    pub esm_pro_expiry: String,
    pub livepatch_enabled: bool,
}

impl Default for UpdatePolicy {
    fn default() -> Self {
        Self::new()
    }
}

impl UpdatePolicy {
    pub fn new() -> Self {
        Self {
            check_frequency: CheckFrequency::Daily,
            security_updates: SecurityUpdatePolicy::DownloadAndInstallAuto,
            other_updates: OtherUpdatePolicy::DisplayWeekly,
            release_upgrade: ReleaseUpgradeNotify::ForLongTermSupportVersions,
            esm_pro_active: true,
            esm_pro_expiry: "05/31/2029".to_string(), // Matches Screenshot 18-53-18
            livepatch_enabled: true,
        }
    }

    pub fn set_check_frequency(&mut self, freq: CheckFrequency) {
        self.check_frequency = freq;
    }

    pub fn set_security_policy(&mut self, policy: SecurityUpdatePolicy) {
        self.security_updates = policy;
    }

    pub fn set_other_policy(&mut self, policy: OtherUpdatePolicy) {
        self.other_updates = policy;
    }

    pub fn set_release_upgrade(&mut self, notify: ReleaseUpgradeNotify) {
        self.release_upgrade = notify;
    }
}
