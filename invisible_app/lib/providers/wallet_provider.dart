import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../models/wallet.dart';
import '../bridge/wallet_bridge.dart';
import '../services/secure_storage.dart';

/// Provider for wallet balances
final walletBalancesProvider = StateNotifierProvider<WalletBalancesNotifier, List<CryptoBalance>>((ref) {
  return WalletBalancesNotifier();
});

/// Provider for transaction history
final transactionsProvider = StateNotifierProvider<TransactionsNotifier, List<Transaction>>((ref) {
  return TransactionsNotifier();
});

/// Provider for total portfolio value
final totalPortfolioValueProvider = Provider<double>((ref) {
  final balances = ref.watch(walletBalancesProvider);
  return balances.fold(0.0, (sum, balance) => sum + balance.usdValue);
});

class WalletBalancesNotifier extends StateNotifier<List<CryptoBalance>> {
  WalletBalancesNotifier() : super([]) {
    _loadBalances();
  }

  /// Load wallet balances from blockchain
  Future<void> _loadBalances() async {
    final mnemonic = await SecureStorage.getWalletMnemonic();
    if (mnemonic != null) {
      await refreshBalances();
    }
  }

  /// Refresh balances from blockchain
  Future<void> refreshBalances() async {
    final mnemonic = await SecureStorage.getWalletMnemonic();
    if (mnemonic == null) return;

    try {
      final balances = await WalletBridge.getBalances(mnemonic);
      state = balances.map((b) => CryptoBalance(
        symbol: b.symbol,
        name: b.name,
        balance: b.balance,
        usdValue: b.usdValue,
        iconEmoji: _getIcon(b.symbol),
        isPrivacyCoin: b.isPrivacyCoin,
      )).toList();
    } catch (e) {
      // Handle error
      print('Error loading wallet balances: $e');
    }
  }

  /// Generate new wallet with mnemonic
  Future<String> generateNewWallet() async {
    final mnemonic = await WalletBridge.generateMnemonic();
    await SecureStorage.storeWalletMnemonic(mnemonic);
    await refreshBalances();
    return mnemonic;
  }

  /// Restore wallet from mnemonic
  Future<bool> restoreWallet(String mnemonic) async {
    try {
      final valid = await WalletBridge.restoreFromMnemonic(mnemonic);
      if (valid) {
        await SecureStorage.storeWalletMnemonic(mnemonic);
        await refreshBalances();
        return true;
      }
      return false;
    } catch (e) {
      return false;
    }
  }

  String _getIcon(String symbol) {
    switch (symbol) {
      case 'XMR': return '🔒';
      case 'ZEC': return '🛡️';
      case 'BTC': return '₿';
      case 'ETH': return 'Ξ';
      default: return '💰';
    }
  }
}

class TransactionsNotifier extends StateNotifier<List<Transaction>> {
  TransactionsNotifier() : super([]); // Start with empty transaction history

  void addTransaction(Transaction transaction) {
    state = [transaction, ...state];
  }
}
