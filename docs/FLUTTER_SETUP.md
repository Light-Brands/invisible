# Flutter Frontend Setup Guide

**For Invisible Contributors & Developers**

This guide walks through setting up the Flutter development environment for building the Invisible mobile and desktop applications.

---

## Prerequisites

- **macOS:** 10.14 or higher
- **Xcode:** Latest version (for iOS)
- **Android Studio:** Latest version (for Android)
- **Git:** Installed
- **Rust:** Already installed (for Invisible core)

---

## Step 1: Install Flutter

### macOS / Linux

```bash
# Clone Flutter repository
cd ~/development
git clone https://github.com/flutter/flutter.git -b stable

# Add Flutter to PATH
echo 'export PATH="$PATH:$HOME/development/flutter/bin"' >> ~/.zshrc
source ~/.zshrc

# Verify installation
flutter --version
```

### Windows

1. Download Flutter SDK: https://docs.flutter.dev/get-started/install/windows
2. Extract to `C:\src\flutter`
3. Add to PATH: `C:\src\flutter\bin`

---

## Step 2: Run Flutter Doctor

```bash
flutter doctor -v
```

**This checks for:**
- ✓ Flutter SDK
- ✓ Xcode (iOS development)
- ✓ Android Studio
- ✓ VS Code / Android Studio plugins
- ✓ Connected devices

**Fix any issues reported before proceeding.**

---

## Step 3: Install Platform SDKs

### iOS (macOS only)

```bash
# Install Xcode from App Store
# Then install command line tools
sudo xcode-select --switch /Applications/Xcode.app/Contents/Developer
sudo xcodebuild -runFirstLaunch

# Accept license
sudo xcodebuild -license accept

# Install CocoaPods (for dependencies)
sudo gem install cocoapods
```

### Android (All platforms)

```bash
# Install Android Studio
# Download from: https://developer.android.com/studio

# During setup, install:
# - Android SDK
# - Android SDK Platform
# - Android Virtual Device

# Accept licenses
flutter doctor --android-licenses
```

### Desktop (Optional)

**macOS:**
Already supported, no additional setup.

**Linux:**
```bash
sudo apt-get install clang cmake ninja-build pkg-config libgtk-3-dev
```

**Windows:**
Visual Studio 2022 with "Desktop development with C++" workload.

---

## Step 4: Create Flutter Project

```bash
cd invisible

# Create Flutter app in project
flutter create invisible_app \
  --org im.invisible \
  --platforms ios,android,macos,windows,linux \
  --description "Privacy-first messenger with zero-trust architecture"

cd invisible_app
```

---

## Step 5: Configure FFI Integration

### Link Rust Libraries

**iOS (ios/Podfile):**
```ruby
platform :ios, '13.0'

post_install do |installer|
  installer.pods_project.targets.each do |target|
    target.build_configurations.each do |config|
      config.build_settings['ENABLE_BITCODE'] = 'NO'
    end
  end
end

# Link Invisible Rust library
pod 'InvisibleCore', :path => '../../target/release'
```

**Android (android/app/build.gradle):**
```gradle
android {
    defaultConfig {
        minSdkVersion 23
        targetSdkVersion 33

        ndk {
            abiFilters 'arm64-v8a', 'armeabi-v7a', 'x86_64'
        }
    }
}

dependencies {
    implementation files('../../target/release/libinvisible_client_ffi.so')
}
```

**macOS (macos/Runner/Info.plist):**
Add signing capabilities for network access.

---

## Step 6: Add Dependencies

**pubspec.yaml:**
```yaml
name: invisible_app
description: Privacy-first messenger
version: 0.1.0+1

environment:
  sdk: '>=3.0.0 <4.0.0'

dependencies:
  flutter:
    sdk: flutter

  # State Management
  flutter_riverpod: ^2.4.0

  # FFI
  ffi: ^2.1.0

  # Storage
  hive: ^2.2.3
  hive_flutter: ^1.1.0

  # UI
  cupertino_icons: ^1.0.6

dev_dependencies:
  flutter_test:
    sdk: flutter
  flutter_lints: ^3.0.0

flutter:
  uses-material-design: true
```

---

## Step 7: Build Rust FFI Library

**Before running Flutter, build the Rust FFI bindings:**

```bash
cd ../  # Back to project root

# Build FFI library for current platform
cargo build --release -p invisible-client-ffi

# For iOS (additional architectures)
cargo build --release --target aarch64-apple-ios -p invisible-client-ffi
cargo build --release --target x86_64-apple-ios -p invisible-client-ffi

# Create universal library (iOS)
lipo -create \
  target/aarch64-apple-ios/release/libinvisible_client_ffi.a \
  target/x86_64-apple-ios/release/libinvisible_client_ffi.a \
  -output ios/libinvisible_ffi.a
```

---

## Step 8: Verify Setup

```bash
cd invisible_app

# Check for issues
flutter doctor -v

# Run on iOS simulator
flutter run -d ios

# Run on Android emulator
flutter run -d android

# Run on macOS
flutter run -d macos
```

---

## Step 9: Development Workflow

### Hot Reload
```bash
# Start app in debug mode
flutter run

# In terminal:
# r - Hot reload
# R - Hot restart
# q - Quit
```

### Testing
```bash
# Run all tests
flutter test

# Run specific test
flutter test test/services/invisible_service_test.dart

# Test with coverage
flutter test --coverage
```

### Building for Release

**iOS:**
```bash
flutter build ios --release
open ios/Runner.xcworkspace  # Upload to App Store via Xcode
```

**Android:**
```bash
flutter build apk --release  # For Play Store
flutter build appbundle --release  # Recommended
```

**Desktop:**
```bash
flutter build macos --release
flutter build windows --release
flutter build linux --release
```

---

## Troubleshooting

### "Rust library not found"
```bash
# Ensure Rust FFI is built
cargo build --release -p invisible-client-ffi

# Copy to correct location
cp target/release/libinvisible_client_ffi.dylib invisible_app/macos/
```

### "FFI function not found"
```bash
# Verify symbols in library
nm -gU target/release/libinvisible_client_ffi.dylib | grep invisible
```

### "CocoaPods error"
```bash
cd invisible_app/ios
pod deintegrate
pod install
```

### "Gradle build failed"
```bash
cd invisible_app/android
./gradlew clean
./gradlew build
```

---

## IDE Setup

### VS Code (Recommended)
```bash
# Install extensions
code --install-extension Dart-Code.dart-code
code --install-extension Dart-Code.flutter

# Open project
code invisible_app/
```

### Android Studio
1. Open `invisible_app/`
2. Install Flutter & Dart plugins
3. Run > Run 'main.dart'

---

## Next Steps

1. ✓ Flutter installed
2. ✓ Project created
3. → Start building UI (see `docs/PHASE_1_WEEK_1.md`)
4. → Create FFI service layer
5. → Build first screen (Conversations)

---

**Ready to build? Let's go! 🚀**
