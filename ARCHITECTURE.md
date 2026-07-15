# Aphelion-rs Architecture Documentation

## Vision & Strategic Direction

Aphelion-rs is designed as a next-generation package manager that bridges traditional package management with advanced holographic storage and computation paradigms. The architecture follows a big-picture-first approach, decomposing into constituent components through systematic reverse engineering of requirements.

---

## Big Picture: Strategic Goals

### Phase 1: POC Foundation (v0.0.x - v0.2.0)
**Focus**: Clean, idiomatic Rust implementations with minimal complexity
- Traditional .deb package handling as baseline
- Simple, straightforward error handling (not over-engineered)
- Modular architecture enabling future extensibility

### Phase 2: Enhanced Capabilities (v0.2.0+)
**Focus**: Source builds and VCS integration
- Clean, parameterized source code build system
- Multi-platform VCS integration (GitHub, GitLab, Gitea)
- Checksummed package registry with verification

### Phase 3: Holographic Storage Integration (Future)
**Focus**: Embeddenator integration for advanced storage
- Sparse balanced ternary HDC (Hyperdimensional Computing) VSA storage
- Holographic differential versioning
- Bind and bundle operations for version management
- Dense storage of cross-version dependency trees

---

## System Decomposition

### 1. Core Package Management Layer

#### 1.1 Package Operations Module
**Purpose**: Handle fundamental package lifecycle operations

**Components**:
- **Installation Engine**: Install packages from multiple sources
- **Removal Engine**: Clean uninstallation with dependency tracking
- **Update Engine**: Version management and upgrade paths
- **Query Engine**: Package search and information retrieval

**Interfaces**:
```rust
pub trait PackageOperations {
    fn install(&self, package: &PackageSpec) -> Result<InstallationResult>;
    fn remove(&self, package: &PackageId) -> Result<RemovalResult>;
    fn update(&self, package: &PackageId) -> Result<UpdateResult>;
    fn query(&self, criteria: &QueryCriteria) -> Result<Vec<PackageInfo>>;
}
```

#### 1.2 Distribution-Specific Handlers
**Purpose**: Abstract distro-specific package formats and conventions

**Current**: Debian/Ubuntu (.deb) support
**Future**: RPM, Arch, etc.

**Components**:
- `DebianHandler`: .deb parsing, dpkg integration
- `UbuntuHandler`: Ubuntu-specific repository handling
- Future: `RpmHandler`, `ArchHandler`, etc.

---

### 2. Source Build & Compilation Layer

#### 2.1 Build System
**Purpose**: Clean, idempotent source code compilation

**Requirements**:
- Dynamic parameterization for build configurations
- Idempotent operations (same inputs → same outputs)
- Clean build environment management
- Build artifact caching

**Components**:
- **Build Parser**: Parse and validate build specifications
- **Build Executor**: Execute build steps in isolated environment
- **Dependency Resolver**: Handle build-time dependencies
- **Artifact Manager**: Cache and manage build outputs

**Architecture**:
```
Source Package → Parser → Dependency Resolution → Build Sandbox
                                                         ↓
                                              Build Execution
                                                         ↓
                                              Artifact Validation → Cache
```

#### 2.2 Build Sandbox
**Purpose**: Isolated, secure build execution environment

**Components**:
- Container orchestration
- Resource limitation
- Network isolation
- Filesystem isolation

---

### 3. Security & Isolation Layer

#### 3.1 Sandbox Architecture
**Purpose**: Multi-layer defense against malicious code

**Security Layers**:

**Layer 1: App-Level Containerization**
- Container runtime integration (e.g., runc, crun)
- Isolated filesystem namespace
- Process isolation

**Layer 2: Kernel-Level Isolation**
- Linux namespaces (PID, NET, MNT, IPC, UTS, USER)
- cgroups for resource control
- seccomp-bpf for syscall filtering
- AppArmor/SELinux profiles

**Layer 3: Full OSI Layer 7 Isolation & Firewalling**
- Complete network stack isolation (L2-L7)
- Deep packet inspection and filtering
- Application protocol filtering (HTTP, HTTPS, DNS, etc.)
- Per-package/per-version network policies
- SSL/TLS interception and inspection
- Protocol-aware filtering
- Content filtering and validation

**Multi-Layer Network Architecture**:
```
Package Install Request
    ↓
Security Policy Check
    ↓
┌─────────────────────────────────────────────────────────┐
│  Sandbox Container (Per Package Version)                │
│  ┌───────────────────────────────────────────────────┐  │
│  │ Layer 7: Application Firewall                     │  │
│  │ - HTTP/HTTPS protocol inspection                  │  │
│  │ - DNS query filtering & validation                │  │
│  │ - TLS/SSL termination & inspection                │  │
│  │ - Application-specific protocol rules             │  │
│  │ - Content-based filtering                         │  │
│  │ - API endpoint whitelisting                       │  │
│  └───────────────────────────────────────────────────┘  │
│  ┌───────────────────────────────────────────────────┐  │
│  │ Layer 4: Transport Firewall                       │  │
│  │ - TCP/UDP port filtering                          │  │
│  │ - Connection tracking                             │  │
│  │ - Rate limiting per connection                    │  │
│  │ - Protocol validation                             │  │
│  └───────────────────────────────────────────────────┘  │
│  ┌───────────────────────────────────────────────────┐  │
│  │ Layer 3: Network Firewall                         │  │
│  │ - IP address whitelisting/blacklisting            │  │
│  │ - CIDR-based rules                                │  │
│  │ - Routing table isolation                         │  │
│  │ - NAT/masquerading                                │  │
│  └───────────────────────────────────────────────────┘  │
│  ┌───────────────────────────────────────────────────┐  │
│  │ Network Namespace Isolation                       │  │
│  │ - Dedicated virtual network interface             │  │
│  │ - Isolated network stack                          │  │
│  │ - Per-version network identity                    │  │
│  │ - Bridge to host (controlled)                     │  │
│  └───────────────────────────────────────────────────┘  │
│  ┌───────────────────────────────────────────────────┐  │
│  │ Filesystem Namespace                              │  │
│  │ - Read-only base                                  │  │
│  │ - Version-specific overlay                        │  │
│  │ - No cross-version file access                    │  │
│  └───────────────────────────────────────────────────┘  │
│  ┌───────────────────────────────────────────────────┐  │
│  │ Process Namespace                                 │  │
│  │ - PID isolation                                   │  │
│  │ - Seccomp filter                                  │  │
│  │ - No inter-version IPC                            │  │
│  └───────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────┘
```

**Layer 7 Firewall Components**:
- **HTTP/HTTPS Inspector**: Deep packet inspection of web traffic
  - URL filtering
  - Method filtering (GET, POST, etc.)
  - Header inspection and validation
  - Response code monitoring
  
- **DNS Firewall**: Comprehensive DNS security
  - Query logging and analysis
  - Domain whitelisting/blacklisting
  - DNS rebinding protection
  - DNSSEC validation
  
- **TLS/SSL Gateway**: Encrypted traffic inspection
  - Certificate validation
  - Protocol version enforcement (TLS 1.2+)
  - Cipher suite restrictions
  - Certificate pinning support
  
- **Protocol Analyzers**: Application-specific protocol enforcement
  - SSH connection monitoring
  - FTP/SFTP filtering
  - SMTP/IMAP restrictions
  - Custom protocol parsers

**Per-Version Isolation**:
Each package version gets its own isolated network environment:
```
Package A v1.0.0 → Namespace A1 → Firewall Policy A1 → Network Stack A1
Package A v1.1.0 → Namespace A2 → Firewall Policy A2 → Network Stack A2
Package B v2.0.0 → Namespace B1 → Firewall Policy B1 → Network Stack B1
```

No cross-version communication unless explicitly allowed by policy.

#### 3.2 Security Policy Engine
**Purpose**: Define and enforce security policies

**Components**:
- Policy parser and validator
- Runtime enforcement engine
- Audit logging
- Violation handling

---

### 4. VCS Integration Layer

#### 4.1 Multi-Platform VCS Support
**Purpose**: Unified interface for GitHub, GitLab, Gitea

**Components**:
- **VCS Abstraction Layer**: Common interface for all VCS platforms
- **Search & Index Engine**: Cross-platform package discovery
- **Authentication Manager**: Secure credential handling
- **Rate Limiting**: Respect API limits

**Interface**:
```rust
pub trait VcsProvider {
    fn search_packages(&self, query: &str) -> Result<Vec<PackageMetadata>>;
    fn fetch_package(&self, repo: &RepoSpec) -> Result<PackageSource>;
    fn verify_signature(&self, package: &Package) -> Result<bool>;
    fn get_checksums(&self, package: &Package) -> Result<Checksums>;
}
```

#### 4.2 Package Registry & Mirror System
**Purpose**: Checksummed record of all supported package versions

**Components**:
- **Top-Level Mirror**: Central registry with checksums
- **Verification Engine**: Cryptographic checksum validation
- **Signature Verification**: GPG/sigstore integration
- **Mirror Sync**: Distributed mirror network

**Data Model**:
```rust
pub struct PackageRegistry {
    packages: HashMap<PackageId, Vec<PackageVersion>>,
    checksums: HashMap<PackageVersion, Checksum>,
    signatures: HashMap<PackageVersion, Signature>,
    mirrors: Vec<MirrorUrl>,
}
```

---

### 5. Storage Backend Layer

#### 5.1 Traditional Storage (Phase 1-2)
**Purpose**: SQLite-based package database

**Components**:
- Package metadata storage
- Dependency graphs
- Installation records
- Version tracking

#### 5.2 Holographic Storage (Phase 3+)
**Purpose**: Embeddenator integration for advanced storage

**Concepts**:
- **Sparse Balanced Ternary HDC**: Hyperdimensional vector representations
- **Holographic Differentials**: Store only version deltas in latent space
- **Bind & Bundle Operations**: Mathematical composition of package versions
- **Cosine Similarity**: Ensure bitwise-perfect accuracy through similarity checks

**Architecture**:
```
Package Version A → Encode (HDC) → Vector_A
Package Version B → Encode (HDC) → Vector_B
                                    ↓
                    Differential = Vector_B - Vector_A (flattened)
                                    ↓
                    Store minimal holographic differential
                                    ↓
Reconstruct B = Vector_A + Differential (bind & bundle)
                                    ↓
                    Verify with cosine similarity ≈ 1.0
```

**Benefits**:
- Store 50+ versions with minimal storage overhead
- Sandboxed cross-version dependency resolution
- Dense storage through holographic compression
- Mathematical version composition (no traditional diffing)

**Operations**:
- **Bind**: Associate version differential with base
- **Bundle**: Compose multiple differentials
- **Unbundle**: Extract specific version from holographic store

---

### 6. Network & Fetch Layer

#### 6.1 Async HTTP Client
**Purpose**: Efficient package downloads

**Components**:
- Parallel download manager
- Resume capability
- Checksum streaming verification
- Bandwidth throttling

#### 6.2 Mirror Optimization
**Purpose**: Dynamic mirror selection

**Components**:
- Latency testing
- Bandwidth estimation
- Geographic optimization
- Fallback handling

---

### 7. Configuration & State Management

#### 7.1 Configuration System
**Purpose**: Unified configuration management

**Components**:
- TOML/YAML configuration parsing
- Environment variable overlay
- CLI argument precedence
- Configuration validation

#### 7.2 State Tracking
**Purpose**: Maintain system state

**Components**:
- Installation state machine
- Rollback capability
- Transaction log
- State verification

---

## Module Architecture

### Current Module Structure
```
src/
├── lib.rs              # Library exports, common types
├── main.rs             # Binary entry point
├── config/             # Configuration management
├── database/           # Storage backend (SQLite/future HDC)
├── network/            # HTTP client, VCS integration
├── package/            # Core package operations
├── debian/             # Debian-specific handlers
└── utils/              # Shared utilities
```

### Future Module Structure
```
src/
├── lib.rs
├── main.rs
├── core/
│   ├── config/
│   ├── state/
│   └── error/
├── package/
│   ├── ops/            # Install, remove, update
│   ├── query/          # Search and info
│   └── verify/         # Signature and checksum
├── build/
│   ├── parser/         # Build spec parsing
│   ├── executor/       # Build execution
│   └── sandbox/        # Build isolation
├── security/
│   ├── sandbox/        # Runtime sandboxing
│   ├── policy/         # Security policies
│   └── firewall/       # Network isolation
├── vcs/
│   ├── github/         # GitHub integration
│   ├── gitlab/         # GitLab integration
│   ├── gitea/          # Gitea integration
│   └── registry/       # Package registry
├── storage/
│   ├── traditional/    # SQLite backend
│   └── holographic/    # Embeddenator HDC integration
├── distro/
│   ├── debian/
│   ├── ubuntu/
│   └── common/
└── network/
    ├── fetch/
    ├── mirror/
    └── sync/
```

---

## Data Flow Architecture

### Package Installation Flow
```
User Command
    ↓
CLI Parser → Argument Validation
    ↓
Package Resolver → Dependency Analysis
    ↓
Source Selection (Registry/VCS/Mirror)
    ↓
Security Policy Check
    ↓
Fetch Package (with verification)
    ↓
Sandbox Preparation
    ↓
Installation in Sandbox
    ↓
Post-Install Verification
    ↓
Commit to System (or Rollback)
    ↓
Update State Database
```

### Source Build Flow
```
Source Package Request
    ↓
VCS Integration → Fetch Source
    ↓
Checksum Verification
    ↓
Build Spec Parsing
    ↓
Build Dependency Resolution
    ↓
Sandbox Creation (Build Isolation)
    ↓
Build Execution (Idempotent)
    ↓
Artifact Generation
    ↓
Build Verification
    ↓
Artifact Caching
    ↓
Installation Flow (as above)
```

---

## Security Considerations

### Threat Model
1. **Malicious Package Code**: Contained via multi-layer sandboxing
2. **Supply Chain Attacks**: Mitigated through checksums and signatures
3. **Network MITM**: TLS + signature verification
4. **Local Privilege Escalation**: Namespaces + seccomp filtering
5. **Resource Exhaustion**: cgroups limits

### Defense Strategies
- **Defense in Depth**: Multiple independent security layers
- **Principle of Least Privilege**: Minimal permissions by default
- **Fail Secure**: Deny by default, explicit allow
- **Audit Everything**: Comprehensive logging of security events
- **Verifiable Builds**: Reproducible builds where possible

---

## Performance Considerations

### Optimization Strategies
1. **Parallel Operations**: Tokio async runtime for concurrency
2. **Caching**: Multi-level caching (metadata, packages, builds)
3. **Lazy Loading**: Load-on-demand for heavy operations
4. **Efficient Storage**: Holographic differentials reduce storage overhead
5. **Smart Mirroring**: Geographic and performance-based selection

---

## Technology Stack Evolution

### Phase 1 (Current - v0.2.0)
- Rust 2021 Edition
- Tokio for async runtime
- SQLite for storage
- reqwest for HTTP
- clap for CLI
- crossterm + tui for TUI

### Phase 2 (v0.2.0 - v1.0)
- Add: Container runtime integration (runc/crun)
- Add: seccomp-bpf for syscall filtering
- Add: VCS API clients (GitHub, GitLab, Gitea)
- Add: Build system (custom or integrate existing)

### Phase 3 (v1.0+)
- Add: Embeddenator integration
- Add: HDC/VSA storage backend
- Add: Holographic differential engine
- Evolve: From SQLite to hybrid storage

---

## Design Principles

1. **Simplicity First**: Start simple, add complexity only when needed
2. **Modularity**: Clear module boundaries, swappable implementations
3. **Security by Default**: Secure configurations out of the box
4. **Idiomatic Rust**: Follow Rust conventions and best practices
5. **Progressive Enhancement**: Build foundation, layer on advanced features
6. **Big Picture → Components**: Top-down decomposition with bottom-up validation

---

## Open Questions & Future Research

1. **Embeddenator Integration Timeline**: When is HDC storage mature enough?
2. **Container Runtime**: runc, crun, or custom minimal implementation?
3. **Build System**: Custom vs. existing (e.g., Bazel, Buck)?
4. **Cross-Distro Abstraction**: How much can be unified vs. distro-specific?
5. **Performance vs. Security**: Optimal tradeoffs for sandboxing overhead?
6. **Holographic Accuracy**: Verify cosine similarity threshold for bitwise perfection

---

This architecture document will evolve as the system is implemented and requirements are refined through systematic decomposition and reverse engineering of each subsystem.
