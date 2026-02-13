import '../ffi/api/wallet.dart';

/// Dart wrapper for wallet FFI functions
///
/// Provides a clean interface to Rust-based BIP39 wallet operations.
/// Supports XMR, ZEC, BTC, and ETH with HD wallet derivation.
class WalletBridge {
  /// Generate a new 12-word BIP39 mnemonic
  ///
  /// Returns a 12-word mnemonic phrase suitable for wallet backup.
  /// The mnemonic should be stored securely and never exposed.
  static Future<String> generateMnemonic() async {
    return await walletGenerateMnemonic();
  }

  /// Restore wallet from mnemonic phrase
  ///
  /// Validates the BIP39 mnemonic and returns true if valid.
  /// Invalid mnemonics will return false.
  static Future<bool> restoreFromMnemonic(String mnemonic) async {
    return await walletRestoreFromMnemonic(mnemonic: mnemonic);
  }

  /// Get balances for all supported currencies
  ///
  /// Returns balances for XMR, ZEC, BTC, and ETH.
  /// NOTE: Currently returns placeholder data. Blockchain integration pending.
  static Future<List<WalletBalance>> getBalances(String mnemonic) async {
    return await walletGetBalances(mnemonic: mnemonic);
  }

  /// Generate a receiving address for a specific currency
  ///
  /// Uses HD wallet derivation (BIP44) to generate addresses.
  /// - mnemonic: The wallet's BIP39 mnemonic
  /// - currency: Currency symbol (XMR, ZEC, BTC, ETH)
  /// - account: Account index for HD derivation (default: 0)
  ///
  /// NOTE: Currently returns placeholder addresses. HD derivation pending.
  static Future<String> generateAddress(
    String mnemonic,
    String currency,
    int account,
  ) async {
    return await walletGenerateAddress(
      mnemonic: mnemonic,
      currency: currency,
      account: account,
    );
  }

  /// Get transaction history for a currency
  ///
  /// Returns all transactions for the specified currency.
  /// NOTE: Currently returns empty list. Blockchain integration pending.
  static Future<List<WalletTransaction>> getTransactions(
    String mnemonic,
    String currency,
  ) async {
    return await walletGetTransactions(
      mnemonic: mnemonic,
      currency: currency,
    );
  }
}
