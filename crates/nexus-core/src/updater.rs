// nexus-core: updater.rs
// Software Updater Interactive Engine: Pending updates manifest, changelog, download size calculation, and execution stages

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PackageCategory {
    Security,
    CoreSystem,
    Applications,
    Libraries,
}

impl PackageCategory {
    pub fn as_str(&self) -> &str {
        match self {
            PackageCategory::Security => "Security updates",
            PackageCategory::CoreSystem => "NovaOS Core & Base System",
            PackageCategory::Applications => "Applications & Desktop",
            PackageCategory::Libraries => "Shared Libraries",
        }
    }
}

#[derive(Debug, Clone)]
pub struct PendingPackage {
    pub id: String,
    pub name: String,
    pub category: PackageCategory,
    pub current_version: String,
    pub new_version: String,
    pub size_bytes: u64,
    pub summary: String,
    pub changelog: String,
    pub selected: bool,
}

impl PendingPackage {
    pub fn new(
        id: &str,
        name: &str,
        category: PackageCategory,
        curr: &str,
        next: &str,
        size_bytes: u64,
        summary: &str,
        changelog: &str,
    ) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            category,
            current_version: curr.to_string(),
            new_version: next.to_string(),
            size_bytes,
            summary: summary.to_string(),
            changelog: changelog.to_string(),
            selected: true,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UpdateStage {
    Idle,
    CheckingUpdates,
    ResolvingDependencies,
    DownloadingPackages { current_pkg: String, progress_pct: u8, speed_mbps: f32 },
    VerifyingSignatures { verified_keys: usize, total_keys: usize },
    UnpackingAndInstalling { current_pkg: String, step_num: usize, total_steps: usize },
    Completed { packages_updated: usize, bytes_saved: u64 },
    Failed(String),
}

#[derive(Debug, Clone)]
pub struct SoftwareUpdaterEngine {
    pub pending_packages: Vec<PendingPackage>,
    pub selected_package_id: Option<String>,
    pub current_stage: UpdateStage,
    pub last_checked_timestamp: String,
}

impl Default for SoftwareUpdaterEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl SoftwareUpdaterEngine {
    pub fn new() -> Self {
        let pending = vec![
            PendingPackage::new(
                "claude",
                "Claude Desktop Assistant",
                PackageCategory::Applications,
                "0.7.12",
                "0.8.0-1~noble",
                82_400_000, // 82.4 MB
                "Official Anthropic Claude desktop client runtime and reasoning engine",
                "- Enhanced artifact rendering\n- Improved memory retention\n- Security patch for local IPC protocol",
            ),
            PendingPackage::new(
                "mongodb-mongocryptd",
                "MongoDB Client-Side Field Level Encryption Helper",
                PackageCategory::Applications,
                "7.0.5",
                "7.0.8-1~noble",
                45_200_000, // 45.2 MB
                "Cryptographic daemon for MongoDB transparent field-level encryption",
                "- Fixed buffer validation in cipher context\n- OpenSSL 3.0.13 interoperability update",
            ),
            PendingPackage::new(
                "linux-novaos-kernel",
                "NovaOS Ring-0 Sovereign Microkernel & Drivers",
                PackageCategory::CoreSystem,
                "6.11.0-nova4",
                "6.11.0-nova5",
                42_100_000, // 42.1 MB
                "High performance memory safety microkernel with Ed25519 syscall verification",
                "- Mitigated Spectre v4 transient execution branch\n- Enhanced VirtIO framebuffer double-buffering throughput\n- Improved scheduler latency for multimedia threads",
            ),
            PendingPackage::new(
                "openssl-core",
                "OpenSSL Cryptography and SSL/TLS Core Runtime",
                PackageCategory::Security,
                "3.0.13-0ubuntu3",
                "3.0.13-0ubuntu3.4",
                14_800_000, // 14.8 MB
                "Secure Sockets Layer toolkit and cryptographic libraries (CVE-2024-2511)",
                "- High severity CVE-2024-2511 fix for TLS session ticket unbounded growth\n- Speed optimization for AES-GCM and ChaCha20-Poly1305",
            ),
            PendingPackage::new(
                "libv8-3.14",
                "V8 JavaScript Engine Embedding Shared Library",
                PackageCategory::Libraries,
                "3.14.5.10-9",
                "3.14.5.10-10",
                7_400_000, // 7.4 MB
                "High performance JavaScript runtime shared object",
                "- Memory alignment fixes for ARM64 and x86_64\n- Compiler warning cleanup",
            ),
        ];

        let selected = pending.first().map(|p| p.id.clone());

        Self {
            pending_packages: pending,
            selected_package_id: selected,
            current_stage: UpdateStage::Idle,
            last_checked_timestamp: "2026-11-25 17:50:00 UTC".to_string(),
        }
    }

    pub fn total_download_bytes(&self) -> u64 {
        self.pending_packages
            .iter()
            .filter(|p| p.selected)
            .map(|p| p.size_bytes)
            .sum()
    }

    pub fn total_download_display(&self) -> String {
        let bytes = self.total_download_bytes();
        let mb = bytes as f64 / (1024.0 * 1024.0);
        format!("{:.1} MB will be downloaded.", mb)
    }

    pub fn toggle_package_selection(&mut self, id: &str) {
        if let Some(pkg) = self.pending_packages.iter_mut().find(|p| p.id == id) {
            pkg.selected = !pkg.selected;
        }
    }

    pub fn select_all(&mut self, select: bool) {
        for pkg in &mut self.pending_packages {
            pkg.selected = select;
        }
    }

    pub fn count_selected(&self) -> usize {
        self.pending_packages.iter().filter(|p| p.selected).count()
    }

    pub fn categories(&self) -> Vec<PackageCategory> {
        vec![
            PackageCategory::Security,
            PackageCategory::CoreSystem,
            PackageCategory::Applications,
            PackageCategory::Libraries,
        ]
    }

    pub fn packages_in_category(&self, cat: &PackageCategory) -> Vec<&PendingPackage> {
        self.pending_packages
            .iter()
            .filter(|p| &p.category == cat)
            .collect()
    }

    /// Dynamic update execution simulation
    pub fn step_update_transaction(&mut self) -> UpdateStage {
        match &self.current_stage {
            UpdateStage::Idle => {
                self.current_stage = UpdateStage::CheckingUpdates;
            }
            UpdateStage::CheckingUpdates => {
                self.current_stage = UpdateStage::ResolvingDependencies;
            }
            UpdateStage::ResolvingDependencies => {
                let first_pkg = self.pending_packages.first().map(|p| p.name.clone()).unwrap_or_else(|| "package".to_string());
                self.current_stage = UpdateStage::DownloadingPackages {
                    current_pkg: first_pkg,
                    progress_pct: 35,
                    speed_mbps: 24.5,
                };
            }
            UpdateStage::DownloadingPackages { progress_pct, .. } => {
                if *progress_pct < 95 {
                    self.current_stage = UpdateStage::DownloadingPackages {
                        current_pkg: "linux-novaos-kernel".to_string(),
                        progress_pct: progress_pct + 35,
                        speed_mbps: 31.8,
                    };
                } else {
                    self.current_stage = UpdateStage::VerifyingSignatures {
                        verified_keys: 5,
                        total_keys: 5,
                    };
                }
            }
            UpdateStage::VerifyingSignatures { .. } => {
                self.current_stage = UpdateStage::UnpackingAndInstalling {
                    current_pkg: "Configuring NovaOS Kernel & Drivers".to_string(),
                    step_num: 3,
                    total_steps: 5,
                };
            }
            UpdateStage::UnpackingAndInstalling { .. } => {
                let count = self.count_selected();
                let bytes = self.total_download_bytes();
                self.current_stage = UpdateStage::Completed {
                    packages_updated: count,
                    bytes_saved: bytes,
                };
            }
            UpdateStage::Completed { .. } => {
                // Keep completed
            }
            UpdateStage::Failed(_) => {
                // Keep failed
            }
        }
        self.current_stage.clone()
    }
}
