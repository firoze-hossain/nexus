// nexus-core: auth.rs
// Authentication: Trusted GPG / OpenPGP / Ed25519 signing keys trust store

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KeyAlgorithm {
    Rsa4096,
    Rsa2048,
    Ed25519,
}

impl KeyAlgorithm {
    pub fn as_str(&self) -> &str {
        match self {
            KeyAlgorithm::Rsa4096 => "RSA 4096",
            KeyAlgorithm::Rsa2048 => "RSA 2048",
            KeyAlgorithm::Ed25519 => "Ed25519",
        }
    }
}

#[derive(Debug, Clone)]
pub struct SigningKey {
    pub key_id: String,
    pub fingerprint: String,
    pub owner_name: String,
    pub email: String,
    pub algorithm: KeyAlgorithm,
    pub created_date: String,
    pub expiry_date: Option<String>,
    pub is_default: bool,
}

impl SigningKey {
    pub fn new(
        key_id: &str,
        fingerprint: &str,
        owner_name: &str,
        email: &str,
        algorithm: KeyAlgorithm,
        created_date: &str,
        expiry_date: Option<&str>,
        is_default: bool,
    ) -> Self {
        Self {
            key_id: key_id.to_string(),
            fingerprint: fingerprint.to_string(),
            owner_name: owner_name.to_string(),
            email: email.to_string(),
            algorithm,
            created_date: created_date.to_string(),
            expiry_date: expiry_date.map(|s| s.to_string()),
            is_default,
        }
    }
}

#[derive(Debug, Clone)]
pub struct AuthKeyStore {
    pub keys: Vec<SigningKey>,
    pub selected_key_id: Option<String>,
}

impl Default for AuthKeyStore {
    fn default() -> Self {
        Self::new()
    }
}

impl AuthKeyStore {
    pub fn new() -> Self {
        let keys = Self::default_keys();
        let selected = keys.first().map(|k| k.key_id.clone());
        Self {
            keys,
            selected_key_id: selected,
        }
    }

    pub fn default_keys() -> Vec<SigningKey> {
        vec![
            SigningKey::new(
                "3B4FE6ACC0B21F32",
                "F6EC B376 2403 7A4F 6FB9 5264 3B4F E6AC C0B2 1F32",
                "Ubuntu Archive Automatic Signing Key (2018)",
                "ftpmaster@ubuntu.com",
                KeyAlgorithm::Rsa4096,
                "2018-09-17",
                None,
                true,
            ),
            SigningKey::new(
                "D94AA3F0EFE21092",
                "8439 38DF 228D 22F7 B374 2BC0 D94A A3F0 EFE2 1092",
                "Ubuntu CD Image Automatic Signing Key (2018)",
                "cdimage@ubuntu.com",
                KeyAlgorithm::Rsa4096,
                "2018-09-17",
                None,
                true,
            ),
            SigningKey::new(
                "NOVA-ROOT-ED25519-2026",
                "9A88 CC41 550E 1192 443B E812 7701 BC8A 0021 FA93",
                "NovaOS Official Master Signing Authority",
                "security@novaos.org",
                KeyAlgorithm::Ed25519,
                "2026-01-01",
                Some("2036-01-01"),
                true,
            ),
            SigningKey::new(
                "BRAVE-KEY-2024",
                "D615 5336 BA5C 0012 3341 88EF B012 CC49 5501 3321",
                "Brave Software Package Signing Key",
                "support@brave.com",
                KeyAlgorithm::Rsa4096,
                "2024-03-12",
                Some("2028-03-12"),
                false,
            ),
        ]
    }

    pub fn restore_defaults(&mut self) {
        self.keys = Self::default_keys();
        self.selected_key_id = self.keys.first().map(|k| k.key_id.clone());
    }

    pub fn import_key_file(&mut self, file_content: &str) -> Result<String, String> {
        let trimmed = file_content.trim();
        if trimmed.is_empty() {
            return Err("Key file content is empty".to_string());
        }

        // Generate synthetic or parsed key fingerprint based on content hash
        let hash = simple_fnv_hash(trimmed.as_bytes());
        let key_id = format!("{:016X}", hash);
        let hash2 = hash.wrapping_mul(31);
        let fingerprint = format!(
            "{:04X} {:04X} {:04X} {:04X} {:04X} {:04X} {:04X} {:04X}",
            (hash >> 48) & 0xFFFF,
            (hash >> 32) & 0xFFFF,
            (hash >> 16) & 0xFFFF,
            hash & 0xFFFF,
            (hash2 >> 48) & 0xFFFF,
            (hash2 >> 32) & 0xFFFF,
            (hash2 >> 16) & 0xFFFF,
            hash2 & 0xFFFF,
        );

        let owner = if trimmed.contains("Comment: ") {
            trimmed
                .lines()
                .find(|l| l.starts_with("Comment: "))
                .map(|l| l[9..].to_string())
                .unwrap_or_else(|| "Imported Key".to_string())
        } else {
            format!("Imported Third-Party Archive Key ({})", &key_id[0..8])
        };

        let new_key = SigningKey::new(
            &key_id,
            &fingerprint,
            &owner,
            "packages@external-mirror.org",
            KeyAlgorithm::Ed25519,
            "2026-11-25",
            None,
            false,
        );

        self.keys.push(new_key);
        self.selected_key_id = Some(key_id.clone());
        Ok(key_id)
    }

    pub fn remove_key(&mut self, key_id: &str) -> bool {
        let initial = self.keys.len();
        self.keys.retain(|k| k.key_id != key_id);
        if self.selected_key_id.as_deref() == Some(key_id) {
            self.selected_key_id = self.keys.first().map(|k| k.key_id.clone());
        }
        self.keys.len() < initial
    }
}

fn simple_fnv_hash(bytes: &[u8]) -> u64 {
    let mut hash: u64 = 0xcbf29ce484222325;
    for &b in bytes {
        hash ^= b as u64;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}
