import 'package:flutter/material.dart';
import 'package:oncology_quest_mobile/src/models/entry_levels.dart';
import 'package:oncology_quest_mobile/src/utilities/colors.dart';

class SubtaskEntry extends StatefulWidget {
  final Subtask subtask;

  const SubtaskEntry({
    super.key,
    required this.subtask
  });

  @override
  State<SubtaskEntry> createState() => _SubtaskEntryState();
}

class _SubtaskEntryState extends State<SubtaskEntry> {
  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.only(
        left: 27,
        right: 15,
        bottom: 20,
      ),
      child: Material(
        color: Colors.transparent,
        child: Row(
          children: <Widget>[
            Icon(
              Icons.circle,
              color: themeColor,
              size: MediaQuery.of(context).size.width * 0.045
            ),
            const SizedBox(width: 12),
            Text(
              widget.subtask.title,
              style: TextStyle(
                color: textColor,
                fontSize: MediaQuery.of(context).size.width * 0.044
              )
            ),
            // const Spacer(),
            // Checkbox(
            //   value: true,
            //   onChanged: (bool? value) => {}
            // )
          ]
        )
      ),
    );
  }
}