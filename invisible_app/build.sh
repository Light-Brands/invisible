#!/bin/bash
set -e

echo "🔨 Building Invisible FFI libraries..."

# Colors for output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Check if Flutter is available (optional - bindings can be pre-generated)
if ! command -v flutter &> /dev/null; then
    echo -e "${YELLOW}⚠️  Flutter not found - will use existing Dart bindings${NC}"
    echo -e "${YELLOW}   Install Flutter to regenerate bindings: https://flutter.dev/docs/get-started/install${NC}"
    echo ""
fi

# Build for macOS (Apple Silicon + Intel)
echo -e "${YELLOW}Building for macOS...${NC}"
cd rust
cargo build --release
cd ..

# Copy to macOS Runner
echo -e "${YELLOW}Copying macOS library...${NC}"
mkdir -p macos/Runner
cp rust/target/release/libinvisible_bridge.dylib macos/Runner/ || echo "Warning: macOS library not found"

echo -e "${GREEN}✅ macOS build complete${NC}"

# Build for iOS (requires iOS toolchain)
if command -v rustup target list | grep -q "aarch64-apple-ios (installed)"; then
    echo -e "${YELLOW}Building for iOS...${NC}"
    cd rust

    # Build for iOS device (ARM64)
    cargo build --release --target aarch64-apple-ios

    # Build for iOS simulator (ARM64 for M1/M2 Macs)
    cargo build --release --target aarch64-apple-ios-sim

    # Build for iOS simulator (x86_64 for Intel Macs)
    cargo build --release --target x86_64-apple-ios

    cd ..

    # Create universal library for simulators
    mkdir -p ios/Runner
    lipo -create \
        rust/target/aarch64-apple-ios-sim/release/libinvisible_bridge.a \
        rust/target/x86_64-apple-ios/release/libinvisible_bridge.a \
        -output ios/Runner/libinvisible_bridge.a 2>/dev/null || \
        cp rust/target/aarch64-apple-ios/release/libinvisible_bridge.a ios/Runner/ || \
        echo "Warning: iOS library not found"

    echo -e "${GREEN}✅ iOS build complete${NC}"
else
    echo -e "${YELLOW}⚠️  iOS targets not installed. Run: rustup target add aarch64-apple-ios aarch64-apple-ios-sim x86_64-apple-ios${NC}"
fi

# Build for Android (requires NDK)
if [ -n "$ANDROID_NDK_HOME" ] || [ -n "$NDK_HOME" ]; then
    echo -e "${YELLOW}Building for Android...${NC}"
    cd rust

    # Build for Android ARM64
    cargo build --release --target aarch64-linux-android 2>/dev/null || echo "Warning: Android ARM64 build failed"

    # Build for Android ARMv7
    cargo build --release --target armv7-linux-androideabi 2>/dev/null || echo "Warning: Android ARMv7 build failed"

    # Build for Android x86_64
    cargo build --release --target x86_64-linux-android 2>/dev/null || echo "Warning: Android x86_64 build failed"

    # Build for Android x86
    cargo build --release --target i686-linux-android 2>/dev/null || echo "Warning: Android x86 build failed"

    cd ..

    # Copy to Android jniLibs
    mkdir -p android/app/src/main/jniLibs/{arm64-v8a,armeabi-v7a,x86,x86_64}
    cp rust/target/aarch64-linux-android/release/libinvisible_bridge.so android/app/src/main/jniLibs/arm64-v8a/ 2>/dev/null || true
    cp rust/target/armv7-linux-androideabi/release/libinvisible_bridge.so android/app/src/main/jniLibs/armeabi-v7a/ 2>/dev/null || true
    cp rust/target/i686-linux-android/release/libinvisible_bridge.so android/app/src/main/jniLibs/x86/ 2>/dev/null || true
    cp rust/target/x86_64-linux-android/release/libinvisible_bridge.so android/app/src/main/jniLibs/x86_64/ 2>/dev/null || true

    echo -e "${GREEN}✅ Android build complete${NC}"
else
    echo -e "${YELLOW}⚠️  Android NDK not found. Set ANDROID_NDK_HOME or NDK_HOME${NC}"
fi

echo ""
echo -e "${GREEN}🎉 FFI libraries built successfully!${NC}"
echo ""
echo "Libraries created:"
echo "  macOS:   macos/Runner/libinvisible_bridge.dylib"
echo "  iOS:     ios/Runner/libinvisible_bridge.a"
echo "  Android: android/app/src/main/jniLibs/{arm64-v8a,armeabi-v7a,x86,x86_64}/"
