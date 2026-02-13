import '../ffi/api/auth.dart';

/// Dart wrapper for authentication FFI functions
///
/// Provides a clean interface to Rust-based Argon2 PIN hashing and TOTP 2FA.
class AuthBridge {
  /// Hash a PIN using Argon2id with secure parameters
  ///
  /// Returns the Argon2 hash string suitable for storage.
  /// The hash includes the salt and algorithm parameters.
  static String hashPin(String pin) {
    return authHashPin(pin: pin);
  }

  /// Verify a PIN against a stored Argon2 hash
  ///
  /// Returns true if the PIN matches the stored hash, false otherwise.
  static bool verifyPin(String pin, String storedHash) {
    return authVerifyPin(pin: pin, storedHash: storedHash);
  }

  /// Generate a new TOTP secret (base32 encoded)
  ///
  /// Returns a 20-byte secret encoded in base32 without padding,
  /// suitable for use with authenticator apps.
  static String generate2FASecret() {
    return authGenerate2FaSecret();
  }

  /// Verify a TOTP code against a secret
  ///
  /// Uses a 30-second time window with +/- 1 step tolerance (90 seconds total)
  /// to account for clock drift.
  ///
  /// Returns true if the code is valid, false otherwise.
  static bool verify2FACode(String secret, String code) {
    return authVerify2FaCode(secret: secret, code: code);
  }
}
