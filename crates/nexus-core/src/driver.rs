// nexus-core: driver.rs
// Additional Drivers: Hardware scan, proprietary DKMS modules, and alternative drivers

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DeviceCategory {
    Gpu,
    Wireless,
    ProcessorMicrocode,
    Ethernet,
}

impl DeviceCategory {
    pub fn as_str(&self) -> &str {
        match self {
            DeviceCategory::Gpu => "Graphics / GPU",
            DeviceCategory::Wireless => "Wireless Adapter",
            DeviceCategory::ProcessorMicrocode => "Processor Microcode",
            DeviceCategory::Ethernet => "Network Interface",
        }
    }
}

#[derive(Debug, Clone)]
pub struct DeviceDriver {
    pub id: String,
    pub title: String,
    pub version: String,
    pub is_proprietary: bool,
    pub is_open_source: bool,
    pub is_recommended: bool,
    pub license: String,
    pub active: bool,
}

impl DeviceDriver {
    pub fn new(
        id: &str,
        title: &str,
        version: &str,
        is_proprietary: bool,
        is_open_source: bool,
        is_recommended: bool,
        license: &str,
        active: bool,
    ) -> Self {
        Self {
            id: id.to_string(),
            title: title.to_string(),
            version: version.to_string(),
            is_proprietary,
            is_open_source,
            is_recommended,
            license: license.to_string(),
            active,
        }
    }
}

#[derive(Debug, Clone)]
pub struct HardwareDevice {
    pub id: String,
    pub vendor: String,
    pub model: String,
    pub category: DeviceCategory,
    pub drivers: Vec<DeviceDriver>,
    pub pending_driver_id: Option<String>,
}

impl HardwareDevice {
    pub fn get_active_driver(&self) -> Option<&DeviceDriver> {
        self.drivers.iter().find(|d| d.active)
    }

    pub fn select_driver(&mut self, driver_id: &str) -> bool {
        if self.drivers.iter().any(|d| d.id == driver_id) {
            self.pending_driver_id = Some(driver_id.to_string());
            true
        } else {
            false
        }
    }

    pub fn apply_selection(&mut self) -> bool {
        if let Some(target_id) = self.pending_driver_id.take() {
            for d in &mut self.drivers {
                d.active = d.id == target_id;
            }
            true
        } else {
            false
        }
    }

    pub fn revert_selection(&mut self) {
        self.pending_driver_id = None;
    }
}

#[derive(Debug, Clone)]
pub struct DriverManager {
    pub devices: Vec<HardwareDevice>,
    pub selected_device_idx: usize,
    pub has_unapplied_changes: bool,
}

impl Default for DriverManager {
    fn default() -> Self {
        Self::new()
    }
}

impl DriverManager {
    pub fn new() -> Self {
        let devices = vec![
            HardwareDevice {
                id: "pci-0000-01-00-0".to_string(),
                vendor: "NVIDIA Corporation".to_string(),
                model: "AD102 [GeForce RTX 4090 / VirtIO Accelerate]".to_string(),
                category: DeviceCategory::Gpu,
                drivers: vec![
                    DeviceDriver::new(
                        "nvidia-550-server",
                        "Using NVIDIA Server Driver (open kernel) from nvidia-driver-550-server (proprietary)",
                        "550.54.14",
                        true,
                        false,
                        false,
                        "NVIDIA Proprietary",
                        false,
                    ),
                    DeviceDriver::new(
                        "nvidia-550-desktop",
                        "Using NVIDIA driver (open kernel) metapackage from nvidia-driver-550 (proprietary, tested)",
                        "550.54.14",
                        true,
                        false,
                        true,
                        "NVIDIA Proprietary",
                        true, // Currently active
                    ),
                    DeviceDriver::new(
                        "nouveau-open",
                        "Using X.Org X server -- Nouveau display driver from xserver-xorg-video-nouveau (open source)",
                        "1.0.17",
                        false,
                        true,
                        false,
                        "MIT / GPL",
                        false,
                    ),
                    DeviceDriver::new(
                        "disable-gpu",
                        "Do not use the device",
                        "0.0.0",
                        false,
                        false,
                        false,
                        "N/A",
                        false,
                    ),
                ],
                pending_driver_id: None,
            },
            HardwareDevice {
                id: "pci-0000-02-00-0".to_string(),
                vendor: "Broadcom Inc. and subsidiaries".to_string(),
                model: "BCM4360 802.11ac Wireless Network Adapter".to_string(),
                category: DeviceCategory::Wireless,
                drivers: vec![
                    DeviceDriver::new(
                        "bcmwl-sta",
                        "Using Broadcom 802.11 Linux STA wireless driver source from bcmwl-kernel-source (proprietary)",
                        "6.30.223.271",
                        true,
                        false,
                        true,
                        "Broadcom Restricted",
                        true,
                    ),
                    DeviceDriver::new(
                        "disable-wifi",
                        "Do not use the device",
                        "0.0.0",
                        false,
                        false,
                        false,
                        "N/A",
                        false,
                    ),
                ],
                pending_driver_id: None,
            },
            HardwareDevice {
                id: "cpu-microcode".to_string(),
                vendor: "Intel Corporation".to_string(),
                model: "Processor Microcode Firmware (13th/14th Gen Core)".to_string(),
                category: DeviceCategory::ProcessorMicrocode,
                drivers: vec![
                    DeviceDriver::new(
                        "intel-microcode-active",
                        "Using Processor microcode firmware for Intel CPUs from intel-microcode (proprietary)",
                        "3.20240312.1",
                        true,
                        false,
                        true,
                        "Intel Microcode License",
                        true,
                    ),
                    DeviceDriver::new(
                        "disable-microcode",
                        "Do not use the device",
                        "0.0.0",
                        false,
                        false,
                        false,
                        "N/A",
                        false,
                    ),
                ],
                pending_driver_id: None,
            },
        ];

        Self {
            devices,
            selected_device_idx: 0,
            has_unapplied_changes: false,
        }
    }

    pub fn select_device_driver(&mut self, device_id: &str, driver_id: &str) -> bool {
        if let Some(dev) = self.devices.iter_mut().find(|d| d.id == device_id) {
            let ok = dev.select_driver(driver_id);
            self.has_unapplied_changes = self.devices.iter().any(|d| d.pending_driver_id.is_some());
            ok
        } else {
            false
        }
    }

    pub fn apply_all_driver_changes(&mut self) -> usize {
        let mut count = 0;
        for dev in &mut self.devices {
            if dev.apply_selection() {
                count += 1;
            }
        }
        self.has_unapplied_changes = false;
        count
    }

    pub fn revert_all_driver_changes(&mut self) {
        for dev in &mut self.devices {
            dev.revert_selection();
        }
        self.has_unapplied_changes = false;
    }
}
