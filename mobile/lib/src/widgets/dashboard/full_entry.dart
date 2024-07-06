import 'package:flutter/material.dart';
import 'package:oncology_quest_mobile/src/models/entry_levels.dart';
import 'package:oncology_quest_mobile/src/utilities/colors.dart';
import 'package:oncology_quest_mobile/src/widgets/dashboard/expandable_section.dart';

class FullEntry extends StatelessWidget {
  bool _supertaskExpanded = false;
  final Set<int> _tasksExpanded = {};
  final Set<int> _subtasksExpanded = {};

  final EntryHierarchy entry;

  FullEntry({
    super.key,
    required this.entry
  });

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.only(bottom: 15),
      child: ExpandableSection(
        backgroundColor: backgroundColor2,
        title: entry.hierarchy.entry.title,
        children: <Widget>[]
      )
    );
  }
}