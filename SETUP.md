# Invisible Development Setup

This guide covers the complete setup process for Invisible development.

## Prerequisites

### Required
- **Rust:** 1.70 or later (`rustup install stable`)
- **Git:** 2.30 or later
- **Claude Code:** Latest version

### Recommended
- **cargo-deny:** Security auditing (`cargo install cargo-deny`)
- **cargo-audit:** Vulnerability scanning (`cargo install cargo-audit`)
- **cargo-tarpaulin:** Code coverage (`cargo install cargo-tarpaulin`)
- **cargo-watch:** Auto-rebuild on changes (`cargo install cargo-watch`)

## Quick Start

### 1. Clone Repository

```bash
git clone https://github.com/Light-Brands/invisible.git
cd invisible
git submodule update --init --recursive
```

### 2. Install Claude Code Plugins

**Important:** Run these commands in a **separate terminal** (not inside Claude Code):

```bash
# Add the ai-coding-config marketplace
claude "/plugin marketplace add https://github.com/TechNickAI/ai-coding-config"

# Install core plugin
claude "/plugin install ai-coding-config"

# Install Sherlock personality (analytical, security-focused)
claude "/plugin install personality-sherlock"

# Enable auto-updates (recommended)
# Run: claude "/plugin"
# Navigate to Marketplaces → ai-coding-config → Enable auto-update
```

After installation, **restart Claude Code**:
```bash
claude
```

### 3. Verify Plugin Installation

```bash
claude "/plugin list"
```

You should see:
- ✅ `ai-coding-config@9.16.0` (or later)
- ✅ `personality-sherlock@latest`

### 4. Install Rust Tooling

```bash
# Install nightly for some advanced features
rustup toolchain install nightly

# Install components
rustup component add rustfmt clippy

# Install security tools
cargo install cargo-deny cargo-audit

# Install development tools
cargo install cargo-watch cargo-tarpaulin
```

### 5. Verify Setup

```bash
# Check formatting
cargo fmt --check

# Run linter
cargo clippy --all-targets

# Security audit
cargo deny check
cargo audit

# Build workspace
cargo build

# Run tests (when available)
cargo test
```

## Project Structure

```
invisible/
├── .ai-coding-config/         # Git submodule: AI tooling config
├── .github/
│   └── workflows/
│       └── ci.yml            # GitHub Actions CI pipeline
├── crates/                    # Rust workspace crates
│   ├── crypto/               # (to be created) Cryptography primitives
│   ├── scrambler/            # (to be created) Network obfuscation
│   ├── relay/                # (to be created) Relay node implementation
│   ├── wallet/               # (to be created) Shadow Wallet
│   └── client/               # (to be created) Client libraries
├── spec/                      # Complete specifications
├── AGENTS.md                  # AI assistant project context
├── CLAUDE.md                  # Symlink to AGENTS.md
├── Cargo.toml                 # Workspace manifest
├── deny.toml                  # cargo-deny configuration
├── .rustfmt.toml             # Rust formatting rules
├── .clippy.toml              # Clippy linting configuration
└── .gitignore                # Git ignore rules
```

## Development Workflow

### Running Tests

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_name

# Run tests in specific crate
cargo test -p crate-name
```

### Code Quality Checks

```bash
# Format code
cargo fmt

# Run clippy
cargo clippy --all-targets --all-features

# Security audit
cargo deny check advisories
cargo audit

# Check for outdated dependencies
cargo outdated
```

### Development Helpers

```bash
# Auto-rebuild on file changes
cargo watch -x build

# Auto-run tests on changes
cargo watch -x test

# Clear build cache
cargo clean
```

### Benchmarking

```bash
# Run benchmarks (when available)
cargo bench

# Run specific benchmark
cargo bench bench_name
```

## Claude Code Commands

Key commands for Invisible development:

### Development
- `/autotask "task description"` - Autonomous feature implementation
- `/systematic-debugging` - Root cause analysis for bugs
- `/verify-fix` - Confirm fixes actually work

### Security & Quality
- `/security-reviewer` - OWASP audits, crypto review, vulnerability scanning
- `/architecture-auditor` - Design pattern validation
- `/test-engineer` - Write comprehensive tests
- `/error-handling-reviewer` - Ensure errors fail securely

### Code Review
- `/multi-review` - Run multiple specialized reviewers
- `/robustness-reviewer` - Production readiness checking
- `/performance-reviewer` - Find bottlenecks, optimize

### Workflow
- `/load-rules` - Load task-specific context
- `/address-pr-comments` - Auto-triage and fix PR feedback
- `/wrap-up` - Merge PR, sync, cleanup

## Security Checklist

Before committing security-critical code:

- [ ] All crypto operations are constant-time
- [ ] Sensitive data is zeroized on drop
- [ ] No unsafe code (or thoroughly reviewed)
- [ ] Property tests for crypto invariants
- [ ] Fuzzing for parsers and protocol handlers
- [ ] Dependencies audited with `cargo deny` and `cargo audit`
- [ ] Threat model documented
- [ ] Error handling fails securely

## Troubleshooting

### Plugin Installation Issues

If plugins don't appear after installation:
1. Restart Claude Code completely
2. Verify marketplace is added: `claude "/plugin marketplace list"`
3. Check installation: `claude "/plugin list"`
4. Try manual update: `claude "/plugin marketplace update ai-coding-config"`

### Build Errors

If you encounter build errors:
```bash
# Clean and rebuild
cargo clean
cargo build

# Update dependencies
cargo update

# Check for conflicts
cargo tree --duplicates
```

### Git Submodule Issues

If `.ai-coding-config` is empty:
```bash
git submodule update --init --recursive
```

## Additional Resources

- [AGENTS.md](AGENTS.md) - Complete project context for AI assistants
- [Master Plan](spec/MASTER-PLAN.md) - Strategic vision
- [Architecture Specs](spec/architecture/) - Technical deep-dives
- [Epics](spec/epics/) - Feature development roadmap

## Next Steps

1. ✅ Verify plugin installation
2. ✅ Run security audit (`cargo deny check && cargo audit`)
3. 📦 Create first crate (`cargo new --lib crates/crypto`)
4. 🔐 Implement core cryptography primitives
5. ✅ Write tests and benchmarks
6. 🚀 Start building!

---

**Remember:** Security is paramount. Every decision should prioritize user privacy.
