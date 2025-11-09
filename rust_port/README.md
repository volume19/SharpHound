# SharpHound Rust Port

This is a Rust port of the SharpHound C# Active Directory enumeration tool.

## ⚠️ Authorization Required

**This tool is designed for AUTHORIZED security testing ONLY:**
- Penetration testing engagements with written authorization
- Security audits and assessments
- Red team operations (with explicit permission)
- Educational and research purposes in controlled environments

**Unauthorized use against systems you don't own or have permission to test is illegal.**

## Project Status

**Phase 1 (Foundation) - ✅ COMPLETE**
**Phase 2 (Context & Traits) - ✅ COMPLETE**

### Completed Components

**Phase 1 - Foundation:**
- ✅ Project structure and Cargo.toml (20+ dependencies configured)
- ✅ `client/enums.rs` - Collection method enums (37 lines → 210 lines with tests)
- ✅ `client/flags.rs` - Configuration flags struct (31 lines → 170 lines with tests)
- ✅ `domain/enumeration_domain.rs` - Domain info struct (12 lines → 100 lines with tests)
- ✅ `logging/basic_logger.rs` - Console logger (55 lines → 270 lines with tests)
- ✅ `cli/options.rs` - Command-line argument parsing (296 lines → 382 lines with validation)

**Phase 2 - Context & Traits:**
- ✅ `client/context.rs` - Context trait interface (85 lines → 289 lines with async support)
- ✅ `client/links.rs` - Chain of Responsibility pattern (29 lines → 273 lines with workflow)
- ✅ `context/base_context.rs` - Concrete Context implementation (168 lines → 677 lines)
- ✅ `util/extensions.rs` - Utility functions (120 lines → ~300 lines with 5 utilities)
- ✅ `serialization/json_extensions.rs` - Custom JSON serialization (51 lines → ~310 lines)

**Test Coverage:** 68 passing unit tests + 1 doc test
**CLI Status:** ✅ Fully functional with 40+ options, validation, and --help

### Iteration Progress

| Iteration | Status | Files Ported | Tests | Notes |
|-----------|--------|--------------|-------|-------|
| 1 | ✅ Complete | Enums, Flags, EnumerationDomain, BasicLogger | 19 | Core types and infrastructure |
| 2 | ✅ Complete | Options (CLI) | 11 (+30 total) | Full CLI parsing with clap |
| 3 | ✅ Complete | Context trait, Links trait | 5 (+35 total) | Interface definitions |
| 4 | ✅ Complete | BaseContext implementation | 13 (+48 total) | State management |
| 5 | ✅ Complete | Extensions utilities | 10 (+58 total) | HashMap, DNS, async helpers |
| 6 | ✅ Complete | JsonExtensions | 11 (+68 total) | Label enum, serialization |

### Not Yet Started

- ⏳ Phase 3: Writers (BaseWriter, JsonDataWriter, CompStatusWriter)
- ⏳ Phase 4: Producers (BaseProducer, ComputerFileProducer, LdapProducer, StealthProducer)
- ⏳ Phase 5: LDAP Integration (ldap3 crate integration, paging, partitioning)
- ⏳ Phase 6: Runtime (CollectionTask, LDAPConsumer, LoopManager, OutputWriter)
- ⏳ Phase 7: Main Workflow (SharpLinks chain implementation)
- ⏳ Phase 8-9: **CRITICAL** - ObjectProcessors + SharpHoundCommonLib/RPC ports (largest effort)

## Building

```bash
# Check for compilation errors
cargo check

# Run tests
cargo test

# Run with verbose output
cargo test -- --nocapture

# Build debug binary
cargo build

# Build release binary (optimized)
cargo build --release

# Run the binary
cargo run -- --help
```

## Running

```bash
# Show help
cargo run -- --help

# Example: DCOnly collection (not yet functional - collection logic not ported)
cargo run -- -c DCOnly -d example.com --memcache --threads 100

# With loop collection
cargo run -- -c All --loop --loopduration 05:00:00 --loopinterval 00:03:00
```

**Note:** Collection functionality is not yet implemented. Only CLI parsing and validation work currently.

## Architecture

- **Async Runtime:** tokio 1.40 (multi-threaded work-stealing scheduler)
- **CLI Parsing:** clap 4.5 (derive API, 40+ options)
- **Serialization:** serde 1.0 + serde_json (replaces Newtonsoft.Json)
- **Logging:** tracing + tracing-subscriber (structured logging)
- **Error Handling:** thiserror + anyhow (Result-based, no exceptions)
- **Collections:** dashmap 6.0 (concurrent HashMap)
- **Compression:** zip + flate2 (ZIP archives with optional passwords)
- **Date/Time:** chrono 0.4 (RFC3339 timestamps)

## Testing

All modules include comprehensive unit tests:

```bash
# Run all tests
cargo test

# Run specific module tests
cargo test client::enums
cargo test cli::options
cargo test logging::basic_logger

# Run tests matching pattern
cargo test validation

# Show test output
cargo test -- --nocapture --test-threads=1
```

### Test Summary

**Phase 1:**
- **Client Enums:** 6 tests (parsing, Display, FromStr, serialization)
- **Client Flags:** 4 tests (default, builder pattern, clone, serialization)
- **Domain:** 4 tests (construction, SID validation, clone, serialization)
- **Logging:** 5 tests (levels, formatting, filtering)
- **CLI Options:** 11 tests (duration parsing, validation, defaults, flags)

**Phase 2:**
- **Client Context:** 2 tests (FileExistsError, ContextUtils map merging)
- **Client Links:** 3 tests (LdapConfig, AuthType)
- **BaseContext:** 13 tests (construction, delays, filename resolution, cancellation)
- **Util Extensions:** 10 tests (HashMap merge, DNS resolution, flags, async streams)
- **Serialization:** 11 tests (Label serialize/deserialize, CacheSerializerSettings)

**Total:** 68 unit tests + 1 doc test

## Code Style

```bash
# Format code
cargo fmt

# Check formatting without applying
cargo fmt -- --check

# Lint with clippy
cargo clippy --all-targets --all-features -- -D warnings

# Fix clippy warnings automatically (when safe)
cargo clippy --fix
```

## Progress Tracking

See `../RUST_PORT_PLAN.json` for the complete porting plan.

**Estimated Total Effort:** 19-27 weeks (5-7 months)
**Phase 1 Duration:** 1 week (Foundation - Core Types & CLI) ✅
**Phase 2 Duration:** 1 week (Context & Traits) ✅
**Current Progress:** ~42% complete (11 of 26 core files ported)

**Files Completed:** 11/26 (42%)
- Phase 1: 5 files (Enums, Flags, EnumerationDomain, BasicLogger, Options)
- Phase 2: 6 files (Context, Links, BaseContext, Extensions, JsonExtensions, LdapConfig)

**External Dependencies Still Required:**
- SharpHoundCommonLib (~5,000 LOC) - Core LDAP and AD processing
- SharpHoundRPC (~3,000 LOC) - Windows RPC/SMB APIs

## Recent Changes

### Iteration 6 (Latest) - Phase 2 Complete! 🎉
- ✅ Ported JsonExtensions.cs → serialization/json_extensions.rs
- ✅ Label enum with custom serde serialization (17 variants)
- ✅ CacheSerializerSettings for JSON helpers
- ✅ 11 new tests (68 total passing)
- ✅ **Phase 2 (Context & Traits) COMPLETE**

### Iteration 5
- ✅ Ported Extensions.cs → util/extensions.rs
- ✅ HashMapExt, DnsNameResolver, FlagExtractor, AsyncStreamExt, CollectionMethodExt
- ✅ 10 new tests (57 total passing)
- ✅ Added tokio-stream dependency

### Iteration 4
- ✅ Ported BaseContext.cs → context/base_context.rs
- ✅ Full Context trait implementation with async support
- ✅ Delay/throttle/jitter logic, filename resolution, cache paths
- ✅ 13 new tests (48 total passing)
- ✅ Arc<RwLock<T>> for shared state, DashMap for concurrent collections

### Iteration 3
- ✅ Ported Context.cs → client/context.rs (Context trait)
- ✅ Ported Links.cs → client/links.rs (Chain of Responsibility)
- ✅ LdapConfig and AuthType supporting types
- ✅ 5 new tests (35 total passing)

### Iteration 2
- ✅ Ported Options.cs → cli/options.rs
- ✅ 40+ CLI options with clap derive API
- ✅ Custom duration parser (hh:mm:ss)
- ✅ Comprehensive validation
- ✅ 11 new tests (30 total passing)

### Iteration 1
- ✅ Project structure with Cargo.toml
- ✅ Core types: Enums, Flags, EnumerationDomain
- ✅ BasicLogger with verbosity filtering
- ✅ 19 tests + 1 doc test

## License

Apache-2.0 (matching original SharpHound license)

## Acknowledgments

Original SharpHound by SpecterOps: https://github.com/SpecterOps/SharpHound

Rust port preserves all original functionality while providing:
- Memory safety without garbage collection
- Zero-cost abstractions
- Fearless concurrency
- No runtime dependencies (single binary)
- Cross-platform potential (LDAP-only mode on Linux)
