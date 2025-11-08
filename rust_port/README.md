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

### Completed Components

- ✅ Project structure and Cargo.toml (20+ dependencies configured)
- ✅ `client/enums.rs` - Collection method enums (37 lines → 210 lines with tests)
- ✅ `client/flags.rs` - Configuration flags struct (31 lines → 170 lines with tests)
- ✅ `domain/enumeration_domain.rs` - Domain info struct (12 lines → 100 lines with tests)
- ✅ `logging/basic_logger.rs` - Console logger (55 lines → 270 lines with tests)
- ✅ `cli/options.rs` - Command-line argument parsing (296 lines → 382 lines with validation)

**Test Coverage:** 30 passing unit tests + 1 doc test
**CLI Status:** ✅ Fully functional with 40+ options, validation, and --help

### Iteration Progress

| Iteration | Status | Files Ported | Tests | Notes |
|-----------|--------|--------------|-------|-------|
| 1 | ✅ Complete | Enums, Flags, EnumerationDomain, BasicLogger | 19 | Core types and infrastructure |
| 2 | ✅ Complete | Options (CLI) | 11 (+30 total) | Full CLI parsing with clap |
| 3 | ⏳ Next | Context trait, BaseContext | TBD | State management |

### Not Yet Started

- ⏳ Phase 2: Context & Traits (Context, Links, BaseContext, Extensions, JsonExtensions)
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

- **Client Enums:** 6 tests (parsing, Display, FromStr, serialization)
- **Client Flags:** 4 tests (default, builder pattern, clone, serialization)
- **Domain:** 4 tests (construction, SID validation, clone, serialization)
- **Logging:** 5 tests (levels, formatting, filtering)
- **CLI Options:** 11 tests (duration parsing, validation, defaults, flags)
- **Total:** 30 unit tests + 1 doc test

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
**Phase 1 Duration:** 1 week (Foundation - Core Types & CLI)
**Current Progress:** ~8% complete (5 of 26 core files ported)

**External Dependencies Still Required:**
- SharpHoundCommonLib (~5,000 LOC) - Core LDAP and AD processing
- SharpHoundRPC (~3,000 LOC) - Windows RPC/SMB APIs

## Recent Changes

### Iteration 2 (Latest)
- ✅ Ported Options.cs → cli/options.rs
- ✅ 40+ CLI options with clap derive API
- ✅ Custom duration parser (hh:mm:ss)
- ✅ Comprehensive validation (LDAP credentials, local admin, jitter, verbosity)
- ✅ Integration with main.rs
- ✅ 11 new tests (30 total passing)
- ✅ Full `--help` output working

### Iteration 1
- ✅ Project structure with Cargo.toml
- ✅ Core types: Enums, Flags, EnumerationDomain
- ✅ BasicLogger with verbosity filtering
- ✅ 19 tests + 1 doc test
- ✅ Module hierarchy (client, domain, logging, cli)

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
