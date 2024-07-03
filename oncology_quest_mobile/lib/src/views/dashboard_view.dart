import 'package:flutter/material.dart';
import 'package:oncology_quest_mobile/src/state/session_state.dart';
import 'package:oncology_quest_mobile/src/widgets/default_profile_icon.dart';
import 'package:provider/provider.dart';

class DashboardView extends StatelessWidget {
  const DashboardView({super.key});

  @override
  Widget build(BuildContext context) {
    final session = Provider.of<SessionState>(context).session!;

    return Scaffold(
      appBar: AppBar(
        automaticallyImplyLeading: false,
        backgroundColor: Theme.of(context).scaffoldBackgroundColor,
        title: Row(
          children: <Widget>[
            DefaultProfileIcon(name: session.user.name),
            const SizedBox(width: 10),
            RichText(
              text: TextSpan(
                style: TextStyle(
                  fontSize: MediaQuery.of(context).size.width * 0.05,
                  color: Theme.of(context).textTheme.bodySmall!.color,
                ),
                children: <TextSpan>[
                  TextSpan(
                    text: session.user.name,
                    style: const TextStyle(fontWeight: FontWeight.bold)
                  ),
                  TextSpan(text: ' (${session.user.username})')
                ]
              )
            )
          ]
        )
      ),
      body: Center(
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: <Widget>[
            Text('Logged in as ${session.user.name}'),
          ]
        )
      )
    );
  }
}