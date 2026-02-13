# Contributing to Invisible

**Welcome!** Invisible is a gift to humanity - a privacy-first messenger with zero-trust architecture. We believe privacy is a fundamental human right, and we're building the tools to protect it.

## 🎯 Our Mission

**Messages that leave no trace. Privacy that answers to no one.**

Invisible provides:
- End-to-end encrypted messaging (Signal protocol + post-quantum)
- 7-layer network obfuscation (VPN, mixnets, dead drops)
- Privacy-first cryptocurrency wallet (XMR, BTC, ZEC, ETH)
- Zero metadata collection (no phone, email, or username required)

## 🌍 Philosophy

- **Privacy by default:** All features maximize user privacy
- **Zero compromise:** We never trade privacy for convenience
- **Open source:** Transparency builds trust
- **Community-driven:** Everyone's voice matters
- **Inclusive:** All skill levels welcome

## 🤝 How to Contribute

### 1. Find an Issue

Check our [Issues](https://github.com/Light-Brands/invisible/issues) page for:
- `good first issue` - Perfect for newcomers
- `help wanted` - We need your expertise
- `bug` - Something's broken
- `enhancement` - New features

Don't see what you want to work on? [Create a new issue](https://github.com/Light-Brands/invisible/issues/new)!

### 2. Fork & Clone

```bash
# Fork the repo on GitHub, then:
git clone https://github.com/YOUR_USERNAME/invisible.git
cd invisible

# Add upstream remote
git remote add upstream https://github.com/Light-Brands/invisible.git
```

### 3. Create a Branch

```bash
git checkout -b feature/your-feature-name
# or
git checkout -b fix/bug-description
```

### 4. Make Your Changes

**Follow our coding standards:**

#### Rust Code
- Run `cargo fmt` before committing
- Run `cargo clippy` and fix warnings
- Add tests for new functionality
- Update documentation

```bash
# Format code
cargo fmt --all

# Check for issues
cargo clippy --all-targets --all-features

# Run tests
cargo test --workspace
```

#### Flutter/Dart Code
- Follow [Effective Dart](https://dart.dev/guides/language/effective-dart) style
- Run `dart format` before committing
- Add widget tests for UI components
- Update documentation

```bash
# Format code
cd invisible_app
dart format .

# Analyze code
flutter analyze

# Run tests
flutter test
```

### 5. Commit Your Changes

**Write clear commit messages:**

```
feat: add post-quantum key exchange

- Implement ML-KEM-768 for key agreement
- Add tests for PQXDH protocol
- Update documentation

Closes #123
```

**Commit message format:**
```
<type>: <subject>

<body>

<footer>
```

**Types:**
- `feat:` New feature
- `fix:` Bug fix
- `docs:` Documentation only
- `style:` Formatting, missing semicolons, etc.
- `refactor:` Code restructuring
- `test:` Adding tests
- `chore:` Maintenance tasks

### 6. Push & Create Pull Request

```bash
git push origin feature/your-feature-name
```

Then create a Pull Request on GitHub with:
- Clear title describing the change
- Description of what you changed and why
- Reference to related issues (#123)
- Screenshots (if UI changes)

### 7. Code Review

- Maintainers will review your PR
- Address feedback by pushing new commits
- Once approved, your code will be merged!

## 🏗️ Development Setup

### Prerequisites

- **Rust:** 1.70+ ([install](https://rustup.rs/))
- **Flutter:** 3.0+ ([install](https://docs.flutter.dev/get-started/install))
- **Git:** Latest version

### Quick Start

```bash
# Clone repository
git clone https://github.com/Light-Brands/invisible.git
cd invisible

# Build Rust core
cargo build --release

# Run tests
cargo test --workspace

# Start relay node
./target/release/invisible-relay

# Set up Flutter (see docs/FLUTTER_SETUP.md)
flutter doctor
cd invisible_app
flutter run
```

**Detailed setup:** See [docs/FLUTTER_SETUP.md](docs/FLUTTER_SETUP.md)

## 🧪 Testing

### Running Tests

```bash
# All Rust tests
cargo test --workspace

# Specific crate
cargo test -p invisible-crypto

# With output
cargo test -- --nocapture

# Flutter tests
cd invisible_app
flutter test
```

### Writing Tests

**Every new feature needs tests:**

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feature_works() {
        // Arrange
        let input = create_test_input();

        // Act
        let result = my_function(input);

        // Assert
        assert_eq!(result, expected);
    }
}
```

## 📚 Documentation

- **Code comments:** Explain WHY, not WHAT
- **Module docs:** `//!` at top of files
- **Function docs:** `///` above public functions
- **Examples:** Show usage in docs

```rust
/// Encrypts a message using the Double Ratchet algorithm.
///
/// # Arguments
/// * `message` - Plaintext message to encrypt
/// * `ratchet_state` - Current ratchet state
///
/// # Returns
/// Encrypted message with authentication tag
///
/// # Example
/// ```
/// let encrypted = encrypt_message(b"Hello", &state)?;
/// ```
pub fn encrypt_message(message: &[u8], state: &RatchetState) -> Result<Vec<u8>> {
    // Implementation
}
```

## 🔐 Security

### Reporting Vulnerabilities

**DO NOT open public issues for security vulnerabilities.**

Instead, email: **security@invisible.im** (coming soon)

Include:
- Description of the vulnerability
- Steps to reproduce
- Impact assessment
- Suggested fix (optional)

We'll respond within 48 hours and work with you to fix it.

### Security Guidelines

- Never commit secrets, keys, or credentials
- Use constant-time operations for crypto
- Zeroize sensitive data after use
- Assume adversaries have nation-state capabilities
- Test against timing attacks

## 🎨 Design Guidelines

### UI/UX Principles

1. **Privacy by default** - Never "enable privacy mode"
2. **Zero decisions** - Users approve, don't configure
3. **Transparent errors** - Teach users when things fail
4. **Progressive disclosure** - Simple for beginners, powerful for experts
5. **Trust & professionalism** - Business-appropriate aesthetics

### Visual Identity

- **Primary color:** Purple `#6C63FF`
- **Security indicators:** Green `#00C853`
- **Typography:** Inter font family
- **Icons:** Outlined style, consistent weight

## 🤔 Questions?

- **Discord:** (coming soon)
- **GitHub Discussions:** [Start a discussion](https://github.com/Light-Brands/invisible/discussions)
- **Email:** hello@invisible.im (coming soon)

## 📜 Code of Conduct

### Our Pledge

We are committed to providing a welcoming and inclusive environment for all contributors, regardless of:
- Age, body size, disability, ethnicity
- Gender identity and expression
- Level of experience
- Nationality, personal appearance, race, religion
- Sexual identity and orientation

### Our Standards

**Examples of behavior that contributes to a positive environment:**
- Being respectful of differing viewpoints
- Gracefully accepting constructive criticism
- Focusing on what is best for the community
- Showing empathy towards other community members

**Examples of unacceptable behavior:**
- Trolling, insulting/derogatory comments, personal or political attacks
- Public or private harassment
- Publishing others' private information without permission
- Other conduct which could reasonably be considered inappropriate

### Enforcement

Instances of abusive, harassing, or otherwise unacceptable behavior may be reported to the project team. All complaints will be reviewed and investigated promptly and fairly.

## 🙏 Thank You!

Every contribution, no matter how small, makes Invisible better for everyone. Thank you for being part of this mission to protect human privacy.

**Together, we're building a more private internet.**

---

**License:** MIT
**Copyright:** 2026 Invisible Project Contributors
