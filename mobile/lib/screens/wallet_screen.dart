import 'package:flutter/material.dart';

/// Shadow Wallet screen
class WalletScreen extends StatefulWidget {
  const WalletScreen({super.key});

  @override
  State<WalletScreen> createState() => _WalletScreenState();
}

class _WalletScreenState extends State<WalletScreen> {
  String selectedCurrency = 'XMR';

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Shadow Wallet'),
        actions: [
          IconButton(
            icon: const Icon(Icons.settings),
            onPressed: () {
              Navigator.pushNamed(context, '/wallet/settings');
            },
          ),
        ],
      ),
      body: Column(
        children: [
          // Currency selector
          Container(
            height: 80,
            padding: const EdgeInsets.all(16),
            child: ListView(
              scrollDirection: Axis.horizontal,
              children: [
                _currencyChip('XMR', 'Monero'),
                _currencyChip('ZEC', 'Zcash'),
                _currencyChip('BTC', 'Bitcoin'),
                _currencyChip('ETH', 'Ethereum'),
              ],
            ),
          ),
          
          // Balance card
          Card(
            margin: const EdgeInsets.all(16),
            child: Padding(
              padding: const EdgeInsets.all(24),
              child: Column(
                children: [
                  Text(
                    selectedCurrency,
                    style: Theme.of(context).textTheme.titleLarge,
                  ),
                  const SizedBox(height: 16),
                  Text(
                    '0.00000000',
                    style: Theme.of(context).textTheme.displaySmall?.copyWith(
                          fontWeight: FontWeight.bold,
                        ),
                  ),
                  const SizedBox(height: 8),
                  Text(
                    '\$0.00 USD',
                    style: Theme.of(context).textTheme.bodyLarge,
                  ),
                ],
              ),
            ),
          ),
          
          // Action buttons
          Padding(
            padding: const EdgeInsets.all(16),
            child: Row(
              mainAxisAlignment: MainAxisAlignment.spaceEvenly,
              children: [
                _actionButton(context, Icons.arrow_upward, 'Send'),
                _actionButton(context, Icons.arrow_downward, 'Receive'),
                _actionButton(context, Icons.swap_horiz, 'Swap'),
              ],
            ),
          ),
          
          // Transactions
          Expanded(
            child: ListView.builder(
              itemCount: 0, // TODO: Load transactions
              itemBuilder: (context, index) {
                return ListTile(
                  leading: const CircleAvatar(
                    child: Icon(Icons.arrow_upward),
                  ),
                  title: const Text('Sent'),
                  subtitle: const Text('0.1 XMR'),
                  trailing: const Text('-\$100.00'),
                );
              },
            ),
          ),
        ],
      ),
    );
  }

  Widget _currencyChip(String symbol, String name) {
    final isSelected = selectedCurrency == symbol;
    return Padding(
      padding: const EdgeInsets.only(right: 8),
      child: ChoiceChip(
        label: Text(symbol),
        selected: isSelected,
        onSelected: (selected) {
          if (selected) {
            setState(() {
              selectedCurrency = symbol;
            });
          }
        },
      ),
    );
  }

  Widget _actionButton(BuildContext context, IconData icon, String label) {
    return Column(
      children: [
        FilledButton(
          onPressed: () {
            // TODO: Handle action
          },
          child: Icon(icon),
        ),
        const SizedBox(height: 8),
        Text(label),
      ],
    );
  }
}
