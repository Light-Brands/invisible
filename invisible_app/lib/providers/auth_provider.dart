import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../models/auth.dart';
import '../bridge/auth_bridge.dart';
import '../services/secure_storage.dart';

/// Provider for authentication state
final authProvider = StateNotifierProvider<AuthNotifier, AuthSession>((ref) {
  return AuthNotifier();
});

class AuthNotifier extends StateNotifier<AuthSession> {
  AuthNotifier() : super(AuthSession()) {
    _checkAuthState();
  }

  /// Check persisted auth state on app startup
  Future<void> _checkAuthState() async {
    final onboardingComplete = await SecureStorage.isOnboardingComplete();
    final pinHash = await SecureStorage.getPin();
    final twoFASecret = await SecureStorage.get2FASecret();

    if (!onboardingComplete) {
      state = state.copyWith(state: AuthState.initial);
    } else if (pinHash == null) {
      state = state.copyWith(state: AuthState.setupPin);
    } else if (twoFASecret == null) {
      state = state.copyWith(state: AuthState.setup2FA);
    } else {
      state = state.copyWith(
        state: AuthState.locked,
        hasCompletedOnboarding: true,
        hasPinSet: true,
        has2FAEnabled: true,
      );
    }
  }

  /// Complete onboarding flow
  Future<void> completeOnboarding() async {
    await SecureStorage.setOnboardingComplete(true);
    state = state.copyWith(
      hasCompletedOnboarding: true,
      state: AuthState.setupPin,
    );
  }

  /// Set PIN using real Argon2 hashing
  Future<void> setPin(String pin) async {
    try {
      final hashedPin = AuthBridge.hashPin(pin);
      await SecureStorage.storePin(hashedPin);

      state = state.copyWith(
        hasPinSet: true,
        state: AuthState.setup2FA,
      );
    } catch (e) {
      // Handle hashing error (e.g., PIN too short)
      throw Exception('Failed to set PIN: $e');
    }
  }

  /// Unlock app with PIN using real Argon2 verification
  Future<bool> unlockWithPin(String pin) async {
    final storedHash = await SecureStorage.getPin();
    if (storedHash == null) return false;

    final valid = AuthBridge.verifyPin(pin, storedHash);
    if (valid) {
      state = state.copyWith(
        state: AuthState.unlocked,
        lastUnlockTime: DateTime.now(),
        failedAttempts: 0,
      );
      return true;
    } else {
      state = state.copyWith(
        failedAttempts: state.failedAttempts + 1,
      );

      // Trigger panic wipe after 5 failed attempts
      if (state.failedAttempts >= 5) {
        await panicWipe();
      }

      return false;
    }
  }

  /// Setup 2FA with real TOTP secret generation
  Future<String> setup2FA() async {
    final secret = AuthBridge.generate2FASecret();
    await SecureStorage.store2FASecret(secret);

    state = state.copyWith(
      has2FAEnabled: true,
    );

    return secret; // Return for QR code generation
  }

  /// Verify 2FA code using real TOTP verification
  Future<bool> verify2FA(String code) async {
    final secret = await SecureStorage.get2FASecret();
    if (secret == null) return false;

    return AuthBridge.verify2FACode(secret, code);
  }

  /// Complete 2FA setup after verification
  Future<void> complete2FASetup() async {
    state = state.copyWith(
      state: AuthState.locked,
    );
  }

  /// Lock the app
  void lock() {
    if (state.isFullySetup) {
      state = state.copyWith(state: AuthState.locked);
    }
  }

  /// Check if should auto-lock
  bool shouldAutoLock(int autoLockMinutes) {
    if (state.lastUnlockTime == null) return false;
    final elapsed = DateTime.now().difference(state.lastUnlockTime!);
    return elapsed.inMinutes >= autoLockMinutes;
  }

  /// Panic wipe - clear all data
  Future<void> panicWipe() async {
    await SecureStorage.clearAll();
    state = AuthSession(); // Reset to initial state
  }

  /// Reset (for demo/testing)
  Future<void> reset() async {
    await SecureStorage.clearAll();
    state = AuthSession(state: AuthState.initial);
  }

  /// Skip setup (DEBUG_MODE only)
  void skipSetup() {
    state = state.copyWith(
      hasCompletedOnboarding: true,
      hasPinSet: true,
      has2FAEnabled: true,
      state: AuthState.unlocked,
      lastUnlockTime: DateTime.now(),
    );
  }
}
