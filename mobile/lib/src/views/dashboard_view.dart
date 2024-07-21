import 'package:flutter/material.dart';
import 'package:oncology_quest_mobile/src/state/selected_rotation_state.dart';
import 'package:oncology_quest_mobile/src/state/selected_user_state.dart';
import 'package:oncology_quest_mobile/src/state/session_state.dart';
import 'package:oncology_quest_mobile/src/utilities.dart';
import 'package:oncology_quest_mobile/src/widgets/dashboard/bottom_navigation_area.dart';
import 'package:oncology_quest_mobile/src/widgets/dashboard/bottom_panel.dart';
import 'package:oncology_quest_mobile/src/widgets/dashboard/search_users_drawer.dart';
import 'package:oncology_quest_mobile/src/widgets/dashboard/top_app_bar.dart';
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
  @override
  Widget build(BuildContext context) {
    final sessionState = Provider.of<SessionState>(context, listen: false);

    if (sessionState.session == null) {
      return const Scaffold(
        body: Center(
          child: CircularProgressIndicator(),
        )
      );
    }

    return Scaffold(
      appBar: TopAppBar(
        session: sessionState.session!,
        onProfileTap: () => showInteractivePanel(context, const BottomPanel())
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
              RotationSelect(session: sessionState.session!),
              Consumer<SelectedRotationState>(
                builder: (context, selectedRotationState, child) => Column(
                  children: <Widget>[
                    if (selectedRotationState.selectedRotationId != null) Consumer<SelectedUserState>(
                      builder: (context, selectedUserState, child) => Column(
                        children: <Widget>[
                          const SizedBox(height: 35),
                          SectionHeading(
                            title: !sessionState.session!.user.isAdmin
                              ? 'My Progress'
                              : selectedUserState.selectedUser == null
                                ? 'Task Entries'
                                : '${selectedUserState.selectedUser!.name}\'s Progress'
                          ),
                          Entries(
                            rotationId: selectedRotationState.selectedRotationId!,
                            session: sessionState.session!,
                            jwt: sessionState.jwt!
                          )
                        ]
                      )
                    )
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
      ),
      drawer: SearchUsersDrawer(jwt: sessionState.jwt!),
      bottomNavigationBar: sessionState.session!.user.isAdmin ? Builder(
        builder: (context) => BottomNavigationArea(
          onSearchTap: () => Scaffold.of(context).openDrawer()
        )
      ) : null
    );
  }

  Widget _buildNoRotationSelected(BuildContext context) {
    return Column(
      children: <Widget>[
        Graphic(imageWidth: graphicWidth(context)),
        Padding(
          padding: const EdgeInsets.all(20),
          child: Text(
            'Select a rotation from the above list to get started.',
            textAlign: TextAlign.center,
            style: TextStyle(
              color: textColor,
              fontSize: uiFontSize(context)
            )
          )
        ),
        const SizedBox(height: 40)
      ]
    );
  }
}