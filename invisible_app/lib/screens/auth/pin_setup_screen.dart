import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../../theme/app_theme.dart';
import '../../providers/auth_provider.dart';

class PinSetupScreen extends ConsumerStatefulWidget {
  const PinSetupScreen({super.key});

  @override
  ConsumerState<PinSetupScreen> createState() => _PinSetupScreenState();
}

class _PinSetupScreenState extends ConsumerState<PinSetupScreen> {
  // TODO: SECURITY - PIN stored in plain Dart String cannot be wiped from memory
  // Limitation: Dart strings are immutable and GC-managed, so we can't zeroize them
  // Mitigation: Minimize exposure time, consider streaming Argon2 hashing per digit
  String _pin = '';
  String? _confirmPin;
  bool _isConfirming = false;

  void _onNumberPress(String number) {
    setState(() {
      if (_isConfirming) {
        if ((_confirmPin ?? '').length < 6) {
          _confirmPin = (_confirmPin ?? '') + number;
          if ((_confirmPin ?? '').length == 6) {
            _verifyPin();
          }
        }
      } else {
        if (_pin.length < 6) {
          _pin += number;
          if (_pin.length == 6) {
            _isConfirming = true;
            _confirmPin = '';
          }
        }
      }
    });
  }

  void _onBackspace() {
    setState(() {
      if (_isConfirming) {
        if ((_confirmPin ?? '').isNotEmpty) {
          _confirmPin = (_confirmPin ?? '').substring(0, (_confirmPin ?? '').length - 1);
        }
      } else {
        if (_pin.isNotEmpty) {
          _pin = _pin.substring(0, _pin.length - 1);
        }
      }
    });
  }

  Future<void> _verifyPin() async {
    if (_pin == _confirmPin) {
      try {
        await ref.read(authProvider.notifier).setPin(_pin);
      } catch (e) {
        if (mounted) {
          ScaffoldMessenger.of(context).showSnackBar(
            SnackBar(
              content: Text('Error setting PIN: $e'),
              backgroundColor: AppTheme.errorRed,
            ),
          );
          setState(() {
            _pin = '';
            _confirmPin = null;
            _isConfirming = false;
          });
        }
      }
    } else {
      ScaffoldMessenger.of(context).showSnackBar(
        const SnackBar(
          content: Text('PINs do not match. Please try again.'),
          backgroundColor: AppTheme.errorRed,
        ),
      );
      setState(() {
        _pin = '';
        _confirmPin = null;
        _isConfirming = false;
      });
    }
  }

  @override
  Widget build(BuildContext context) {
    final currentPin = _isConfirming ? (_confirmPin ?? '') : _pin;

    return Scaffold(
      appBar: AppBar(
        title: Text(_isConfirming ? 'Confirm PIN' : 'Create PIN'),
      ),
      body: SafeArea(
        child: Column(
          children: [
            const Spacer(),

            // Title
            const Text(
              'Create a 6-digit PIN',
              style: TextStyle(
                fontSize: 24,
                fontWeight: FontWeight.bold,
                color: AppTheme.textPrimary,
              ),
            ),

            const SizedBox(height: 8),

            Text(
              _isConfirming
                  ? 'Confirm your PIN'
                  : 'This PIN will be required to unlock the app',
              style: const TextStyle(
                fontSize: 14,
                color: AppTheme.textSecondary,
              ),
              textAlign: TextAlign.center,
            ),

            const SizedBox(height: 40),

            // PIN dots
            Row(
              mainAxisAlignment: MainAxisAlignment.center,
              children: List.generate(6, (index) {
                final isFilled = index < currentPin.length;
                return Container(
                  margin: const EdgeInsets.symmetric(horizontal: 8),
                  width: 16,
                  height: 16,
                  decoration: BoxDecoration(
                    shape: BoxShape.circle,
                    color: isFilled ? AppTheme.primaryPurple : AppTheme.surface,
                    border: Border.all(
                      color: isFilled ? AppTheme.primaryPurple : AppTheme.textSecondary,
                      width: 2,
                    ),
                  ),
                );
              }),
            ),

            const Spacer(),

            // Numpad
            Padding(
              padding: const EdgeInsets.all(24.0),
              child: Column(
                children: [
                  _buildNumRow(['1', '2', '3']),
                  const SizedBox(height: 16),
                  _buildNumRow(['4', '5', '6']),
                  const SizedBox(height: 16),
                  _buildNumRow(['7', '8', '9']),
                  const SizedBox(height: 16),
                  _buildNumRow(['', '0', '⌫']),
                ],
              ),
            ),

            const SizedBox(height: 32),
          ],
        ),
      ),
    );
  }

  Widget _buildNumRow(List<String> numbers) {
    return Row(
      mainAxisAlignment: MainAxisAlignment.spaceEvenly,
      children: numbers.map((number) {
        if (number.isEmpty) {
          return const SizedBox(width: 80, height: 80);
        }

        return InkWell(
          onTap: () {
            if (number == '⌫') {
              _onBackspace();
            } else {
              _onNumberPress(number);
            }
          },
          borderRadius: BorderRadius.circular(40),
          child: Container(
            width: 80,
            height: 80,
            decoration: BoxDecoration(
              shape: BoxShape.circle,
              color: AppTheme.surface,
            ),
            child: Center(
              child: Text(
                number,
                style: const TextStyle(
                  fontSize: 28,
                  fontWeight: FontWeight.w500,
                  color: AppTheme.textPrimary,
                ),
              ),
            ),
          ),
        );
      }).toList(),
    );
  }
}
