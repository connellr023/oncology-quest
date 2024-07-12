import 'package:flutter/material.dart';
import 'package:oncology_quest_mobile/src/models/entry_levels.dart';
import 'package:oncology_quest_mobile/src/models/session.dart';
import 'package:oncology_quest_mobile/src/state/entries_state.dart';
import 'package:oncology_quest_mobile/src/utilities/colors.dart';
import 'package:oncology_quest_mobile/src/utilities/error_handling.dart';
import 'package:oncology_quest_mobile/src/utilities/regex.dart';
import 'package:oncology_quest_mobile/src/utilities/sizing.dart';
import 'package:oncology_quest_mobile/src/widgets/dashboard/basic_option.dart';
import 'package:oncology_quest_mobile/src/widgets/dashboard/progressable_entry_layer.dart';
import 'package:oncology_quest_mobile/src/widgets/dashboard/input_panel.dart';
import 'package:oncology_quest_mobile/src/widgets/dashboard/subtask_entry.dart';
import 'package:provider/provider.dart';

class Entries extends StatelessWidget {
  final Session session;
  final String jwt;
  final int rotationId;

  const Entries({
    super.key,
    required this.session,
    required this.jwt,
    required this.rotationId
  });

  void _showCreateEntryModal(BuildContext context, String title, void Function(String) onConfirm) {
    showInteractivePanel(context, InputPanel(
      hintText: title,
      errorMessage: '$title title can only contain letters, numbers, and the characters +, -, (, ), and / and be within 1 and 100 characters.',
      regex: entryTitleRegex,
      onConfirm: onConfirm
    ));
  }
  
  @override
  Widget build(BuildContext context) {
    return Consumer<EntriesState>(
      builder: (context, entriesState, child) {
        final entries = entriesState.entries[rotationId];

        return Column(
          children: <Widget>[
            if (entries != null && entries.isNotEmpty) ...entries.asMap().entries.map(
              (entry) => _buildFullEntry(context, entriesState, entry.value, entry.key)
            )
            else Padding(
              padding: const EdgeInsets.only(
                top: 20,
                bottom: 20
              ),
              child: Text(
                'No entries found for this rotation.',
                style: TextStyle(
                  color: textColor.withOpacity(0.6),
                  fontSize: uiFontSize(context)
                )
              )
            ),
            if (session.user.isAdmin) ...<Widget>[
              BasicOption(
                backgroundColor: backgroundColor2,
                title: 'New CBD Phase',
                color: okColor,
                icon: Icons.add,
                padding: const EdgeInsets.all(15),
                onTap: () => _showCreateEntryModal(
                  context,
                  'New CBD Phase',
                  (title) => attemptFallible(context, () => entriesState.createSupertask(jwt, title, rotationId))
                )
              ),
              const SizedBox(height: 20)
            ]
          ]
        );
      }
    );
  }

  Widget _buildFullEntry(BuildContext context, EntriesState entriesState, EntryHierarchy supertaskLevel, int supertaskIndex) {
    return Padding(
      padding: const EdgeInsets.only(bottom: 15),
      child: ClipRRect(
        borderRadius: BorderRadius.circular(15),
        child: ProgressableEntryLayer(
          calculateProgress: (state) => state.calculateSupertaskProgress(rotationId, supertaskLevel),
          session: session,
          backgroundColor: backgroundColor2,
          title: supertaskLevel.hierarchy.entry.title,
          children: <Widget>[
            ...supertaskLevel.hierarchy.children.asMap().entries.map((taskEntry) => ProgressableEntryLayer(
              calculateProgress: (state) => state.calculateTaskProgress(rotationId, supertaskLevel.hierarchy.entry.id, taskEntry.value.entry.id, taskEntry.value.children),
              session: session,
              backgroundColor: backgroundColor3,
              title: taskEntry.value.entry.title,
              children: <Widget>[
                const SizedBox(height: 15),
                ...taskEntry.value.children.map((subtask) => SubtaskEntry(
                  session: session,
                  jwt: jwt,
                  subtask: subtask,
                  supertaskId: supertaskLevel.hierarchy.entry.id,
                  taskId: taskEntry.value.entry.id
                )),
                if (session.user.isAdmin) _buildNewEntryButton(
                  context,
                  'New Clinical Experience',
                  (subtaskTitle) => attemptFallible(context, () => entriesState.createSubtask(jwt, subtaskTitle, rotationId, taskEntry.value.entry.id, supertaskIndex, taskEntry.key))
                )
              ]
            )),
            if (session.user.isAdmin) _buildNewEntryButton(
              context,
              'New EPA',
              (taskTitle) => attemptFallible(context, () => entriesState.createTask(jwt, taskTitle, rotationId, supertaskLevel.hierarchy.entry.id, supertaskIndex))
            )
          ]
        )
      )
    );
  }

  Widget _buildNewEntryButton(BuildContext context, String title, void Function(String) onConfirm) {
    return BasicOption(
      title: title,
      color: okColor,
      icon: Icons.add,
      padding: const EdgeInsets.all(15),
      borderRadius: 0,
      onTap: () => _showCreateEntryModal(
        context,
        title,
        onConfirm
      )
    );
  }
}