import 'package:flutter_secure_storage/flutter_secure_storage.dart';

/// Secure storage service for sensitive authentication data
///
/// Uses platform-specific secure storage:
/// - iOS/macOS: Keychain
/// - Android: Keystore
/// - Linux: libsecret
/// - Windows: Credential Store
class SecureStorage {
  static const _storage = FlutterSecureStorage(
    aOptions: AndroidOptions(
      encryptedSharedPreferences: true,
    ),
    iOptions: IOSOptions(
      accessibility: KeychainAccessibility.first_unlock_this_device,
    ),
  );

  // PIN storage
  static Future<void> storePin(String hashedPin) async {
    await _storage.write(key: 'pin_hash', value: hashedPin);
  }

  static Future<String?> getPin() async {
    return await _storage.read(key: 'pin_hash');
  }

  // 2FA secret storage
  static Future<void> store2FASecret(String secret) async {
    await _storage.write(key: '2fa_secret', value: secret);
  }

  static Future<String?> get2FASecret() async {
    return await _storage.read(key: '2fa_secret');
  }

  // Onboarding status
  static Future<void> setOnboardingComplete(bool complete) async {
    await _storage.write(
        key: 'onboarding_complete', value: complete.toString());
  }

  static Future<bool> isOnboardingComplete() async {
    final value = await _storage.read(key: 'onboarding_complete');
    return value == 'true';
  }

  // Clear all (for panic wipe)
  static Future<void> clearAll() async {
    await _storage.deleteAll();
  }
}
