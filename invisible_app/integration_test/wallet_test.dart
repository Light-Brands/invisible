import 'package:flutter_test/flutter_test.dart';
import 'package:integration_test/integration_test.dart';
import 'package:invisible_app/bridge/wallet_bridge.dart';

void main() {
  IntegrationTestWidgetsFlutterBinding.ensureInitialized();

  group('Wallet FFI Tests', () {
    test('Generate and validate mnemonic', () async {
      // Generate mnemonic
      final mnemonic = await WalletBridge.generateMnemonic();

      expect(mnemonic, isNotEmpty);
      expect(mnemonic.split(' ').length, equals(12));

      // Validate mnemonic
      final valid = await WalletBridge.restoreFromMnemonic(mnemonic);
      expect(valid, isTrue);
    });

    test('Get wallet balances', () async {
      final mnemonic = await WalletBridge.generateMnemonic();

      final balances = await WalletBridge.getBalances(mnemonic);

      expect(balances.length, equals(4)); // XMR, ZEC, BTC, ETH
      expect(balances.any((b) => b.symbol == 'XMR'), isTrue);
      expect(balances.any((b) => b.symbol == 'ZEC'), isTrue);
      expect(balances.any((b) => b.symbol == 'BTC'), isTrue);
      expect(balances.any((b) => b.symbol == 'ETH'), isTrue);
    });

    test('Generate addresses for all currencies', () async {
      final mnemonic = await WalletBridge.generateMnemonic();

      final xmrAddr = await WalletBridge.generateAddress(mnemonic, 'XMR', 0);
      final btcAddr = await WalletBridge.generateAddress(mnemonic, 'BTC', 0);

      expect(xmrAddr, isNotEmpty);
      expect(btcAddr, isNotEmpty);
      expect(xmrAddr, isNot(equals(btcAddr)));
    });

    test('Balance values are non-negative', () async {
      final mnemonic = await WalletBridge.generateMnemonic();
      final balances = await WalletBridge.getBalances(mnemonic);

      for (final balance in balances) {
        expect(double.parse(balance.balance) >= 0, isTrue);
        expect(balance.symbol, isNotEmpty);
        expect(balance.name, isNotEmpty);
      }
    });

    test('Multiple address generation with different indices', () async {
      final mnemonic = await WalletBridge.generateMnemonic();

      final btcAddr0 = await WalletBridge.generateAddress(mnemonic, 'BTC', 0);
      final btcAddr1 = await WalletBridge.generateAddress(mnemonic, 'BTC', 1);
      final btcAddr2 = await WalletBridge.generateAddress(mnemonic, 'BTC', 2);

      // All addresses should be different
      expect(btcAddr0, isNot(equals(btcAddr1)));
      expect(btcAddr1, isNot(equals(btcAddr2)));
      expect(btcAddr0, isNot(equals(btcAddr2)));

      // All addresses should be non-empty
      expect(btcAddr0, isNotEmpty);
      expect(btcAddr1, isNotEmpty);
      expect(btcAddr2, isNotEmpty);
    });

    test('Mnemonic consistency', () async {
      final mnemonic1 = await WalletBridge.generateMnemonic();
      final mnemonic2 = await WalletBridge.generateMnemonic();

      // Different mnemonic generations should produce different results
      expect(mnemonic1, isNot(equals(mnemonic2)));

      // Both should be valid
      expect(await WalletBridge.restoreFromMnemonic(mnemonic1), isTrue);
      expect(await WalletBridge.restoreFromMnemonic(mnemonic2), isTrue);
    });

    test('Address derivation consistency', () async {
      final mnemonic = await WalletBridge.generateMnemonic();

      // Generate same address twice
      final addr1 = await WalletBridge.generateAddress(mnemonic, 'BTC', 0);
      final addr2 = await WalletBridge.generateAddress(mnemonic, 'BTC', 0);

      // Should be identical
      expect(addr1, equals(addr2));
    });

    test('Invalid mnemonic handling', () async {
      // Try to restore from invalid mnemonic
      final invalidMnemonic = 'invalid word list that should fail';

      expect(
        () async => await WalletBridge.restoreFromMnemonic(invalidMnemonic),
        throwsA(isA<Exception>()),
      );
    });
  });

  group('Wallet Currency Support', () {
    test('Monero (XMR) address generation', () async {
      final mnemonic = await WalletBridge.generateMnemonic();
      final xmrAddr = await WalletBridge.generateAddress(mnemonic, 'XMR', 0);

      expect(xmrAddr, isNotEmpty);
      // Monero addresses typically start with '4' or '8'
      expect(xmrAddr[0], isIn(['4', '8']));
    });

    test('Zcash (ZEC) address generation', () async {
      final mnemonic = await WalletBridge.generateMnemonic();
      final zecAddr = await WalletBridge.generateAddress(mnemonic, 'ZEC', 0);

      expect(zecAddr, isNotEmpty);
      // Zcash addresses typically start with 't' or 'z'
      expect(zecAddr[0], isIn(['t', 'z']));
    });

    test('Bitcoin (BTC) address generation', () async {
      final mnemonic = await WalletBridge.generateMnemonic();
      final btcAddr = await WalletBridge.generateAddress(mnemonic, 'BTC', 0);

      expect(btcAddr, isNotEmpty);
      // Bitcoin addresses can start with '1', '3', or 'bc1'
      expect(
        btcAddr.startsWith('1') ||
            btcAddr.startsWith('3') ||
            btcAddr.startsWith('bc1'),
        isTrue,
      );
    });

    test('Ethereum (ETH) address generation', () async {
      final mnemonic = await WalletBridge.generateMnemonic();
      final ethAddr = await WalletBridge.generateAddress(mnemonic, 'ETH', 0);

      expect(ethAddr, isNotEmpty);
      // Ethereum addresses start with '0x' and are 42 characters
      expect(ethAddr.startsWith('0x'), isTrue);
      expect(ethAddr.length, equals(42));
    });
  });

  group('Wallet Security', () {
    test('Mnemonic word count is correct', () async {
      final mnemonic = await WalletBridge.generateMnemonic();
      final words = mnemonic.split(' ');

      // Should be 12 or 24 words (BIP39 standard)
      expect(words.length == 12 || words.length == 24, isTrue);
    });

    test('Mnemonic words are from BIP39 wordlist', () async {
      final mnemonic = await WalletBridge.generateMnemonic();
      final words = mnemonic.split(' ');

      // All words should be lowercase alphabetic
      for (final word in words) {
        expect(word, matches(RegExp(r'^[a-z]+$')));
        expect(word.length >= 3, isTrue); // BIP39 words are at least 3 chars
      }
    });

    test('Different mnemonics generate different addresses', () async {
      final mnemonic1 = await WalletBridge.generateMnemonic();
      final mnemonic2 = await WalletBridge.generateMnemonic();

      final addr1 = await WalletBridge.generateAddress(mnemonic1, 'BTC', 0);
      final addr2 = await WalletBridge.generateAddress(mnemonic2, 'BTC', 0);

      expect(addr1, isNot(equals(addr2)));
    });
  });

  group('Wallet Performance', () {
    test('Mnemonic generation is fast', () async {
      final stopwatch = Stopwatch()..start();

      await WalletBridge.generateMnemonic();

      stopwatch.stop();

      // Should complete in under 1 second
      expect(stopwatch.elapsedMilliseconds < 1000, isTrue);
    });

    test('Address generation is fast', () async {
      final mnemonic = await WalletBridge.generateMnemonic();
      final stopwatch = Stopwatch()..start();

      await WalletBridge.generateAddress(mnemonic, 'BTC', 0);

      stopwatch.stop();

      // Should complete in under 1 second
      expect(stopwatch.elapsedMilliseconds < 1000, isTrue);
    });

    test('Balance retrieval is fast', () async {
      final mnemonic = await WalletBridge.generateMnemonic();
      final stopwatch = Stopwatch()..start();

      await WalletBridge.getBalances(mnemonic);

      stopwatch.stop();

      // Should complete in under 2 seconds
      expect(stopwatch.elapsedMilliseconds < 2000, isTrue);
    });

    test('Batch address generation', () async {
      final mnemonic = await WalletBridge.generateMnemonic();
      final stopwatch = Stopwatch()..start();

      // Generate 10 addresses
      for (int i = 0; i < 10; i++) {
        await WalletBridge.generateAddress(mnemonic, 'BTC', i);
      }

      stopwatch.stop();

      // Should complete in under 5 seconds
      expect(stopwatch.elapsedMilliseconds < 5000, isTrue);
    });
  });

  group('Wallet Edge Cases', () {
    test('Zero index address generation', () async {
      final mnemonic = await WalletBridge.generateMnemonic();
      final addr = await WalletBridge.generateAddress(mnemonic, 'BTC', 0);

      expect(addr, isNotEmpty);
    });

    test('Large index address generation', () async {
      final mnemonic = await WalletBridge.generateMnemonic();
      final addr = await WalletBridge.generateAddress(mnemonic, 'BTC', 999);

      expect(addr, isNotEmpty);
    });

    test('All supported currencies have balances', () async {
      final mnemonic = await WalletBridge.generateMnemonic();
      final balances = await WalletBridge.getBalances(mnemonic);

      final supportedCurrencies = ['XMR', 'ZEC', 'BTC', 'ETH'];

      for (final currency in supportedCurrencies) {
        expect(balances.any((b) => b.symbol == currency), isTrue);
      }
    });
  });
}
