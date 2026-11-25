# Nexus: Sovereign Software Sources, Mirror & Update Policy Studio

**Nexus** is a pure Rust, cross-platform software repository, mirror speed benchmark, cryptographic trust store, and system updater suite designed for **NovaOS**, **Linux (Ubuntu/Debian)**, **macOS**, and **Windows**.

Inspired by Ubuntu's `software-properties-gtk` ("Software & Updates") and `update-manager` ("Software Updater"), Nexus unifies repository feed orchestration, dynamic mirror latency testing, Ed25519/GPG authentication, hardware driver selection, automated update scheduling, and interactive system upgrades into a single high-performance, double-buffered vector studio.

---

## 🌟 Key Features

### 1. Ubuntu Software & Core Channels
- Toggles for official distribution channels: `main`, `universe`, `restricted`, `multiverse`, and `source code`.
- Download server dropdown selector (`Main Server`, `Regional Server`, `Other...`).
- CD-ROM / Volume offline installation toggle.
- Dynamic export to `/etc/apt/sources.list`.

### 2. Multi-Continental Mirror Benchmark & Geo-Routing
- Complete mirror database across North America, Europe, Asia, Africa, and Global CDN.
- **Dynamic "Select Best Server" Benchmark**: Measures real/simulated network latency (RTT) and throughput in milliseconds, ranks mirrors, and automatically chooses the lowest latency mirror.

### 3. Other Software (PPAs, External Feeds & deb822)
- Full CRUD operations for third-party package repositories (e.g., Brave Browser, Docker CE, Visual Studio Code, Google Chrome, NovaOS Universe).
- Supports Launchpad PPA syntax (`ppa:user/repo`) and deb lines (`deb [arch=amd64] https://... noble main`).
- Dynamic toggling and export.

### 4. Updates Schedule & Policies
- Configurable update check frequency: *Daily*, *Every two days*, *Weekly*, *Every two weeks*, *Never*.
- Unattended security updates policy: *Download and install automatically*, *Download automatically and notify*, *Display immediately*.
- Non-security updates frequency: *Display immediately*, *Display weekly*.
- LTS release upgrade notifications.
- NovaOS Pro / ESM (Expanded Security Maintenance) status tracking.

### 5. Authentication & Cryptographic Trust Store
- GPG & Ed25519 signing key store.
- Parses key IDs, owners, algorithms, creation/expiration dates, and generates 40-character cryptographic fingerprints.
- Import key files from ASCII armor, remove keys, and one-click *Restore Defaults*.

### 6. Additional Drivers & DKMS Hardware Management
- Scans system PCI/hardware devices (GPU, Wi-Fi 802.11ac, CPU microcode firmware).
- Select between proprietary, open-source, or disabled device states.
- Multi-device batch apply and revert workflow.

### 7. Developer Options
- Pre-release proposed channel toggle (`noble-proposed` / `novaos-proposed`).
- Debug logging and custom staging repository URL.

### 8. Software Updater Interactive Engine (2nd Mode)
- Dynamic package upgrade manifest inspection.
- Real-time download size calculation (`191.9 MB will be downloaded.`).
- Category tree: Security Updates, NovaOS Core & Base System, Applications, Shared Libraries.
- Multi-stage transaction pipeline:
  1. Checking updates
  2. Resolving dependencies
  3. Downloading packages (with speed MB/s and progress bar)
  4. Verifying Ed25519 cryptographic signatures
  5. Unpacking and configuring packages
  6. Finalizing and cleaning cache
- Mode-switching between Software & Updates and Software Updater via the `Settings...` button.

---

## 🏗️ Workspace Architecture

```
nexus/
├── Cargo.toml
├── crates/
│   ├── nexus-core/       # Channels, mirrors, feeds, policy, keys, drivers, and updater engine
│   ├── nexus-storage/    # Disk persistence and serialization
│   ├── nexus-render/     # Pure 2D software double-buffered vector rasterizer
│   ├── nexus-ui/         # Interactive state machine and event dispatcher
│   └── nexus-app/        # Cross-platform CLI runner and qualification test suite
└── novaos/               # Ring-3 bare-metal ELF runner for NovaOS
```

---

## 🚀 CLI Usage

```bash
# Check for pending software updates
cargo run -p nexus-app -- --check

# Run latency benchmark across global mirrors
cargo run -p nexus-app -- --benchmark

# Display active update policies
cargo run -p nexus-app -- --policy

# List detected hardware drivers
cargo run -p nexus-app -- --drivers

# List trusted cryptographic signing keys
cargo run -p nexus-app -- --keys

# Run autonomous 10/10 qualification test suite
cargo run -p nexus-app -- --test-full
```

---

## 🛡️ License
Licensed under the Apache-2.0 / MIT license. Developed for NovaOS.
