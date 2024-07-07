import 'package:flutter/material.dart';
import 'package:oncology_quest_mobile/src/state/selected_rotation_state.dart';
import 'package:oncology_quest_mobile/src/state/session_state.dart';
import 'package:oncology_quest_mobile/src/utilities/colors.dart';
import 'package:oncology_quest_mobile/src/widgets/dashboard/bottom_panel.dart';
import 'package:oncology_quest_mobile/src/widgets/dashboard/dashboard_app_bar.dart';
import 'package:oncology_quest_mobile/src/widgets/dashboard/entries.dart';
import 'package:oncology_quest_mobile/src/widgets/dashboard/graphic.dart';
import 'package:oncology_quest_mobile/src/widgets/dashboard/rotation_select.dart';
import 'package:oncology_quest_mobile/src/widgets/dashboard/section_heading.dart';
import 'package:provider/provider.dart';

class DashboardView extends StatefulWidget {
  const DashboardView({super.key});

  @override
  State<DashboardView> createState() => _DashboardViewState();
}

class _DashboardViewState extends State<DashboardView> {
  void _showBottomPanel(BuildContext context) {
    showModalBottomSheet(
      context: context,
      backgroundColor: backgroundColor2,
      builder: (BuildContext context) {
        return const BottomPanel();
      }
    );
  }

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
      appBar: DashboardAppBar(
        session: session,
        onProfileTap: () => _showBottomPanel(context)
      ),
      body: Padding(
        padding: const EdgeInsets.only(
          left: 15,
          right: 15
        ),
        child: SingleChildScrollView(
          child: Column(
            mainAxisAlignment: MainAxisAlignment.start,
            children: <Widget>[
              const SizedBox(height: 15),
              RotationSelect(session: session),
              Consumer<SelectedRotationState>(
                builder: (context, selectedRotationState, child) => Column(
                  children: <Widget>[
                    if (selectedRotationState.selectedRotationId != null) ...<Widget>[
                      const SizedBox(height: 35),
                      SectionHeading(context: context, title: session.user.isAdmin ? 'Task Entries' : 'My Progress'),
                      Entries(
                        session: session,
                        rotationId: selectedRotationState.selectedRotationId!
                      )
                    ]
                    else ...<Widget>[
                      const SizedBox(height: 60),
                      _buildNoRotationSelected(context)
                    ]
                  ]
                )
              )
            ]
          )
        )
      )
    );
  }

  Widget _buildNoRotationSelected(BuildContext context) {
    return Column(
      children: <Widget>[
        Graphic(imageWidth: MediaQuery.of(context).size.width * 0.65),
        Padding(
          padding: const EdgeInsets.all(20),
          child: Text(
            'Select a rotation from the above list to get started.',
            textAlign: TextAlign.center,
            style: TextStyle(
              color: textColor,
              fontSize: MediaQuery.of(context).size.width * 0.043
            )
          )
        ),
        const SizedBox(height: 40)
      ]
    );
  }
}