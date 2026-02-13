import 'package:flutter/material.dart';

/// Main conversations list screen
class ConversationsScreen extends StatefulWidget {
  const ConversationsScreen({super.key});

  @override
  State<ConversationsScreen> createState() => _ConversationsScreenState();
}

class _ConversationsScreenState extends State<ConversationsScreen> {
  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Invisible'),
        actions: [
          IconButton(
            icon: const Icon(Icons.search),
            onPressed: () {
              // TODO: Open search
            },
          ),
          IconButton(
            icon: const Icon(Icons.account_balance_wallet),
            onPressed: () {
              Navigator.pushNamed(context, '/wallet');
            },
          ),
        ],
      ),
      body: ListView.builder(
        itemCount: 0, // TODO: Load conversations
        itemBuilder: (context, index) {
          return ListTile(
            leading: CircleAvatar(
              child: Text('A'),
            ),
            title: Text('Contact Name'),
            subtitle: Text('Last message...'),
            trailing: Text('12:34'),
            onTap: () {
              Navigator.pushNamed(context, '/chat');
            },
          );
        },
      ),
      floatingActionButton: FloatingActionButton(
        onPressed: () {
          Navigator.pushNamed(context, '/new-conversation');
        },
        child: const Icon(Icons.edit),
      ),
    );
  }
}
