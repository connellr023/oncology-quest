import 'package:flutter/material.dart';
import 'package:oncology_quest_mobile/src/models/entry_levels.dart';
import 'package:oncology_quest_mobile/src/models/session.dart';
import 'package:oncology_quest_mobile/src/utilities/colors.dart';
import 'package:oncology_quest_mobile/src/widgets/dashboard/two_variant_option.dart';

class SubtaskEntry extends StatefulWidget {
  final Session session;
  final Subtask subtask;

  const SubtaskEntry({
    super.key,
    required this.session,
    required this.subtask
  });

  @override
  State<SubtaskEntry> createState() => _SubtaskEntryState();
}

class _SubtaskEntryState extends State<SubtaskEntry> {
  bool _isComplete = false;

  void _toggleComplete() {
    setState(() {
      _isComplete = !_isComplete;
    });
  }

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
            const Spacer(),
            TwoVariantOption(
              firstColor: errorColor,
              secondColor: okColor,
              firstIcon: Icons.close,
              secondIcon: Icons.done,
              firstText: 'Working',
              secondText: 'Done',
              context: context,
              inFirstVariant: !_isComplete,
              isDisabled: widget.session.user.isAdmin,
              onTap: () => _toggleComplete()
            )
          ]
        )
      ),
    );
  }
}