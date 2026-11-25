// nexus-render: lib.rs
// Pure 2D software double-buffered vector rasterizer for Nexus

use nexus_core::*;

pub const COLOR_BG: u32 = 0xFFF2F2F2;
pub const COLOR_PANEL_BG: u32 = 0xFFFFFFFF;
pub const COLOR_HEADER_BG: u32 = 0xFF353535;
pub const COLOR_HEADER_TEXT: u32 = 0xFFFFFFFF;
pub const COLOR_TEXT_PRIMARY: u32 = 0xFF222222;
pub const COLOR_TEXT_MUTED: u32 = 0xFF666666;
pub const COLOR_ACCENT_UBUNTU: u32 = 0xFFE95420; // Canonical Ubuntu Orange
pub const COLOR_ACCENT_HOVER: u32 = 0xFFFF7043;
pub const COLOR_BORDER: u32 = 0xFFCCCCCC;
pub const COLOR_SELECTED_ROW: u32 = 0xFFE0E0E0;
pub const COLOR_SUCCESS_GREEN: u32 = 0xFF388E3C;
pub const COLOR_WARNING_AMBER: u32 = 0xFFF57C00;
pub const COLOR_DANGER_RED: u32 = 0xFFD32F2F;

pub struct PixelBuffer {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<u32>,
}

impl PixelBuffer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            pixels: vec![COLOR_BG; width * height],
        }
    }

    pub fn clear(&mut self, color: u32) {
        for p in &mut self.pixels {
            *p = color;
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: u32) {
        if x < self.width && y < self.height {
            self.pixels[y * self.width + x] = color;
        }
    }

    pub fn fill_rect(&mut self, x: usize, y: usize, w: usize, h: usize, color: u32) {
        let x_end = (x + w).min(self.width);
        let y_end = (y + h).min(self.height);
        for cy in y..y_end {
            let row_offset = cy * self.width;
            for cx in x..x_end {
                self.pixels[row_offset + cx] = color;
            }
        }
    }

    pub fn draw_rect(&mut self, x: usize, y: usize, w: usize, h: usize, color: u32) {
        if w == 0 || h == 0 {
            return;
        }
        self.fill_rect(x, y, w, 1, color);
        self.fill_rect(x, y + h - 1, w, 1, color);
        self.fill_rect(x, y, 1, h, color);
        self.fill_rect(x + w - 1, y, 1, h, color);
    }

    pub fn draw_text(&mut self, x: usize, y: usize, text: &str, color: u32) {
        let mut cx = x;
        for c in text.chars() {
            if cx + 8 > self.width {
                break;
            }
            if c == '\n' {
                cx = x;
                continue;
            }
            self.draw_char_8x8(cx, y, c, color);
            cx += 8;
        }
    }

    fn draw_char_8x8(&mut self, x: usize, y: usize, c: char, color: u32) {
        let glyph = get_glyph_bitmap(c);
        for row in 0..8 {
            let bits = glyph[row];
            for col in 0..8 {
                if (bits & (1 << (7 - col))) != 0 {
                    self.set_pixel(x + col, y + row, color);
                }
            }
        }
    }

    pub fn draw_button(&mut self, x: usize, y: usize, w: usize, h: usize, label: &str, is_primary: bool) {
        let bg = if is_primary { COLOR_ACCENT_UBUNTU } else { 0xFFEAEAEA };
        let text_color = if is_primary { 0xFFFFFFFF } else { COLOR_TEXT_PRIMARY };
        self.fill_rect(x, y, w, h, bg);
        self.draw_rect(x, y, w, h, if is_primary { 0xFFC4380D } else { COLOR_BORDER });
        let text_w = label.len() * 8;
        let tx = if text_w < w { x + (w - text_w) / 2 } else { x + 4 };
        let ty = if h > 8 { y + (h - 8) / 2 } else { y };
        self.draw_text(tx, ty, label, text_color);
    }

    pub fn draw_checkbox(&mut self, x: usize, y: usize, checked: bool, label: &str) {
        self.fill_rect(x, y, 14, 14, 0xFFFFFFFF);
        self.draw_rect(x, y, 14, 14, COLOR_BORDER);
        if checked {
            self.fill_rect(x + 3, y + 3, 8, 8, COLOR_ACCENT_UBUNTU);
        }
        self.draw_text(x + 20, y + 3, label, COLOR_TEXT_PRIMARY);
    }

    pub fn draw_radio(&mut self, x: usize, y: usize, checked: bool, label: &str) {
        self.fill_rect(x, y, 14, 14, 0xFFFFFFFF);
        self.draw_rect(x, y, 14, 14, COLOR_BORDER);
        if checked {
            self.fill_rect(x + 4, y + 4, 6, 6, COLOR_ACCENT_UBUNTU);
        }
        self.draw_text(x + 20, y + 3, label, COLOR_TEXT_PRIMARY);
    }

    pub fn draw_progress_bar(&mut self, x: usize, y: usize, w: usize, h: usize, pct: u8) {
        self.fill_rect(x, y, w, h, 0xFFE0E0E0);
        self.draw_rect(x, y, w, h, COLOR_BORDER);
        let filled_w = ((w as u64 * pct as u64) / 100) as usize;
        if filled_w > 0 {
            self.fill_rect(x, y, filled_w, h, COLOR_ACCENT_UBUNTU);
        }
    }
}

pub struct NexusRenderer {
    pub buffer: PixelBuffer,
}

impl NexusRenderer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            buffer: PixelBuffer::new(width, height),
        }
    }

    pub fn render(&mut self, studio: &NexusStudio) {
        self.buffer.clear(COLOR_BG);

        match studio.mode {
            NexusWindowMode::SoftwareAndUpdates => {
                self.render_software_and_updates(studio);
            }
            NexusWindowMode::SoftwareUpdater => {
                self.render_software_updater(studio);
            }
        }
    }

    fn render_software_and_updates(&mut self, studio: &NexusStudio) {
        // Window Title Bar
        self.buffer.fill_rect(0, 0, self.buffer.width, 36, COLOR_HEADER_BG);
        self.buffer.draw_text(16, 12, "Software & Updates", COLOR_HEADER_TEXT);

        // Mode switch pill button in title bar
        self.buffer.draw_button(self.buffer.width - 180, 6, 160, 24, "Open Updater...", false);

        // Tab Navigation Bar
        let tabs = [
            (NexusActiveTab::UbuntuSoftware, "Ubuntu Software"),
            (NexusActiveTab::OtherSoftware, "Other Software"),
            (NexusActiveTab::UpdatesPolicy, "Updates"),
            (NexusActiveTab::Authentication, "Authentication"),
            (NexusActiveTab::AdditionalDrivers, "Additional Drivers"),
            (NexusActiveTab::DeveloperOptions, "Developer Options"),
        ];

        let tab_w = 120;
        let mut cur_x = 16;
        let tab_y = 44;

        for (tab, title) in tabs.iter() {
            let is_active = studio.active_tab == *tab;
            let bg = if is_active { COLOR_PANEL_BG } else { 0xFFE0E0E0 };
            let fg = if is_active { COLOR_ACCENT_UBUNTU } else { COLOR_TEXT_MUTED };
            self.buffer.fill_rect(cur_x, tab_y, tab_w, 28, bg);
            self.buffer.draw_rect(cur_x, tab_y, tab_w, 28, COLOR_BORDER);
            if is_active {
                self.buffer.fill_rect(cur_x, tab_y, tab_w, 3, COLOR_ACCENT_UBUNTU);
            }
            self.buffer.draw_text(cur_x + 8, tab_y + 10, title, fg);
            cur_x += tab_w + 4;
        }

        // Main Panel Body
        let panel_x = 16;
        let panel_y = 76;
        let panel_w = self.buffer.width - 32;
        let panel_h = self.buffer.height - 128;
        self.buffer.fill_rect(panel_x, panel_y, panel_w, panel_h, COLOR_PANEL_BG);
        self.buffer.draw_rect(panel_x, panel_y, panel_w, panel_h, COLOR_BORDER);

        match studio.active_tab {
            NexusActiveTab::UbuntuSoftware => self.render_tab_ubuntu_software(studio, panel_x, panel_y, panel_w, panel_h),
            NexusActiveTab::OtherSoftware => self.render_tab_other_software(studio, panel_x, panel_y, panel_w, panel_h),
            NexusActiveTab::UpdatesPolicy => self.render_tab_updates(studio, panel_x, panel_y, panel_w, panel_h),
            NexusActiveTab::Authentication => self.render_tab_authentication(studio, panel_x, panel_y, panel_w, panel_h),
            NexusActiveTab::AdditionalDrivers => self.render_tab_additional_drivers(studio, panel_x, panel_y, panel_w, panel_h),
            NexusActiveTab::DeveloperOptions => self.render_tab_developer_options(studio, panel_x, panel_y, panel_w, panel_h),
            NexusActiveTab::MirrorSpeedTest => self.render_modal_mirror_selection(studio),
        }

        // Bottom Footer Bar
        let footer_y = self.buffer.height - 44;
        self.buffer.draw_text(20, footer_y + 14, &studio.status_message, COLOR_TEXT_MUTED);
        self.buffer.draw_button(self.buffer.width - 110, footer_y + 6, 90, 28, "Close", false);
    }

    fn render_tab_ubuntu_software(&mut self, studio: &NexusStudio, x: usize, y: usize, w: usize, _h: usize) {
        self.buffer.draw_text(x + 20, y + 20, "Downloadable from the Internet", COLOR_TEXT_PRIMARY);

        let mut row_y = y + 46;
        for ch in &studio.channels.channels {
            self.buffer.draw_checkbox(x + 24, row_y, ch.enabled, &ch.name);
            row_y += 26;
        }

        row_y += 10;
        self.buffer.fill_rect(x + 20, row_y, w - 40, 1, COLOR_BORDER);
        row_y += 16;

        self.buffer.draw_text(x + 20, row_y + 6, "Download from:", COLOR_TEXT_PRIMARY);
        let selected_mirror = studio.mirrors.get_selected_mirror();
        self.buffer.fill_rect(x + 140, row_y, 360, 26, 0xFFF9F9F9);
        self.buffer.draw_rect(x + 140, row_y, 360, 26, COLOR_BORDER);
        self.buffer.draw_text(x + 148, row_y + 9, &selected_mirror.name, COLOR_TEXT_PRIMARY);

        // "Choose Mirror / Speed Test..." button
        self.buffer.draw_button(x + 510, row_y, 160, 26, "Select Mirror...", true);

        row_y += 40;
        self.buffer.fill_rect(x + 20, row_y, w - 40, 1, COLOR_BORDER);
        row_y += 16;

        self.buffer.draw_text(x + 20, row_y, "Installable from CD-ROM/DVD", COLOR_TEXT_PRIMARY);
        self.buffer.draw_checkbox(x + 24, row_y + 24, studio.channels.cdrom_install_enabled, "Cdrom with NovaOS 2026 'Noble' - Release amd64 Binary-1");
    }

    fn render_tab_other_software(&mut self, studio: &NexusStudio, x: usize, y: usize, w: usize, h: usize) {
        self.buffer.draw_text(x + 20, y + 16, "Third-party software repositories & PPAs:", COLOR_TEXT_PRIMARY);

        let list_x = x + 20;
        let list_y = y + 38;
        let list_w = w - 40;
        let list_h = h - 90;
        self.buffer.fill_rect(list_x, list_y, list_w, list_h, 0xFFFAFAFA);
        self.buffer.draw_rect(list_x, list_y, list_w, list_h, COLOR_BORDER);

        let mut row_y = list_y + 8;
        for feed in &studio.feeds.feeds {
            let is_selected = studio.feeds.selected_feed_id.as_deref() == Some(&feed.id);
            if is_selected {
                self.buffer.fill_rect(list_x + 2, row_y - 4, list_w - 4, 34, COLOR_SELECTED_ROW);
            }
            self.buffer.draw_checkbox(list_x + 12, row_y, feed.enabled, &feed.name);
            self.buffer.draw_text(list_x + 36, row_y + 14, &feed.uri, COLOR_TEXT_MUTED);
            row_y += 38;
        }

        // Action Buttons Toolbar at bottom of tab
        let btn_y = y + h - 42;
        self.buffer.draw_button(x + 20, btn_y, 80, 28, "Add...", true);
        self.buffer.draw_button(x + 110, btn_y, 80, 28, "Edit...", false);
        self.buffer.draw_button(x + 200, btn_y, 80, 28, "Remove", false);
        self.buffer.draw_button(x + 290, btn_y, 120, 28, "Add Volume...", false);
    }

    fn render_tab_updates(&mut self, studio: &NexusStudio, x: usize, y: usize, _w: usize, _h: usize) {
        let mut cur_y = y + 20;

        self.buffer.draw_text(x + 20, cur_y + 6, "Automatically check for updates:", COLOR_TEXT_PRIMARY);
        self.draw_dropdown_preview(x + 320, cur_y, 220, studio.policy.check_frequency.as_str());
        cur_y += 38;

        self.buffer.draw_text(x + 20, cur_y + 6, "When there are security updates:", COLOR_TEXT_PRIMARY);
        self.draw_dropdown_preview(x + 320, cur_y, 300, studio.policy.security_updates.as_str());
        cur_y += 38;

        self.buffer.draw_text(x + 20, cur_y + 6, "When there are other updates:", COLOR_TEXT_PRIMARY);
        self.draw_dropdown_preview(x + 320, cur_y, 220, studio.policy.other_updates.as_str());
        cur_y += 38;

        self.buffer.draw_text(x + 20, cur_y + 6, "Notify me of a new Ubuntu version:", COLOR_TEXT_PRIMARY);
        self.draw_dropdown_preview(x + 320, cur_y, 300, studio.policy.release_upgrade.as_str());
        cur_y += 46;

        // Ubuntu Pro / NovaOS Enterprise Status Box (matches Screenshot 18-53-18)
        self.buffer.fill_rect(x + 20, cur_y, 600, 72, 0xFFF5F5F5);
        self.buffer.draw_rect(x + 20, cur_y, 600, 72, COLOR_BORDER);
        self.buffer.draw_text(x + 32, cur_y + 14, "NovaOS Pro & ESM Support", COLOR_TEXT_PRIMARY);
        self.buffer.draw_text(x + 32, cur_y + 32, &format!("Status: Active until {}", studio.policy.esm_pro_expiry), COLOR_SUCCESS_GREEN);
        self.buffer.draw_text(x + 32, cur_y + 48, "Expanded Security Maintenance (ESM) covers all 23,000 packages.", COLOR_TEXT_MUTED);
    }

    fn render_tab_authentication(&mut self, studio: &NexusStudio, x: usize, y: usize, w: usize, h: usize) {
        self.buffer.draw_text(x + 20, y + 16, "Trusted software providers and signing keys:", COLOR_TEXT_PRIMARY);

        let table_x = x + 20;
        let table_y = y + 38;
        let table_w = w - 40;
        let table_h = h - 90;
        self.buffer.fill_rect(table_x, table_y, table_w, table_h, 0xFFFFFFFF);
        self.buffer.draw_rect(table_x, table_y, table_w, table_h, COLOR_BORDER);

        let mut row_y = table_y + 8;
        for key in &studio.auth_keys.keys {
            let is_sel = studio.auth_keys.selected_key_id.as_deref() == Some(&key.key_id);
            if is_sel {
                self.buffer.fill_rect(table_x + 2, row_y - 4, table_w - 4, 38, COLOR_SELECTED_ROW);
            }
            self.buffer.draw_text(table_x + 12, row_y, &format!("{} [{}]", key.owner_name, key.key_id), COLOR_TEXT_PRIMARY);
            self.buffer.draw_text(table_x + 12, row_y + 16, &format!("Fingerprint: {}", key.fingerprint), COLOR_TEXT_MUTED);
            row_y += 42;
        }

        let btn_y = y + h - 42;
        self.buffer.draw_button(x + 20, btn_y, 160, 28, "Import Key File...", true);
        self.buffer.draw_button(x + 190, btn_y, 100, 28, "Remove", false);
        self.buffer.draw_button(x + 300, btn_y, 140, 28, "Restore Defaults", false);
    }

    fn render_tab_additional_drivers(&mut self, studio: &NexusStudio, x: usize, y: usize, _w: usize, h: usize) {
        self.buffer.draw_text(x + 20, y + 16, "Available proprietary and alternative hardware drivers:", COLOR_TEXT_PRIMARY);

        let mut row_y = y + 42;
        for dev in &studio.drivers.devices {
            self.buffer.draw_text(x + 20, row_y, &format!("{} - {}", dev.vendor, dev.model), 0xFF0D47A1);
            row_y += 18;
            for drv in &dev.drivers {
                let is_checked = drv.active || dev.pending_driver_id.as_deref() == Some(&drv.id);
                self.buffer.draw_radio(x + 32, row_y, is_checked, &drv.title);
                row_y += 22;
            }
            row_y += 10;
        }

        let btn_y = y + h - 42;
        self.buffer.draw_button(x + 20, btn_y, 130, 28, "Apply Changes", true);
        self.buffer.draw_button(x + 160, btn_y, 90, 28, "Revert", false);
    }

    fn render_tab_developer_options(&mut self, studio: &NexusStudio, x: usize, y: usize, _w: usize, _h: usize) {
        self.buffer.draw_text(x + 20, y + 20, "Developer pre-release and proposed package testing:", COLOR_TEXT_PRIMARY);
        self.buffer.draw_checkbox(x + 24, y + 50, studio.developer.proposed_channel_enabled, &studio.developer.proposed_channel_name);
        self.buffer.draw_text(x + 44, y + 74, "Warning: Proposed packages are under active development and may cause system instability.", COLOR_WARNING_AMBER);

        self.buffer.draw_checkbox(x + 24, y + 110, studio.developer.debug_logging_enabled, "Enable verbose transaction logs for package operations");
    }

    fn render_modal_mirror_selection(&mut self, studio: &NexusStudio) {
        let mw = 680;
        let mh = 480;
        let mx = (self.buffer.width - mw) / 2;
        let my = (self.buffer.height - mh) / 2;

        self.buffer.fill_rect(mx, my, mw, mh, COLOR_PANEL_BG);
        self.buffer.draw_rect(mx, my, mw, mh, COLOR_HEADER_BG);

        self.buffer.fill_rect(mx, my, mw, 36, COLOR_HEADER_BG);
        self.buffer.draw_text(mx + 16, my + 12, "Choose a Download Server", COLOR_HEADER_TEXT);

        // Mirror list tree
        let tree_x = mx + 20;
        let tree_y = my + 50;
        let tree_w = mw - 40;
        let tree_h = mh - 130;
        self.buffer.fill_rect(tree_x, tree_y, tree_w, tree_h, 0xFFFAFAFA);
        self.buffer.draw_rect(tree_x, tree_y, tree_w, tree_h, COLOR_BORDER);

        let mut row_y = tree_y + 8;
        for mirror in &studio.mirrors.mirrors {
            let is_sel = studio.mirrors.selected_mirror_id == mirror.id;
            if is_sel {
                self.buffer.fill_rect(tree_x + 2, row_y - 2, tree_w - 4, 22, COLOR_SELECTED_ROW);
            }
            self.buffer.draw_text(tree_x + 10, row_y + 4, &format!("[{}] {}", mirror.country, mirror.name), COLOR_TEXT_PRIMARY);
            if let Some(latency) = mirror.latency_ms {
                let (color, tag) = match mirror.status {
                    MirrorStatus::Fast => (COLOR_SUCCESS_GREEN, "Fast"),
                    MirrorStatus::Normal => (COLOR_ACCENT_UBUNTU, "Good"),
                    MirrorStatus::Slow => (COLOR_WARNING_AMBER, "Slow"),
                    _ => (COLOR_DANGER_RED, "Down"),
                };
                self.buffer.draw_text(tree_x + tree_w - 120, row_y + 4, &format!("{}ms ({})", latency, tag), color);
            }
            row_y += 24;
            if row_y + 24 > tree_y + tree_h {
                break;
            }
        }

        // Action Buttons inside Modal
        let btn_y = my + mh - 44;
        self.buffer.draw_button(mx + 20, btn_y, 170, 30, "Select Best Server", true);
        self.buffer.draw_button(mx + mw - 230, btn_y, 110, 30, "Choose Server", false);
        self.buffer.draw_button(mx + mw - 110, btn_y, 90, 30, "Cancel", false);
    }

    fn render_software_updater(&mut self, studio: &NexusStudio) {
        // Window Title Bar
        self.buffer.fill_rect(0, 0, self.buffer.width, 36, COLOR_HEADER_BG);
        self.buffer.draw_text(16, 12, "Software Updater", COLOR_HEADER_TEXT);

        let x = 20;
        let y = 50;

        self.buffer.draw_text(x, y, "Updated software has been issued since NovaOS was released.", COLOR_TEXT_PRIMARY);
        self.buffer.draw_text(x, y + 18, "Do you want to install it now?", COLOR_TEXT_PRIMARY);

        self.buffer.draw_text(x, y + 44, &studio.updater.total_download_display(), COLOR_TEXT_MUTED);

        // Package Tree Box
        let box_x = x;
        let box_y = y + 70;
        let box_w = self.buffer.width - 40;
        let box_h = self.buffer.height - 210;
        self.buffer.fill_rect(box_x, box_y, box_w, box_h, 0xFFFFFFFF);
        self.buffer.draw_rect(box_x, box_y, box_w, box_h, COLOR_BORDER);

        let mut row_y = box_y + 8;
        for cat in studio.updater.categories() {
            let pkgs = studio.updater.packages_in_category(&cat);
            if pkgs.is_empty() {
                continue;
            }
            self.buffer.draw_text(box_x + 10, row_y, &format!("> {}", cat.as_str()), 0xFF0D47A1);
            row_y += 20;

            for pkg in pkgs {
                let is_sel = studio.updater.selected_package_id.as_deref() == Some(&pkg.id);
                if is_sel {
                    self.buffer.fill_rect(box_x + 2, row_y - 2, box_w - 4, 22, COLOR_SELECTED_ROW);
                }
                self.buffer.draw_checkbox(box_x + 24, row_y, pkg.selected, &pkg.name);
                let size_mb = pkg.size_bytes as f64 / (1024.0 * 1024.0);
                self.buffer.draw_text(box_x + box_w - 90, row_y + 3, &format!("{:.1} MB", size_mb), COLOR_TEXT_MUTED);
                row_y += 22;
            }
        }

        // Selected package changelog preview box at bottom
        let desc_y = box_y + box_h + 10;
        if let Some(ref sel_id) = studio.updater.selected_package_id {
            if let Some(pkg) = studio.updater.pending_packages.iter().find(|p| &p.id == sel_id) {
                self.buffer.draw_text(x, desc_y, &format!("Details: {} ({} -> {})", pkg.name, pkg.current_version, pkg.new_version), COLOR_TEXT_PRIMARY);
                self.buffer.draw_text(x, desc_y + 16, &pkg.summary, COLOR_TEXT_MUTED);
            }
        }

        // Progress bar if active update stage
        if let UpdateStage::DownloadingPackages { progress_pct, speed_mbps, .. } = studio.updater.current_stage {
            self.buffer.draw_progress_bar(x, desc_y + 36, box_w, 14, progress_pct);
            self.buffer.draw_text(x, desc_y + 54, &format!("Downloading updates: {}% ({:.1} MB/s)", progress_pct, speed_mbps), COLOR_TEXT_PRIMARY);
        }

        // Bottom action buttons
        let btn_y = self.buffer.height - 44;
        self.buffer.draw_button(x, btn_y, 110, 30, "Settings...", false);
        self.buffer.draw_button(self.buffer.width - 290, btn_y, 140, 30, "Remind Me Later", false);
        self.buffer.draw_button(self.buffer.width - 140, btn_y, 120, 30, "Install Now", true);
    }

    fn draw_dropdown_preview(&mut self, x: usize, y: usize, w: usize, text: &str) {
        self.buffer.fill_rect(x, y, w, 26, 0xFFFAFAFA);
        self.buffer.draw_rect(x, y, w, 26, COLOR_BORDER);
        self.buffer.draw_text(x + 8, y + 8, text, COLOR_TEXT_PRIMARY);
        self.buffer.draw_text(x + w - 18, y + 8, "v", COLOR_TEXT_MUTED);
    }
}

fn get_glyph_bitmap(c: char) -> [u8; 8] {
    match c {
        'A' => [0x18, 0x24, 0x42, 0x7E, 0x42, 0x42, 0x42, 0x00],
        'B' => [0x7C, 0x42, 0x7C, 0x42, 0x42, 0x7C, 0x00, 0x00],
        'C' => [0x3C, 0x42, 0x40, 0x40, 0x42, 0x3C, 0x00, 0x00],
        'D' => [0x78, 0x44, 0x42, 0x42, 0x44, 0x78, 0x00, 0x00],
        'E' => [0x7E, 0x40, 0x78, 0x40, 0x40, 0x7E, 0x00, 0x00],
        'F' => [0x7E, 0x40, 0x78, 0x40, 0x40, 0x40, 0x00, 0x00],
        'G' => [0x3C, 0x42, 0x40, 0x4E, 0x42, 0x3C, 0x00, 0x00],
        'H' => [0x42, 0x42, 0x7E, 0x42, 0x42, 0x42, 0x00, 0x00],
        'I' => [0x38, 0x10, 0x10, 0x10, 0x10, 0x38, 0x00, 0x00],
        'J' => [0x1C, 0x08, 0x08, 0x08, 0x48, 0x30, 0x00, 0x00],
        'K' => [0x44, 0x48, 0x70, 0x48, 0x44, 0x42, 0x00, 0x00],
        'L' => [0x40, 0x40, 0x40, 0x40, 0x40, 0x7E, 0x00, 0x00],
        'M' => [0x66, 0x7E, 0x5A, 0x42, 0x42, 0x42, 0x00, 0x00],
        'N' => [0x42, 0x62, 0x52, 0x4A, 0x46, 0x42, 0x00, 0x00],
        'O' => [0x3C, 0x42, 0x42, 0x42, 0x42, 0x3C, 0x00, 0x00],
        'P' => [0x7C, 0x42, 0x7C, 0x40, 0x40, 0x40, 0x00, 0x00],
        'Q' => [0x3C, 0x42, 0x42, 0x52, 0x4A, 0x3C, 0x02, 0x00],
        'R' => [0x7C, 0x42, 0x7C, 0x48, 0x44, 0x42, 0x00, 0x00],
        'S' => [0x3C, 0x40, 0x38, 0x04, 0x42, 0x3C, 0x00, 0x00],
        'T' => [0x7E, 0x18, 0x18, 0x18, 0x18, 0x18, 0x00, 0x00],
        'U' => [0x42, 0x42, 0x42, 0x42, 0x42, 0x3C, 0x00, 0x00],
        'V' => [0x42, 0x42, 0x42, 0x24, 0x24, 0x18, 0x00, 0x00],
        'W' => [0x42, 0x42, 0x42, 0x5A, 0x7E, 0x66, 0x00, 0x00],
        'X' => [0x42, 0x24, 0x18, 0x18, 0x24, 0x42, 0x00, 0x00],
        'Y' => [0x42, 0x24, 0x18, 0x18, 0x18, 0x18, 0x00, 0x00],
        'Z' => [0x7E, 0x04, 0x08, 0x10, 0x20, 0x7E, 0x00, 0x00],
        'a' => [0x00, 0x38, 0x04, 0x3C, 0x44, 0x3A, 0x00, 0x00],
        'b' => [0x40, 0x58, 0x64, 0x42, 0x42, 0x7C, 0x00, 0x00],
        'c' => [0x00, 0x3C, 0x42, 0x40, 0x42, 0x3C, 0x00, 0x00],
        'd' => [0x04, 0x1C, 0x24, 0x44, 0x44, 0x3A, 0x00, 0x00],
        'e' => [0x00, 0x38, 0x44, 0x7C, 0x40, 0x3C, 0x00, 0x00],
        'f' => [0x18, 0x24, 0x20, 0x70, 0x20, 0x20, 0x00, 0x00],
        'g' => [0x00, 0x3A, 0x44, 0x44, 0x3C, 0x04, 0x38, 0x00],
        'h' => [0x40, 0x58, 0x64, 0x42, 0x42, 0x42, 0x00, 0x00],
        'i' => [0x10, 0x00, 0x30, 0x10, 0x10, 0x38, 0x00, 0x00],
        'j' => [0x08, 0x00, 0x18, 0x08, 0x08, 0x48, 0x30, 0x00],
        'k' => [0x40, 0x48, 0x50, 0x60, 0x50, 0x48, 0x00, 0x00],
        'l' => [0x30, 0x10, 0x10, 0x10, 0x10, 0x38, 0x00, 0x00],
        'm' => [0x00, 0x6A, 0x7F, 0x49, 0x49, 0x49, 0x00, 0x00],
        'n' => [0x00, 0x58, 0x64, 0x42, 0x42, 0x42, 0x00, 0x00],
        'o' => [0x00, 0x38, 0x44, 0x44, 0x44, 0x38, 0x00, 0x00],
        'p' => [0x00, 0x58, 0x64, 0x42, 0x7C, 0x40, 0x40, 0x00],
        'q' => [0x00, 0x3A, 0x44, 0x44, 0x3A, 0x02, 0x02, 0x00],
        'r' => [0x00, 0x5C, 0x62, 0x40, 0x40, 0x40, 0x00, 0x00],
        's' => [0x00, 0x3C, 0x40, 0x38, 0x04, 0x78, 0x00, 0x00],
        't' => [0x20, 0x70, 0x20, 0x20, 0x24, 0x18, 0x00, 0x00],
        'u' => [0x00, 0x42, 0x42, 0x42, 0x46, 0x3A, 0x00, 0x00],
        'v' => [0x00, 0x42, 0x42, 0x24, 0x24, 0x18, 0x00, 0x00],
        'w' => [0x00, 0x42, 0x42, 0x5A, 0x7E, 0x24, 0x00, 0x00],
        'x' => [0x00, 0x42, 0x24, 0x18, 0x24, 0x42, 0x00, 0x00],
        'y' => [0x00, 0x42, 0x44, 0x38, 0x08, 0x30, 0x00, 0x00],
        'z' => [0x00, 0x7E, 0x08, 0x10, 0x20, 0x7E, 0x00, 0x00],
        '0' => [0x3C, 0x42, 0x46, 0x52, 0x62, 0x3C, 0x00, 0x00],
        '1' => [0x18, 0x28, 0x08, 0x08, 0x08, 0x3E, 0x00, 0x00],
        '2' => [0x3C, 0x42, 0x04, 0x18, 0x20, 0x7E, 0x00, 0x00],
        '3' => [0x3C, 0x42, 0x04, 0x18, 0x02, 0x3C, 0x00, 0x00],
        '4' => [0x08, 0x18, 0x28, 0x48, 0x7E, 0x08, 0x00, 0x00],
        '5' => [0x7E, 0x40, 0x7C, 0x02, 0x42, 0x3C, 0x00, 0x00],
        '6' => [0x3C, 0x40, 0x7C, 0x42, 0x42, 0x3C, 0x00, 0x00],
        '7' => [0x7E, 0x02, 0x04, 0x08, 0x10, 0x20, 0x00, 0x00],
        '8' => [0x3C, 0x42, 0x3C, 0x42, 0x42, 0x3C, 0x00, 0x00],
        '9' => [0x3C, 0x42, 0x42, 0x3E, 0x02, 0x3C, 0x00, 0x00],
        '.' => [0x00, 0x00, 0x00, 0x00, 0x00, 0x18, 0x18, 0x00],
        ',' => [0x00, 0x00, 0x00, 0x00, 0x00, 0x18, 0x18, 0x30],
        ':' => [0x00, 0x18, 0x18, 0x00, 0x18, 0x18, 0x00, 0x00],
        ';' => [0x00, 0x18, 0x18, 0x00, 0x18, 0x18, 0x30, 0x00],
        '/' => [0x02, 0x04, 0x08, 0x10, 0x20, 0x40, 0x00, 0x00],
        '-' => [0x00, 0x00, 0x7E, 0x00, 0x00, 0x00, 0x00, 0x00],
        '_' => [0x00, 0x00, 0x00, 0x00, 0x00, 0xFF, 0x00, 0x00],
        '(' => [0x0C, 0x18, 0x30, 0x30, 0x18, 0x0C, 0x00, 0x00],
        ')' => [0x30, 0x18, 0x0C, 0x0C, 0x18, 0x30, 0x00, 0x00],
        '[' => [0x3C, 0x30, 0x30, 0x30, 0x30, 0x3C, 0x00, 0x00],
        ']' => [0x3C, 0x0C, 0x0C, 0x0C, 0x0C, 0x3C, 0x00, 0x00],
        '>' => [0x20, 0x10, 0x08, 0x04, 0x08, 0x10, 0x20, 0x00],
        '%' => [0x62, 0x64, 0x08, 0x10, 0x26, 0x46, 0x00, 0x00],
        '?' => [0x3C, 0x42, 0x04, 0x08, 0x00, 0x08, 0x00, 0x00],
        '!' => [0x18, 0x18, 0x18, 0x18, 0x00, 0x18, 0x00, 0x00],
        _ => [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_renderer_initialization_and_rasterization() {
        let mut renderer = NexusRenderer::new(800, 600);
        let mut studio = NexusStudio::new();

        // Render Software & Updates mode
        renderer.render(&studio);
        assert_eq!(renderer.buffer.pixels.len(), 800 * 600);

        // Render Software Updater mode
        studio.set_mode(NexusWindowMode::SoftwareUpdater);
        renderer.render(&studio);
        assert_eq!(renderer.buffer.pixels.len(), 800 * 600);

        // Render Modal Mirror Selection
        studio.set_tab(NexusActiveTab::MirrorSpeedTest);
        studio.set_mode(NexusWindowMode::SoftwareAndUpdates);
        renderer.render(&studio);
    }
}
