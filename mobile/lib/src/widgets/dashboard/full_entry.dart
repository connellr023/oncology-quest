import 'package:flutter/material.dart';
import 'package:oncology_quest_mobile/src/models/entry_levels.dart';
import 'package:oncology_quest_mobile/src/utilities/colors.dart';
import 'package:oncology_quest_mobile/src/widgets/dashboard/expandable_section.dart';
import 'package:oncology_quest_mobile/src/widgets/dashboard/subtask_entry.dart';

class FullEntry extends StatelessWidget {
  // bool _supertaskExpanded = false;
  // final Set<int> _tasksExpanded = {};
  // final Set<int> _subtasksExpanded = {};

  final EntryHierarchy level;

  const FullEntry({
    super.key,
    required this.level
  });

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.only(bottom: 15),
      child: ClipRRect(
        borderRadius: BorderRadius.circular(15),
        child: ExpandableSection(
          backgroundColor: backgroundColor2,
          title: level.hierarchy.entry.title,
          children: level.hierarchy.children.map((taskLevel) => ExpandableSection(
            backgroundColor: backgroundColor3,
            title: taskLevel.entry.title,
            children: <Widget>[
              const SizedBox(height: 15),
              ...taskLevel.children.map((subtask) => SubtaskEntry(subtask: subtask))
            ]
          )).toList()
        ),
      )
    );
  }
}