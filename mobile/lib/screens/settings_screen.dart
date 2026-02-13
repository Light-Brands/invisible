import 'package:flutter/material.dart';

/// Settings screen
class SettingsScreen extends StatelessWidget {
  const SettingsScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Settings'),
      ),
      body: ListView(
        children: [
          // Account section
          _sectionHeader('Account'),
          ListTile(
            leading: const Icon(Icons.person),
            title: const Text('Profile'),
            subtitle: const Text('Manage your profile'),
            onTap: () {},
          ),
          ListTile(
            leading: const Icon(Icons.key),
            title: const Text('Identity Key'),
            subtitle: const Text('View your identity key'),
            onTap: () {},
          ),
          ListTile(
            leading: const Icon(Icons.backup),
            title: const Text('Backup & Restore'),
            subtitle: const Text('Backup your keys and messages'),
            onTap: () {},
          ),
          
          const Divider(),
          
          // Privacy section
          _sectionHeader('Privacy & Security'),
          ListTile(
            leading: const Icon(Icons.lock),
            title: const Text('2FA Settings'),
            subtitle: const Text('Manage two-factor authentication'),
            onTap: () {},
          ),
          ListTile(
            leading: const Icon(Icons.security),
            title: const Text('Burn Room Timer'),
            subtitle: const Text('Default self-destruct timer'),
            trailing: const Text('24h'),
            onTap: () {},
          ),
          ListTile(
            leading: const Icon(Icons.vpn_key),
            title: const Text('Ghost VPN'),
            subtitle: const Text('Always-on VPN tunnel'),
            trailing: Switch(value: true, onChanged: (v) {}),
          ),
          ListTile(
            leading: const Icon(Icons.warning),
            title: const Text('Panic Wipe'),
            subtitle: const Text('Configure duress PIN'),
            onTap: () {},
          ),
          
          const Divider(),
          
          // Network section
          _sectionHeader('Network'),
          ListTile(
            leading: const Icon(Icons.router),
            title: const Text('Relay Nodes'),
            subtitle: const Text('3 nodes active'),
            onTap: () {},
          ),
          ListTile(
            leading: const Icon(Icons.cloud_off),
            title: const Text('Tor Fallback'),
            subtitle: const Text('Use Tor if mixnet unavailable'),
            trailing: Switch(value: false, onChanged: (v) {}),
          ),
          
          const Divider(),
          
          // App section
          _sectionHeader('Application'),
          ListTile(
            leading: const Icon(Icons.palette),
            title: const Text('Theme'),
            subtitle: const Text('System default'),
            onTap: () {},
          ),
          ListTile(
            leading: const Icon(Icons.notifications),
            title: const Text('Notifications'),
            subtitle: const Text('Manage notification settings'),
            onTap: () {},
          ),
          ListTile(
            leading: const Icon(Icons.info),
            title: const Text('About'),
            subtitle: const Text('Version 0.1.0'),
            onTap: () {},
          ),
        ],
      ),
    );
  }

  Widget _sectionHeader(String title) {
    return Padding(
      padding: const EdgeInsets.fromLTRB(16, 16, 16, 8),
      child: Text(
        title,
        style: const TextStyle(
          fontWeight: FontWeight.bold,
          fontSize: 14,
        ),
      ),
    );
  }
}
