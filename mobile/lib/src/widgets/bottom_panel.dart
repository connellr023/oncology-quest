import 'package:flutter/material.dart';
import 'package:oncology_quest_mobile/src/utilities/colors.dart';

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

  @override
  Widget build(BuildContext context) {
    return SizedBox(
      height: 135,
      child: ListView(
        padding: const EdgeInsets.all(10),
        children: <Widget>[
          _buildOption(
            'Delete Account',
            Icons.delete_forever,
            () => Navigator.pop(context),
            splashColor: errorColor
          ),
          _buildOption(
            'Logout',
            Icons.logout,
            () => Navigator.pop(context),
            splashColor: okColor
          )
        ],
      ),
    );
  }
}