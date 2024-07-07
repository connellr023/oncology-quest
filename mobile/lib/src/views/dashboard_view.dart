import 'package:flutter/material.dart';
import 'package:oncology_quest_mobile/src/models/session.dart';
import 'package:oncology_quest_mobile/src/state/entries_state.dart';
import 'package:oncology_quest_mobile/src/state/session_state.dart';
import 'package:oncology_quest_mobile/src/utilities/colors.dart';
import 'package:oncology_quest_mobile/src/widgets/dashboard/bottom_panel.dart';
import 'package:oncology_quest_mobile/src/widgets/dashboard/dashboard_app_bar.dart';
import 'package:oncology_quest_mobile/src/widgets/dashboard/full_entry.dart';
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
  int? _selectedRotationId;

  void _updateSelectedRotationId(int? id) {
    setState(() {
      _selectedRotationId = id;
    });
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
              RotationSelect(
                session: session,
                onRotationSelect: _updateSelectedRotationId,
              ),
              if (_selectedRotationId != null) ...<Widget>[
                const SizedBox(height: 35),
                SectionHeading(context: context, title: session.user.isAdmin ? 'Task Entries' : 'My Progress'),
                _buildEntries(session)
              ]
              else ...<Widget>[
                const SizedBox(height: 60),
                _buildNoRotationSelected(context)
              ]
            ]
          )
        )
      )
    );
  }

  Consumer<EntriesState> _buildEntries(Session session) {
    return Consumer<EntriesState>(
      builder: (context, entriesState, child) {
        final entries = entriesState.entries[_selectedRotationId];

        if (entries == null || entries.isEmpty) {
          return Column(
            children: <Widget>[
              const SizedBox(height: 15),
              Text(
                'No entries found for this rotation.',
                style: TextStyle(
                  color: textColor.withOpacity(0.6),
                  fontSize: MediaQuery.of(context).size.width * 0.045
                )
              )
            ]
          );
        }

        return Column(
          children: entries.map((entry) {
            return FullEntry(
              level: entry,
              session: session
            );
          }).toList()
        );
      }
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