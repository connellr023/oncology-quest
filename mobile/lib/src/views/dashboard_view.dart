import 'package:flutter/material.dart';
import 'package:oncology_quest_mobile/src/state/session_state.dart';
import 'package:oncology_quest_mobile/src/utilities/colors.dart';
import 'package:oncology_quest_mobile/src/widgets/bottom_panel.dart';
import 'package:oncology_quest_mobile/src/widgets/default_profile_icon.dart';
import 'package:provider/provider.dart';

class DashboardView extends StatelessWidget {
  const DashboardView({super.key});

  @override
  Widget build(BuildContext context) {
    final session = Provider.of<SessionState>(context, listen: false).session;

    if (session == null) {
      return const Scaffold(
        body: Center(
          child: CircularProgressIndicator(),
        )
      );
    }

    return Scaffold(
      appBar: AppBar(
        automaticallyImplyLeading: false,
        backgroundColor: Theme.of(context).scaffoldBackgroundColor,
        title: Row(
          children: <Widget>[
            DefaultProfileIcon(
              name: session.user.name,
              onTap: () => _showBottomPanel(context),
            ),
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
        child: Padding(
          padding: const EdgeInsets.all(15),
          child: Column(
            mainAxisAlignment: MainAxisAlignment.start,
            children: <Widget>[
              _buildHeading(context, 'Rotations'),
              SizedBox(
                width: double.infinity,
                child: Wrap(
                  spacing: 10,
                  runSpacing: 10,
                  children: session.rotations.entries.map<Widget>((entry) {
                    return _buildRotationOption(context, entry.value.name);
                  }).toList()
                ),
              ),
              const SizedBox(height: 40),
              _buildHeading(context, 'My Progress'),
            ]
          )
        )
      )
    );
  }

  Widget _buildHeading(BuildContext context, String title) {
    return Column(
      children: [
        SizedBox(
          width: double.infinity,
          child: Text(
            title,
            textAlign: TextAlign.left,
            style: TextStyle(
              color: Theme.of(context).textTheme.bodySmall!.color,
              fontSize: MediaQuery.of(context).size.width * 0.068
            )
          )
        ),
        const SizedBox(height: 13)
      ],
    );
  }

  Widget _buildRotationOption(BuildContext context, String name) {
    const double borderRadius = 18;
    
    return Material(
      color: backgroundColor2,
      borderRadius: BorderRadius.circular(borderRadius),
      child: InkWell(
        splashColor: okColor,
        borderRadius: BorderRadius.circular(borderRadius),
        onTap: () => {},
        child: Container(
          padding: const EdgeInsets.all(17),
          child: Text(
            name,
            style: TextStyle(
              color: textColor,
              fontSize: MediaQuery.of(context).size.width * 0.042
            )
          )
        )
      )
    );
  }

  void _showBottomPanel(BuildContext context) {
    showModalBottomSheet(
      context: context,
      backgroundColor: backgroundColor2,
      builder: (BuildContext context) {
        return const BottomPanel();
      }
    );
  }
}