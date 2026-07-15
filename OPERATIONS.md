# Operations & Configuration Guide

## Overview

Aphelion-rs is designed to be maximally configurable while remaining intuitive and straightforward. All configuration is accessible through:
- **TUI** (Terminal User Interface) - Interactive visual configuration
- **CLI** (Command Line Interface) - Scriptable command-line options
- **API** (Programmatic Interface) - Integration with other tools
- **Config File** (TOML/YAML) - Persistent configuration

---

## Network Operation Modes

### Network Security Profiles

Different network protocols carry different risk levels. Aphelion-rs categorizes network operations by risk and allows fine-grained control.

#### Default Mode: HTTPS Only (Secure)
**Risk Level**: Low  
**Protocol**: HTTPS (TLS 1.2+)  
**Use Case**: Standard operations, production environments

```toml
[network]
mode = "https_only"
tls_version = "1.2+"
verify_certificates = true
certificate_pinning = true
```

**Characteristics**:
- Encrypted transport
- Certificate validation
- Man-in-the-middle protection
- Default for Tier 0-1 packages

---

#### HTTP Mode (Risky)
**Risk Level**: Medium  
**Protocol**: HTTP (unencrypted)  
**Use Case**: Legacy mirrors, internal networks

```toml
[network]
mode = "http_allowed"
require_checksums = true  # Mandatory for HTTP
warn_insecure = true
```

**Characteristics**:
- Unencrypted transport (passive eavesdropping possible)
- Mandatory checksum verification
- Warning displayed to user
- Requires explicit user approval
- Only allowed for Tier 0 (official repos)

**Security Measures**:
- Post-download checksum verification
- Signature verification required
- Traffic logged for audit
- User explicitly warned

---

#### Peer-to-Peer Mode (High Risk)
**Risk Level**: High  
**Protocol**: P2P (BitTorrent, IPFS, etc.)  
**Use Case**: Distributed package distribution, bandwidth optimization

```toml
[network]
mode = "p2p_enabled"
p2p_protocol = "bittorrent"  # or "ipfs"
max_peers = 50
seed_after_download = true
trust_p2p = false  # Always verify via checksums
```

**Characteristics**:
- Distributed download
- No central authority
- Faster for popular packages
- Higher risk of malicious peers

**Security Measures**:
- Mandatory cryptographic verification
- Checksum validation per chunk
- Signature verification required
- Malicious peer detection and blacklisting
- Only allowed for packages with strong signatures
- Not allowed for Tier 2-4 packages (too risky)

---

#### Custom Network Providers
**Risk Level**: Variable  
**Protocol**: Plugin-based  
**Use Case**: Custom corporate mirrors, specialized CDNs

```toml
[network.providers]
enabled = ["aws-s3", "gcs", "azure-blob", "custom-mirror"]

[[network.providers.custom]]
name = "corporate-mirror"
protocol = "https"
base_url = "https://mirror.corp.internal"
verify_ssl = true
priority = 1
```

---

## Intelligent Resource Management

### Dynamic Resource Detection

Aphelion-rs automatically detects and requests resources from the host to optimize performance without causing OOM (Out of Memory) or blocking issues.

#### Resource Detection Strategy

```rust
// Pseudo-code for resource detection
fn detect_available_resources() -> ResourceProfile {
    let total_memory = system.total_memory();
    let available_memory = system.available_memory();
    let used_memory = total_memory - available_memory;
    
    let total_cpu = system.cpu_count();
    let cpu_usage = system.cpu_usage();
    let available_cpu = total_cpu * (1.0 - cpu_usage);
    
    let disk_io_bandwidth = system.disk_io_available();
    let network_bandwidth = system.network_bandwidth_available();
    
    ResourceProfile {
        memory: calculate_safe_allocation(available_memory),
        cpu: calculate_safe_cpu_cores(available_cpu),
        disk_io: calculate_safe_io_rate(disk_io_bandwidth),
        network: calculate_safe_network_rate(network_bandwidth),
    }
}
```

#### Resource Reservation Configuration

```toml
[resources]
# Resource reservation mode
mode = "adaptive"  # or "conservative", "aggressive", "manual"

# Memory management
[resources.memory]
strategy = "adaptive"
min_reserve = "512MB"      # Minimum to reserve
max_reserve = "4GB"        # Maximum to use
target_utilization = 0.7   # Use up to 70% of available
oom_prevention = true      # Never allocate beyond safe threshold
spill_to_disk = true       # Use disk swap if needed

# CPU management
[resources.cpu]
strategy = "adaptive"
min_cores = 1
max_cores = "auto"         # All available cores
target_utilization = 0.8   # Use up to 80% of available CPU
priority = "normal"        # or "low", "high"
affinity = "auto"          # CPU affinity for containers

# Disk I/O management
[resources.disk]
strategy = "adaptive"
io_priority = "normal"     # ionice priority
max_iops = "auto"          # Auto-detect safe IOPS
buffer_size = "64MB"       # Write buffer size
use_direct_io = false      # Direct I/O bypass page cache
async_io = true            # Non-blocking I/O

# Network bandwidth
[resources.network]
strategy = "adaptive"
max_download_bandwidth = "auto"  # Auto-detect or "10MB/s"
max_upload_bandwidth = "auto"
connection_pooling = true
keep_alive = true
```

#### OOM Prevention

**Strategy 1: Pre-allocation Validation**
```
Before Operation:
1. Detect available memory
2. Estimate operation memory requirement
3. Check: required < (available * safety_factor)
4. If insufficient, wait or spill to disk
```

**Strategy 2: Memory Pressure Monitoring**
```
During Operation:
1. Monitor memory pressure in real-time
2. If pressure > threshold:
   - Pause non-critical operations
   - Flush buffers to disk
   - Release caches
   - Wait for memory to free
3. Resume when pressure decreases
```

**Strategy 3: Graceful Degradation**
```
On High Memory Pressure:
1. Reduce parallelism (fewer concurrent downloads)
2. Smaller buffer sizes
3. More aggressive disk caching
4. Slower operation but no OOM
```

---

## Non-Blocking I/O Architecture

### Intelligent I/O Operations

All I/O operations use non-blocking, asynchronous patterns to maximize throughput without blocking.

#### Async I/O Strategy

```toml
[io]
# I/O engine
engine = "tokio"           # Tokio async runtime
io_uring = true            # Use io_uring if available (Linux 5.1+)
thread_pool_size = "auto"  # Auto-size based on cores

# Write strategies
[io.write]
strategy = "write_ahead"   # Write-ahead logging pattern
buffer_size = "64MB"       # Write buffer
fsync_mode = "adaptive"    # "always", "never", "adaptive"
batch_writes = true        # Batch small writes
coalesce_writes = true     # Coalesce adjacent writes

# Read strategies
[io.read]
strategy = "read_ahead"    # Predictive read-ahead
buffer_size = "64MB"       # Read buffer
prefetch = true            # Prefetch likely reads
cache_size = "256MB"       # In-memory cache

# Concurrent operations
[io.concurrency]
max_concurrent_downloads = "auto"  # Based on bandwidth + memory
max_concurrent_installs = "auto"   # Based on I/O capacity
max_concurrent_builds = "auto"     # Based on CPU cores
queue_depth = 128          # I/O queue depth
```

#### Non-Blocking Write Pattern

```
Download → Memory Buffer → Write Queue → Disk (async)
                              ↓
                         Never blocks main thread
                              ↓
                         Background flush thread
```

**Benefits**:
- Main operations never block on I/O
- Parallel downloads/installs
- Optimal disk utilization
- No write stalls

---

## Mirror Management & Optimization

### Intelligent Mirror Selection

#### Mirror Discovery

```toml
[mirrors]
# Automatic mirror discovery
auto_discover = true
discovery_sources = ["mirror_list", "geo_ip", "cdn", "community"]

# Mirror fetching strategy
fetch_strategy = "parallel_with_fallback"
parallel_mirrors = 3       # Download from 3 mirrors simultaneously
chunk_size = "1MB"         # Chunk size for parallel downloads
fastest_mirror_wins = true # Use chunks from fastest mirror

# Mirror health monitoring
health_check_interval = "5min"
health_check_timeout = "3s"
auto_disable_slow = true
slow_threshold = "1MB/s"   # Below this = "slow"

# Geographic preferences
geo_preference = "auto"    # Auto-detect based on IP
preferred_regions = ["us", "eu"]
blacklisted_regions = []
```

#### Parallel Mirror Strategy

**Multi-Mirror Download**:
```
Package Download:
├─ Mirror 1 (US East)  → Chunks 0-5
├─ Mirror 2 (EU West)  → Chunks 6-11
└─ Mirror 3 (Asia)     → Chunks 12-17

Assemble chunks as they arrive (fastest wins)
Verify checksums per chunk
```

**Load Balancing**:
- Distribute load across multiple mirrors
- Never overwhelm single mirror
- Respect mirror rate limits
- Automatic failover

#### Mirror Rate Limiting

```toml
[mirrors.rate_limiting]
# Per-mirror limits
max_concurrent_connections = 4
max_bandwidth_per_mirror = "5MB/s"
respect_mirror_limits = true
backoff_on_429 = true      # HTTP 429 Too Many Requests
backoff_multiplier = 2.0   # Exponential backoff

# Global limits
global_max_connections = 50
global_max_bandwidth = "auto"
```

#### Mirror Blacklisting/Whitelisting

```toml
[mirrors.filtering]
# Blacklist specific mirrors
blacklist = [
    "slow-mirror.example.com",
    "unreliable-mirror.org"
]

# Blacklist by region
blacklist_regions = ["cn", "ru"]  # Example: exclude specific regions

# Whitelist only specific mirrors
use_whitelist = false
whitelist = [
    "official-mirror.debian.org",
    "trusted-cdn.ubuntu.com"
]

# Whitelist by region (restrict to specific regions)
whitelist_regions = ["us", "eu", "au"]
```

**CLI Examples**:
```bash
# Blacklist a mirror
aphelion mirror blacklist slow-mirror.example.com

# Whitelist only specific regions
aphelion mirror whitelist-region us eu

# List mirror status
aphelion mirror list

# Test mirror speeds
aphelion mirror benchmark
```

---

## Security Profiles

Different environments have different security requirements. Aphelion-rs provides pre-configured security profiles.

### Personal Profile (Default)
```toml
[profile.personal]
network_mode = "https_only"
allow_http = false
allow_p2p = false
mirror_selection = "auto"
regions = "auto"
package_tiers = [0, 1, 2]  # Allow up to Tier 2
require_approval_tier3 = true
```

### Government Profile
```toml
[profile.government]
network_mode = "https_only"
allow_http = false
allow_p2p = false
mirror_selection = "whitelist"
whitelist_mirrors = ["gov-approved-mirror.gov"]
whitelist_regions = ["us"]
package_tiers = [0, 1]  # Only vetted packages
require_approval_tier1 = true
audit_all_operations = true
fips_mode = true  # FIPS 140-2 compliance
```

### Corporate Profile
```toml
[profile.corporate]
network_mode = "https_only"
allow_http = true  # Internal mirrors may use HTTP
allow_p2p = false
mirror_selection = "whitelist"
whitelist_mirrors = ["corporate-mirror.corp", "backup-mirror.corp"]
proxy = "http://proxy.corp:8080"
package_tiers = [0, 1, 2]
require_signatures = true
verify_against_internal_registry = true
```

### Development Profile
```toml
[profile.development]
network_mode = "https_only"
allow_http = true
allow_p2p = true  # Faster downloads
mirror_selection = "auto"
regions = "auto"
package_tiers = [0, 1, 2, 3]  # Allow alpha/beta
require_approval_tier3 = false  # Auto-approve for dev
parallel_downloads = 10
aggressive_caching = true
```

---

## Configuration Interface

### TUI Configuration

**Interactive Menu System**:
```
┌─ Aphelion Configuration ─────────────────────────┐
│                                                   │
│  Network Settings                           >     │
│  Mirror Management                          >     │
│  Resource Allocation                        >     │
│  Security Profile                           >     │
│  Package Tiers                              >     │
│  Performance Tuning                         >     │
│                                                   │
│  [Apply]  [Reset]  [Save]  [Cancel]              │
└───────────────────────────────────────────────────┘
```

**Network Settings Submenu**:
```
┌─ Network Settings ───────────────────────────────┐
│                                                   │
│  Network Mode:        [HTTPS Only ▼]             │
│    ○ HTTPS Only (Secure)                         │
│    ○ Allow HTTP (Risky)                          │
│    ○ Enable P2P (High Risk)                      │
│                                                   │
│  Max Bandwidth:       [Auto      ▼]              │
│  Parallel Downloads:  [5         ▲▼]             │
│  Connection Timeout:  [30s       ▲▼]             │
│                                                   │
│  [Back]                                    [Save] │
└───────────────────────────────────────────────────┘
```

**Mirror Management TUI**:
```
┌─ Mirror Management ──────────────────────────────┐
│                                                   │
│ ┌─ Active Mirrors ────────────────────────────┐  │
│ │ ✓ us.mirror.ubuntu.com      [10.2 MB/s]    │  │
│ │ ✓ eu.mirror.ubuntu.com      [8.5 MB/s]     │  │
│ │ ✗ slow.mirror.com           [0.1 MB/s]     │  │
│ └─────────────────────────────────────────────┘  │
│                                                   │
│  Region Filter:      [All ▼]                     │
│  Blacklisted:        slow.mirror.com             │
│                                                   │
│  [Benchmark] [Blacklist] [Whitelist] [Refresh]   │
└───────────────────────────────────────────────────┘
```

### CLI Configuration

**Comprehensive CLI Interface**:
```bash
# Network mode
aphelion config network-mode [https|http|p2p]

# Resource allocation
aphelion config resources --memory 4GB --cpu 4 --mode adaptive

# Mirror management
aphelion mirror add <url>
aphelion mirror remove <url>
aphelion mirror blacklist <url>
aphelion mirror whitelist <url>
aphelion mirror region --allow us,eu --deny cn,ru
aphelion mirror benchmark
aphelion mirror list

# Security profile
aphelion config profile [personal|government|corporate|development]

# Performance tuning
aphelion config performance --parallel-downloads 10 --buffer-size 64MB

# Show current config
aphelion config show

# Edit config file
aphelion config edit

# Validate config
aphelion config validate

# Export/import config
aphelion config export > config.toml
aphelion config import config.toml
```

### API Configuration

**Programmatic Configuration**:
```rust
use aphelion::{Config, NetworkMode, ResourceStrategy};

let mut config = Config::new();

// Network configuration
config.network()
    .mode(NetworkMode::HttpsOnly)
    .max_bandwidth("10MB/s")
    .parallel_downloads(5)
    .timeout(30);

// Resource configuration
config.resources()
    .strategy(ResourceStrategy::Adaptive)
    .memory_max("4GB")
    .cpu_cores("auto")
    .oom_prevention(true);

// Mirror configuration
config.mirrors()
    .auto_discover(true)
    .parallel_mirrors(3)
    .blacklist_region("cn")
    .whitelist_mirror("trusted-mirror.com");

// Security profile
config.security_profile(SecurityProfile::Corporate);

// Save configuration
config.save("/etc/aphelion/config.toml")?;
```

### Configuration File (TOML)

**Complete Configuration Example**:
```toml
# /etc/aphelion/config.toml

# General settings
[general]
version = "0.1.0"
profile = "personal"  # personal, government, corporate, development

# Network configuration
[network]
mode = "https_only"  # https_only, http_allowed, p2p_enabled
max_bandwidth = "auto"
parallel_downloads = 5
connection_timeout = 30
retry_attempts = 3
user_agent = "Aphelion/0.1.0"

# TLS/SSL settings
[network.tls]
version = "1.2+"
verify_certificates = true
certificate_pinning = true

# Mirror configuration
[mirrors]
auto_discover = true
parallel_mirrors = 3
fetch_strategy = "parallel_with_fallback"
health_check_interval = "5min"

# Mirror filtering
[mirrors.filtering]
blacklist = ["slow-mirror.com"]
blacklist_regions = []
whitelist = []
whitelist_regions = ["us", "eu"]

# Resource management
[resources]
mode = "adaptive"

[resources.memory]
min_reserve = "512MB"
max_reserve = "4GB"
target_utilization = 0.7
oom_prevention = true

[resources.cpu]
min_cores = 1
max_cores = "auto"
target_utilization = 0.8

[resources.disk]
io_priority = "normal"
max_iops = "auto"
async_io = true

[resources.network]
max_download_bandwidth = "auto"
connection_pooling = true

# I/O configuration
[io]
engine = "tokio"
io_uring = true

[io.write]
buffer_size = "64MB"
strategy = "write_ahead"
batch_writes = true

[io.read]
buffer_size = "64MB"
strategy = "read_ahead"
prefetch = true

# Security configuration
[security]
default_tier_limit = 2  # Allow up to Tier 2
require_signatures = true
verify_checksums = true

# Package tiers
[security.tiers]
tier0_enabled = true
tier1_enabled = true
tier2_enabled = true
tier2_requires_approval = false
tier3_enabled = false
tier3_requires_approval = true

# Logging
[logging]
level = "info"
file = "/var/log/aphelion/aphelion.log"
rotate_size = "100MB"
rotate_count = 10

# Performance tuning
[performance]
parallel_operations = "auto"
cache_size = "1GB"
aggressive_caching = false
preload_metadata = true
```

---

## Configuration Priority

When configuration is specified in multiple places, priority is:

1. **CLI arguments** (highest priority)
2. **Environment variables** (e.g., `APHELION_NETWORK_MODE=https`)
3. **User config file** (`~/.config/aphelion/config.toml`)
4. **System config file** (`/etc/aphelion/config.toml`)
5. **Compiled defaults** (lowest priority)

---

## Configuration Validation

All configuration changes are validated before being applied:

```bash
# Validate current config
$ aphelion config validate
✓ Network configuration valid
✓ Mirror configuration valid
✓ Resource limits valid
✓ Security settings valid
✓ All checks passed

# Validate config file
$ aphelion config validate --file my-config.toml
✗ Error: max_bandwidth "10ZB/s" exceeds system capability
✗ Error: blacklist_regions contains invalid region code "xx"
2 errors found
```

---

## Summary

Aphelion-rs provides:

1. **Network Flexibility**: HTTPS (default), HTTP (risky), P2P (high risk), custom providers
2. **Intelligent Resource Management**: Adaptive allocation, OOM prevention, dynamic detection
3. **Non-Blocking I/O**: Async operations, write-ahead logging, never blocks
4. **Smart Mirror Management**: Parallel fetching, load balancing, blacklist/whitelist, regional control
5. **Security Profiles**: Personal, Government, Corporate, Development
6. **Unified Configuration**: TUI, CLI, API, config file - all equally powerful
7. **Intuitive Management**: Easy to configure, hard to misconfigure

**Goal**: Maximum configurability with intuitive, straightforward management suitable for all use cases from personal desktop to government/enterprise deployments.
