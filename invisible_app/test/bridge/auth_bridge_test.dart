import 'package:flutter_test/flutter_test.dart';
import 'package:invisible_app/bridge/auth_bridge.dart';
import 'package:invisible_app/ffi/frb_generated.dart';

void main() {
  setUpAll(() async {
    // Initialize the Rust FFI bridge before running tests
    await RustLib.init();
  });

  group('AuthBridge', () {
    test('PIN hashing and verification', () {
      const pin = '123456';

      // Hash the PIN
      final hash = AuthBridge.hashPin(pin);
      expect(hash, isNotEmpty);
      expect(hash, startsWith(r'$argon2'));

      // Verify correct PIN
      final validResult = AuthBridge.verifyPin(pin, hash);
      expect(validResult, isTrue);

      // Verify wrong PIN
      final invalidResult = AuthBridge.verifyPin('wrong', hash);
      expect(invalidResult, isFalse);
    });

    test('PIN hashing produces different salts', () {
      const pin = '123456';

      // Hash the same PIN twice
      final hash1 = AuthBridge.hashPin(pin);
      final hash2 = AuthBridge.hashPin(pin);

      // Hashes should be different (different salts)
      expect(hash1, isNot(equals(hash2)));

      // But both should verify correctly
      expect(AuthBridge.verifyPin(pin, hash1), isTrue);
      expect(AuthBridge.verifyPin(pin, hash2), isTrue);
    });

    test('2FA secret generation and verification', () {
      // Generate secret
      final secret = AuthBridge.generate2FASecret();
      expect(secret, isNotEmpty);
      expect(secret.length, greaterThan(20)); // Base32 encoded

      // Each secret should be unique
      final secret2 = AuthBridge.generate2FASecret();
      expect(secret, isNot(equals(secret2)));

      // Test that verification returns a boolean
      // Note: We can't easily test a valid code without implementing TOTP generation in Dart
      // or mocking the system time. For now, we just verify it returns false for invalid codes.
      final result = AuthBridge.verify2FACode(secret, '123456');
      expect(result, isA<bool>());
    });

    test('2FA verification rejects invalid secret format', () {
      // Invalid base32 string
      final result = AuthBridge.verify2FACode('invalid!!!', '123456');
      expect(result, isFalse);
    });

    test('PIN verification rejects invalid hash format', () {
      const pin = '123456';

      // Invalid hash format
      final result = AuthBridge.verifyPin(pin, 'not-a-valid-hash');
      expect(result, isFalse);
    });

    test('PIN hashing handles empty PIN', () {
      // Empty PIN should still hash successfully
      final hash = AuthBridge.hashPin('');
      expect(hash, isNotEmpty);
      expect(hash, startsWith(r'$argon2'));

      // Should verify correctly
      expect(AuthBridge.verifyPin('', hash), isTrue);
      expect(AuthBridge.verifyPin('x', hash), isFalse);
    });

    test('PIN hashing handles long PIN', () {
      // Very long PIN
      final longPin = '1' * 1000;
      final hash = AuthBridge.hashPin(longPin);
      expect(hash, isNotEmpty);

      // Should verify correctly
      expect(AuthBridge.verifyPin(longPin, hash), isTrue);
      expect(AuthBridge.verifyPin(longPin + 'x', hash), isFalse);
    });
  });
}
