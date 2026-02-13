# Invisible Mobile

Privacy-first secure messenger mobile app built with Flutter.

## Prerequisites

- Flutter SDK 3.0.0 or higher
- Dart SDK 3.0.0 or higher
- Xcode (for iOS development)
- Android Studio (for Android development)

## Getting Started

### Install Dependencies

```bash
flutter pub get
```

### Run the App

```bash
# Development mode
flutter run

# Release mode
flutter run --release
```

### Run Tests

```bash
# Unit tests
flutter test

# Integration tests
flutter test integration_test
```

## Project Structure

```
mobile/
├── lib/
│   ├── core/          # Core utilities and services
│   ├── data/          # Data layer (repositories, data sources)
│   ├── domain/        # Business logic (entities, use cases)
│   ├── presentation/  # UI layer (screens, widgets, bloc)
│   └── main.dart      # App entry point
├── test/              # Unit tests
├── integration_test/  # Integration tests
└── assets/            # Images, fonts, etc.
```

## Features

- 🔐 End-to-end encryption (X3DH + Double Ratchet)
- 🛡️ Zero-metadata architecture
- 🔒 Secure local storage (SQLCipher)
- 🎭 No phone/email/username required
- 🔥 Burn rooms (self-destructing messages)
- 💳 Shadow Wallet (privacy-first crypto)
- 🎬 Voice/Video calls (WebRTC)
- 🚨 Panic wipe with duress PIN

## Security

- All sensitive data encrypted at rest
- Biometric authentication
- No logging or analytics
- Memory zeroization for keys
- Secure key exchange via QR codes

## Development

### Code Style

This project follows Flutter's official style guide with strict linting rules. Run:

```bash
# Check linting
flutter analyze

# Format code
flutter format .
```

### Building

```bash
# Android APK
flutter build apk --release

# iOS IPA
flutter build ios --release

# App Bundle
flutter build appbundle --release
```

## Platform-Specific Setup

### Android

Minimum SDK: 23 (Android 6.0)
Target SDK: 34 (Android 14)

### iOS

Minimum iOS version: 13.0
Requires face ID / touch ID capabilities

## License

Proprietary - Internal use only
