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

**Phase 1 (Foundation) - IN PROGRESS**

### Completed Components

- ✅ Project structure and Cargo.toml
- ✅ `client/enums.rs` - Collection method enums (37 lines → 210 lines with tests)
- ✅ `client/flags.rs` - Configuration flags struct (31 lines → 170 lines with tests)
- ✅ `domain/enumeration_domain.rs` - Domain info struct (12 lines → 100 lines with tests)
- ✅ `logging/basic_logger.rs` - Console logger (55 lines → 270 lines with tests)

**Test Coverage:** 19 passing unit tests + 1 doc test

### In Progress

- 🚧 `cli/options.rs` - Command-line argument parsing with clap

### Not Started

- ⏳ Context trait and implementation
- ⏳ Producers (LDAP, File, Stealth)
- ⏳ Runtime coordination and collection tasks
- ⏳ Writers (JSON, CSV, ZIP)
- ⏳ Object processors (requires porting SharpHoundCommonLib)

## Building

```bash
# Check for compilation errors
cargo check

# Run tests
cargo test

# Build debug binary
cargo build

# Build release binary (optimized)
cargo build --release
```

## Running

```bash
# Currently only shows initialization message
cargo run

# Release binary
./target/release/sharphound --help  # (Not yet implemented)
```

## Architecture

- **Async Runtime:** tokio 1.40
- **CLI Parsing:** clap 4.5 (derive API)
- **Serialization:** serde 1.0 + serde_json
- **Logging:** tracing + tracing-subscriber
- **Error Handling:** thiserror + anyhow
- **Collections:** dashmap (concurrent HashMap)
- **Compression:** zip + flate2

## Testing

All modules include comprehensive unit tests:

```bash
# Run all tests
cargo test

# Run specific module tests
cargo test client::enums
cargo test logging::basic_logger

# Run with output
cargo test -- --nocapture
```

## Code Style

```bash
# Format code
cargo fmt

# Lint with clippy
cargo clippy --all-targets --all-features -- -D warnings
```

## Progress Tracking

See `RUST_PORT_PLAN.json` in the root directory for the complete porting plan.

**Estimated Total Effort:** 19-27 weeks (5-7 months)
**Phase 1 Duration:** 1 week (Foundation - Core Types & CLI)

## License

Apache-2.0 (matching original SharpHound license)

## Acknowledgments

Original SharpHound by SpecterOps: https://github.com/SpecterOps/SharpHound
