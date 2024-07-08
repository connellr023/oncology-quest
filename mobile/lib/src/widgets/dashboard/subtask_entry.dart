import 'package:flutter/material.dart';
import 'package:oncology_quest_mobile/src/models/entry_levels.dart';
import 'package:oncology_quest_mobile/src/models/session.dart';
import 'package:oncology_quest_mobile/src/models/user_task.dart';
import 'package:oncology_quest_mobile/src/state/user_tasks_state.dart';
import 'package:oncology_quest_mobile/src/utilities/colors.dart';
import 'package:oncology_quest_mobile/src/utilities/error_handling.dart';
import 'package:oncology_quest_mobile/src/widgets/dashboard/two_variant_option.dart';
import 'package:provider/provider.dart';

class SubtaskEntry extends StatefulWidget {
  final Session session;
  final String jwt;
  final Subtask subtask;

  const SubtaskEntry({
    super.key,
    required this.session,
    required this.jwt,
    required this.subtask
  });

  @override
  State<SubtaskEntry> createState() => _SubtaskEntryState();
}

class _SubtaskEntryState extends State<SubtaskEntry> {
  late bool _isCompleted;
  late String _comment;

  UserTasksState get _userTasksState => Provider.of<UserTasksState>(context, listen: false);
  UserTask? get _userTask => _userTasksState.userTasks[widget.subtask.rotationId]?.structure[widget.subtask.id];

  Future<void> _optimisticUpdateUserTask(bool isCompleted, String comment) async {
    setState(() {
      _isCompleted = isCompleted;
      _comment = comment;
    });

    final success = await attemptFallible(context, () => _userTasksState.updateUserTask(
      widget.jwt,
      widget.subtask.rotationId,
      widget.subtask.id,
      widget.session.user.id,
      _isCompleted,
      _comment
    ));

    if (!success) {
      setState(() {
        _isCompleted = _userTask?.isCompleted ?? false;
        _comment = _userTask?.comment ?? '';
      });
    } 
  }

  @override
  Widget build(BuildContext context) {
    _isCompleted = _userTask?.isCompleted ?? false;
    _comment = _userTask?.comment ?? '';

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
            Expanded(
              child: Text(
                widget.subtask.title,
                style: TextStyle(
                  color: textColor,
                  fontSize: MediaQuery.of(context).size.width * 0.044
                )
              )
            ),
            const SizedBox(width: 10),
            TwoVariantOption(
              firstColor: errorColor,
              secondColor: okColor,
              firstIcon: Icons.close,
              secondIcon: Icons.done,
              firstText: 'Working',
              secondText: 'Done',
              context: context,
              inFirstVariant: !_isCompleted,
              isDisabled: widget.session.user.isAdmin,
              onTap: () => _optimisticUpdateUserTask(!_isCompleted, _comment)
            )
          ]
        )
      )
    );
  }
}