// nexus-core: feed.rs
// Other Software: 3rd-party repositories, PPAs, external deb/rpm feeds

#[derive(Debug, Clone)]
pub struct PpaFeed {
    pub id: String,
    pub name: String,
    pub uri: String,
    pub distribution: String,
    pub components: Vec<String>,
    pub enabled: bool,
    pub is_source: bool,
    pub comment: String,
}

impl PpaFeed {
    pub fn new(
        id: &str,
        name: &str,
        uri: &str,
        distribution: &str,
        components: &[&str],
        enabled: bool,
        is_source: bool,
        comment: &str,
    ) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            uri: uri.to_string(),
            distribution: distribution.to_string(),
            components: components.iter().map(|s| s.to_string()).collect(),
            enabled,
            is_source,
            comment: comment.to_string(),
        }
    }

    pub fn to_deb_line(&self) -> String {
        let prefix = if self.is_source { "deb-src" } else { "deb" };
        format!(
            "{} {} {} {}",
            prefix,
            self.uri,
            self.distribution,
            self.components.join(" ")
        )
    }
}

#[derive(Debug, Clone)]
pub struct FeedManager {
    pub feeds: Vec<PpaFeed>,
    pub selected_feed_id: Option<String>,
}

impl Default for FeedManager {
    fn default() -> Self {
        Self::new()
    }
}

impl FeedManager {
    pub fn new() -> Self {
        let feeds = vec![
            PpaFeed::new(
                "brave",
                "Brave Browser Release Channel",
                "https://brave-browser-apt-release.s3.brave.com/",
                "stable",
                &["main"],
                true,
                false,
                "Official Brave Software APT repository",
            ),
            PpaFeed::new(
                "docker",
                "Docker Community Edition",
                "https://download.docker.com/linux/ubuntu",
                "noble",
                &["stable"],
                true,
                false,
                "Official Docker CE binary packages",
            ),
            PpaFeed::new(
                "vscode",
                "Microsoft Visual Studio Code",
                "https://packages.microsoft.com/repos/code",
                "stable",
                &["main"],
                true,
                false,
                "Visual Studio Code binaries and updates",
            ),
            PpaFeed::new(
                "chrome",
                "Google Chrome",
                "https://dl.google.com/linux/chrome/deb/",
                "stable",
                &["main"],
                true,
                false,
                "Google Chrome Linux web browser updates",
            ),
            PpaFeed::new(
                "novaos-universe",
                "NovaOS Community Extras & Ports",
                "https://pkg.novaos.org/universe",
                "nova-1.0",
                &["universe", "multiverse"],
                true,
                false,
                "NovaOS community ports, toolchains and sovereign applications",
            ),
        ];

        Self {
            feeds,
            selected_feed_id: Some("brave".to_string()),
        }
    }

    pub fn add_feed_from_line(&mut self, line: &str) -> Result<String, String> {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            return Err("APT line cannot be empty".to_string());
        }

        // Support PPA format: ppa:user/repo
        if trimmed.starts_with("ppa:") {
            let ppa_path = &trimmed[4..];
            let id = format!("ppa-{}", ppa_path.replace('/', "-"));
            let uri = format!("https://ppa.launchpadcontent.net/{}/ubuntu", ppa_path);
            let feed = PpaFeed::new(
                &id,
                &format!("PPA: {}", ppa_path),
                &uri,
                "noble",
                &["main"],
                true,
                false,
                &format!("Launchpad PPA for {}", ppa_path),
            );
            self.feeds.push(feed);
            return Ok(id);
        }

        let parts: Vec<&str> = trimmed.split_whitespace().collect();
        if parts.len() < 3 {
            return Err("Invalid repository line syntax. Expected: deb [options] <uri> <distribution> <components...>".to_string());
        }

        let is_source = parts[0] == "deb-src";
        let mut idx = 1;
        if parts[idx].starts_with('[') {
            while idx < parts.len() && !parts[idx].ends_with(']') {
                idx += 1;
            }
            idx += 1;
        }

        if idx >= parts.len() {
            return Err("Missing repository URI in deb line".to_string());
        }

        let uri = parts[idx];
        let distribution = if idx + 1 < parts.len() { parts[idx + 1] } else { "stable" };
        let components = if idx + 2 < parts.len() { &parts[idx + 2..] } else { &["main"] };

        let host = uri
            .trim_start_matches("http://")
            .trim_start_matches("https://")
            .split('/')
            .next()
            .unwrap_or("repo");
        let id = format!("feed-{}-{}", host.replace('.', "-"), self.feeds.len() + 1);

        let feed = PpaFeed::new(
            &id,
            host,
            uri,
            distribution,
            components,
            true,
            is_source,
            &format!("Third-party repository from {}", host),
        );
        self.feeds.push(feed);
        Ok(id)
    }

    pub fn toggle_feed(&mut self, id: &str) -> bool {
        if let Some(f) = self.feeds.iter_mut().find(|f| f.id == id) {
            f.enabled = !f.enabled;
            true
        } else {
            false
        }
    }

    pub fn remove_feed(&mut self, id: &str) -> bool {
        let initial_len = self.feeds.len();
        self.feeds.retain(|f| f.id != id);
        if self.selected_feed_id.as_deref() == Some(id) {
            self.selected_feed_id = self.feeds.first().map(|f| f.id.clone());
        }
        self.feeds.len() < initial_len
    }

    pub fn export_all_sources(&self) -> String {
        let mut out = String::new();
        out.push_str("# Third-party software sources managed by Nexus\n");
        for feed in &self.feeds {
            if feed.enabled {
                out.push_str(&format!("# {}\n", feed.name));
                out.push_str(&format!("{}\n\n", feed.to_deb_line()));
            } else {
                out.push_str(&format!("# [DISABLED] {}\n", feed.name));
                out.push_str(&format!("# {}\n\n", feed.to_deb_line()));
            }
        }
        out
    }
}
