// nexus-storage: lib.rs
// Configuration persistence and disk state serialization for Nexus

use nexus_core::*;

pub struct StorageManager {
    pub base_dir: String,
}

impl StorageManager {
    pub fn new(base_dir: &str) -> Self {
        Self {
            base_dir: base_dir.to_string(),
        }
    }

    pub fn serialize_channels(&self, channels: &ChannelManager) -> String {
        let mut out = String::new();
        out.push_str("{\n  \"active_server\": \"");
        out.push_str(&channels.active_server_url);
        out.push_str("\",\n  \"channels\": [\n");
        for (i, ch) in channels.channels.iter().enumerate() {
            out.push_str("    {\"id\": \"");
            out.push_str(&ch.id);
            out.push_str("\", \"enabled\": ");
            out.push_str(if ch.enabled { "true" } else { "false" });
            out.push_str("}");
            if i + 1 < channels.channels.len() {
                out.push(',');
            }
            out.push('\n');
        }
        out.push_str("  ]\n}\n");
        out
    }

    pub fn serialize_feeds(&self, feeds: &FeedManager) -> String {
        feeds.export_all_sources()
    }

    pub fn serialize_policy(&self, policy: &UpdatePolicy) -> String {
        format!(
            "{{\n  \"check_frequency\": \"{}\",\n  \"security_updates\": \"{}\",\n  \"other_updates\": \"{}\",\n  \"release_upgrade\": \"{}\",\n  \"esm_pro_active\": {},\n  \"livepatch_enabled\": {}\n}}\n",
            policy.check_frequency.as_str(),
            policy.security_updates.as_str(),
            policy.other_updates.as_str(),
            policy.release_upgrade.as_str(),
            policy.esm_pro_active,
            policy.livepatch_enabled
        )
    }

    pub fn serialize_auth_keys(&self, keys: &AuthKeyStore) -> String {
        let mut out = String::new();
        out.push_str("[\n");
        for (i, k) in keys.keys.iter().enumerate() {
            out.push_str(&format!(
                "  {{\"key_id\": \"{}\", \"owner\": \"{}\", \"algorithm\": \"{}\", \"is_default\": {}}}",
                k.key_id, k.owner_name, k.algorithm.as_str(), k.is_default
            ));
            if i + 1 < keys.keys.len() {
                out.push(',');
            }
            out.push('\n');
        }
        out.push_str("]\n");
        out
    }

    pub fn serialize_studio_state(&self, studio: &NexusStudio) -> String {
        format!(
            "# Nexus Studio Configuration State\nactive_mode={:?}\nactive_tab={:?}\nactive_mirror={}\npending_updates={}\n",
            studio.mode,
            studio.active_tab,
            studio.mirrors.selected_mirror_id,
            studio.updater.count_selected()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_storage_serialization() {
        let storage = StorageManager::new("/tmp/nexus");
        let studio = NexusStudio::new();

        let channels_json = storage.serialize_channels(&studio.channels);
        assert!(channels_json.contains("active_server"));
        assert!(channels_json.contains("main"));

        let feeds_str = storage.serialize_feeds(&studio.feeds);
        assert!(feeds_str.contains("brave-browser"));

        let policy_str = storage.serialize_policy(&studio.policy);
        assert!(policy_str.contains("Daily"));

        let keys_str = storage.serialize_auth_keys(&studio.auth_keys);
        assert!(keys_str.contains("Ubuntu Archive"));

        let state_str = storage.serialize_studio_state(&studio);
        assert!(state_str.contains("SoftwareAndUpdates"));
    }
}
