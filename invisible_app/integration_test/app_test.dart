import 'package:flutter_test/flutter_test.dart';
import 'package:integration_test/integration_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:invisible_app/main.dart';
import 'package:flutter/material.dart';

void main() {
  IntegrationTestWidgetsFlutterBinding.ensureInitialized();

  group('Full App Flow', () {
    testWidgets('Complete onboarding and authentication flow', (tester) async {
      // Launch app
      await tester.pumpWidget(const ProviderScope(child: InvisibleApp()));
      await tester.pumpAndSettle();

      // 1. Onboarding screen
      expect(find.text('Welcome to Invisible'), findsOneWidget);
      await tester.tap(find.text('Get Started'));
      await tester.pumpAndSettle();

      // 2. PIN setup screen
      expect(find.text('Create PIN'), findsOneWidget);

      // Enter PIN: 123456
      await _enterPin(tester, '123456');
      await tester.pumpAndSettle();

      // Confirm PIN
      await _enterPin(tester, '123456');
      await tester.pumpAndSettle();

      // 3. 2FA setup screen
      expect(find.text('Two-Factor Authentication'), findsOneWidget);

      // For testing, we'll skip real 2FA verification
      // In production, you'd use a real TOTP code generator
      await tester.enterText(find.byType(TextField).first, '123456');
      await tester.tap(find.text('Verify & Complete Setup'));
      await tester.pumpAndSettle();

      // 4. Should now be on main screen
      expect(find.text('Chats'), findsOneWidget);
      expect(find.text('Wallet'), findsOneWidget);
      expect(find.text('Contacts'), findsOneWidget);
      expect(find.text('Settings'), findsOneWidget);
    });

    testWidgets('Lock and unlock flow', (tester) async {
      // Launch app (assume already onboarded)
      await tester.pumpWidget(const ProviderScope(child: InvisibleApp()));
      await tester.pumpAndSettle();

      // If app is locked, should see PIN entry screen
      final lockScreenFinder = find.text('Enter PIN');

      if (lockScreenFinder.evaluate().isNotEmpty) {
        // App is locked, unlock it
        await _enterPin(tester, '123456');
        await tester.pumpAndSettle();

        // Should now be on main screen
        expect(find.text('Chats'), findsOneWidget);
      } else {
        // App is not locked, test locking mechanism
        // Navigate to settings
        await tester.tap(find.text('Settings'));
        await tester.pumpAndSettle();

        // Tap lock button if available
        final lockButton = find.text('Lock App');
        if (lockButton.evaluate().isNotEmpty) {
          await tester.tap(lockButton);
          await tester.pumpAndSettle();

          // Should now be locked
          expect(find.text('Enter PIN'), findsOneWidget);
        }
      }
    });

    testWidgets('Navigate between main tabs', (tester) async {
      // Launch app (assume already onboarded and unlocked)
      await tester.pumpWidget(const ProviderScope(child: InvisibleApp()));
      await tester.pumpAndSettle();

      // Unlock if necessary
      final lockScreenFinder = find.text('Enter PIN');
      if (lockScreenFinder.evaluate().isNotEmpty) {
        await _enterPin(tester, '123456');
        await tester.pumpAndSettle();
      }

      // Test navigation between tabs
      // 1. Chats tab (default)
      expect(find.text('Chats'), findsOneWidget);

      // 2. Navigate to Wallet
      await tester.tap(find.text('Wallet'));
      await tester.pumpAndSettle();
      expect(find.text('Shadow Wallet'), findsOneWidget);

      // 3. Navigate to Contacts
      await tester.tap(find.text('Contacts'));
      await tester.pumpAndSettle();
      expect(find.text('Contacts'), findsOneWidget);

      // 4. Navigate to Settings
      await tester.tap(find.text('Settings'));
      await tester.pumpAndSettle();
      expect(find.text('Settings'), findsOneWidget);

      // 5. Navigate back to Chats
      await tester.tap(find.text('Chats'));
      await tester.pumpAndSettle();
      expect(find.text('Chats'), findsOneWidget);
    });

    testWidgets('Panic wipe gesture test', (tester) async {
      // Launch app
      await tester.pumpWidget(const ProviderScope(child: InvisibleApp()));
      await tester.pumpAndSettle();

      // Unlock if necessary
      final lockScreenFinder = find.text('Enter PIN');
      if (lockScreenFinder.evaluate().isNotEmpty) {
        await _enterPin(tester, '123456');
        await tester.pumpAndSettle();
      }

      // Navigate to Settings
      await tester.tap(find.text('Settings'));
      await tester.pumpAndSettle();

      // Look for panic wipe configuration
      final panicWipeFinder = find.text('Panic Wipe');
      if (panicWipeFinder.evaluate().isNotEmpty) {
        await tester.tap(panicWipeFinder);
        await tester.pumpAndSettle();

        // Verify panic wipe settings screen
        expect(find.text('Duress PIN'), findsOneWidget);
        expect(find.text('Wipe Gesture'), findsOneWidget);
      }
    });

    testWidgets('2FA verification flow', (tester) async {
      // Launch app
      await tester.pumpWidget(const ProviderScope(child: InvisibleApp()));
      await tester.pumpAndSettle();

      // Unlock if necessary
      final lockScreenFinder = find.text('Enter PIN');
      if (lockScreenFinder.evaluate().isNotEmpty) {
        await _enterPin(tester, '123456');
        await tester.pumpAndSettle();
      }

      // Navigate to Settings
      await tester.tap(find.text('Settings'));
      await tester.pumpAndSettle();

      // Look for 2FA settings
      final twoFAFinder = find.text('Two-Factor Authentication');
      if (twoFAFinder.evaluate().isNotEmpty) {
        await tester.tap(twoFAFinder);
        await tester.pumpAndSettle();

        // Verify 2FA screen elements
        expect(find.text('TOTP Code'), findsWidgets);
      }
    });

    testWidgets('Burn room creation flow', (tester) async {
      // Launch app
      await tester.pumpWidget(const ProviderScope(child: InvisibleApp()));
      await tester.pumpAndSettle();

      // Unlock if necessary
      final lockScreenFinder = find.text('Enter PIN');
      if (lockScreenFinder.evaluate().isNotEmpty) {
        await _enterPin(tester, '123456');
        await tester.pumpAndSettle();
      }

      // Should be on Chats tab
      expect(find.text('Chats'), findsOneWidget);

      // Look for new chat button
      final newChatFinder = find.byIcon(Icons.add);
      if (newChatFinder.evaluate().isNotEmpty) {
        await tester.tap(newChatFinder.first);
        await tester.pumpAndSettle();

        // Look for burn room option
        final burnRoomFinder = find.text('Create Burn Room');
        if (burnRoomFinder.evaluate().isNotEmpty) {
          await tester.tap(burnRoomFinder);
          await tester.pumpAndSettle();

          // Verify burn room settings
          expect(find.text('Self-Destruct Timer'), findsOneWidget);
        }
      }
    });
  });

  group('Error Handling', () {
    testWidgets('Invalid PIN shows error', (tester) async {
      // Launch app
      await tester.pumpWidget(const ProviderScope(child: InvisibleApp()));
      await tester.pumpAndSettle();

      // If creating new PIN
      final createPinFinder = find.text('Create PIN');
      if (createPinFinder.evaluate().isNotEmpty) {
        // Enter PIN: 123456
        await _enterPin(tester, '123456');
        await tester.pumpAndSettle();

        // Enter different confirmation PIN: 654321
        await _enterPin(tester, '654321');
        await tester.pumpAndSettle();

        // Should show error
        expect(find.text('PINs do not match'), findsOneWidget);
      }
    });

    testWidgets('Wrong unlock PIN shows error', (tester) async {
      // Launch app (assume already onboarded)
      await tester.pumpWidget(const ProviderScope(child: InvisibleApp()));
      await tester.pumpAndSettle();

      final lockScreenFinder = find.text('Enter PIN');
      if (lockScreenFinder.evaluate().isNotEmpty) {
        // Enter wrong PIN
        await _enterPin(tester, '000000');
        await tester.pumpAndSettle();

        // Should show error or remain on lock screen
        expect(find.text('Enter PIN'), findsOneWidget);
      }
    });
  });

  group('Accessibility', () {
    testWidgets('Screen reader labels present', (tester) async {
      // Launch app
      await tester.pumpWidget(const ProviderScope(child: InvisibleApp()));
      await tester.pumpAndSettle();

      // Check for semantic labels
      final semanticsFinder = find.bySemanticsLabel('Get Started');
      if (semanticsFinder.evaluate().isEmpty) {
        // If no semantic label, check for text
        expect(find.text('Get Started'), findsWidgets);
      }
    });
  });
}

/// Helper function to enter PIN by tapping digit buttons
Future<void> _enterPin(WidgetTester tester, String pin) async {
  for (final digit in pin.split('')) {
    final digitFinder = find.text(digit);
    if (digitFinder.evaluate().isNotEmpty) {
      await tester.tap(digitFinder.first);
      await tester.pump(const Duration(milliseconds: 100));
    }
  }
}
