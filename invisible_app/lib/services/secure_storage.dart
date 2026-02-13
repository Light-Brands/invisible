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

  // SECURITY: Obfuscated key names to reduce attack surface
  // Malware scanning for obvious keys like 'pin_hash' or '2fa_secret' won't find these
  static const String _pinHashKey = 'invisible_a8f3d2e1';
  static const String _2faSecretKey = 'invisible_b9c4e3f2';
  static const String _onboardingKey = 'invisible_c1d5f4a3';

  // PIN storage
  static Future<void> storePin(String hashedPin) async {
    await _storage.write(key: _pinHashKey, value: hashedPin);
  }

  static Future<String?> getPin() async {
    return await _storage.read(key: _pinHashKey);
  }

  // 2FA secret storage
  static Future<void> store2FASecret(String secret) async {
    await _storage.write(key: _2faSecretKey, value: secret);
  }

  static Future<String?> get2FASecret() async {
    return await _storage.read(key: _2faSecretKey);
  }

  // Onboarding status
  static Future<void> setOnboardingComplete(bool complete) async {
    await _storage.write(key: _onboardingKey, value: complete.toString());
  }

  static Future<bool> isOnboardingComplete() async {
    final value = await _storage.read(key: _onboardingKey);
    return value == 'true';
  }

  // Rate limiting for failed PIN attempts
  static Future<void> recordFailedAttempt() async {
    final now = DateTime.now().millisecondsSinceEpoch.toString();
    await _storage.write(key: 'last_failed_attempt', value: now);

    // Increment failed attempts counter
    final currentStr = await _storage.read(key: 'failed_attempts');
    final current = currentStr != null ? int.parse(currentStr) : 0;
    await _storage.write(key: 'failed_attempts', value: (current + 1).toString());
  }

  static Future<int> getRequiredWaitSeconds() async {
    final lastAttemptStr = await _storage.read(key: 'last_failed_attempt');
    if (lastAttemptStr == null) return 0;

    final lastAttempt = DateTime.fromMillisecondsSinceEpoch(
      int.parse(lastAttemptStr),
    );
    final elapsed = DateTime.now().difference(lastAttempt);

    // Exponential backoff: 2^attempts seconds (max 60s)
    // 1st fail: 1s, 2nd: 2s, 3rd: 4s, 4th: 8s, 5th+: capped at 60s
    final failedStr = await _storage.read(key: 'failed_attempts');
    final failed = failedStr != null ? int.parse(failedStr) : 0;
    final requiredWait = (1 << failed).clamp(0, 60);

    final remaining = requiredWait - elapsed.inSeconds;
    return remaining > 0 ? remaining : 0;
  }

  static Future<void> clearFailedAttempts() async {
    await _storage.delete(key: 'failed_attempts');
    await _storage.delete(key: 'last_failed_attempt');
  }

  // Clear all (for panic wipe)
  static Future<void> clearAll() async {
    await _storage.deleteAll();
  }
}
