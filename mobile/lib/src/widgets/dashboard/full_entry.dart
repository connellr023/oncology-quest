import 'package:flutter/material.dart';
import 'package:oncology_quest_mobile/src/models/entry_levels.dart';
import 'package:oncology_quest_mobile/src/utilities/colors.dart';

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
      child: ClipRRect(
        borderRadius: BorderRadius.circular(15),
        child: Theme(
          data: Theme.of(context).copyWith(
            splashColor: Colors.red
          ),
          child: Material(
            color: Colors.transparent,
            child: ExpansionTile(
              backgroundColor: backgroundColor2,
              collapsedBackgroundColor: backgroundColor2,
              iconColor: themeColor,
              collapsedIconColor: textColor,
              controlAffinity: ListTileControlAffinity.leading,
              title: Text(
                entry.hierarchy.entry.title,
                style: TextStyle(
                  fontSize: MediaQuery.of(context).size.width * 0.055,
                  color: textColor
                ),
              ),
              children: <Widget>[]
            ),
          ),
        ),
      ),
    );
  }
}