import 'package:flutter/material.dart';
import 'package:oncology_quest_mobile/src/state/session_state.dart';
import 'package:oncology_quest_mobile/src/utilities/colors.dart';
import 'package:oncology_quest_mobile/src/widgets/dashboard/panel_option.dart';
import 'package:provider/provider.dart';

class BottomPanel extends StatelessWidget {
  const BottomPanel({super.key});

  void onLogout(BuildContext context) {
    final sessionState = Provider.of<SessionState>(context, listen: false);
    
    Navigator.pushReplacementNamed(context, '/');
    sessionState.logout();
  }

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.all(10),
      child: Column(
        mainAxisSize: MainAxisSize.min,
        children: <Widget>[
          PanelOption(
            text: 'Close',
            icon: Icons.close,
            onTap: () => Navigator.pop(context),
            splashColor: warningColor
          ),
          PanelOption(
            text: 'Delete Account',
            icon: Icons.delete_forever,
            onTap: () => Navigator.pop(context),
            splashColor: errorColor
          ),
          PanelOption(
            text: 'Logout',
            icon: Icons.logout,
            onTap: () => onLogout(context),
            splashColor: okColor
          )
        ],
      ),
    );
  }
}