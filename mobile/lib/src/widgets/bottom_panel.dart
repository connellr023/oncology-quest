import 'package:flutter/material.dart';
import 'package:oncology_quest_mobile/src/state/session_state.dart';
import 'package:oncology_quest_mobile/src/utilities/colors.dart';
import 'package:provider/provider.dart';

class BottomPanel extends StatelessWidget {
  const BottomPanel({super.key});

  Widget _buildOption(String text, IconData icon, Function() onTap, {Color splashColor = textColor}) {
    return Material(
      color: Colors.transparent, // Use the appropriate background color
      child: InkWell(
        splashColor: splashColor,
        borderRadius: BorderRadius.circular(20), // Set the border radius for the splash effect
        onTap: onTap,
        child: ListTile(
          leading: Icon(icon, color: textColor),
          title: Text(
            text,
            style: const TextStyle(color: textColor),
          ),
        ),
      )
    );
  }

  void onLogout(BuildContext context) {
    final sessionState = Provider.of<SessionState>(context, listen: false);
    
    Navigator.pushReplacementNamed(context, '/');
    sessionState.logout();
  }

  @override
  Widget build(BuildContext context) {
    return SizedBox(
      height: 190,
      child: ListView(
        padding: const EdgeInsets.all(10),
        children: <Widget>[
          _buildOption(
            'Close',
            Icons.close,
            () => Navigator.pop(context),
            splashColor: warningColor
          ),
          _buildOption(
            'Delete Account',
            Icons.delete_forever,
            () => Navigator.pop(context),
            splashColor: errorColor
          ),
          _buildOption(
            'Logout',
            Icons.logout,
            () => onLogout(context),
            splashColor: okColor
          )
        ],
      ),
    );
  }
}