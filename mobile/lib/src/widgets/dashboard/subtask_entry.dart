import 'package:flutter/material.dart';
import 'package:oncology_quest_mobile/src/models/entry_levels.dart';
import 'package:oncology_quest_mobile/src/models/session.dart';
import 'package:oncology_quest_mobile/src/models/user_task.dart';
import 'package:oncology_quest_mobile/src/state/user_tasks_state.dart';
import 'package:oncology_quest_mobile/src/utilities.dart';
import 'package:oncology_quest_mobile/src/widgets/dashboard/panel_input_option.dart';
import 'package:oncology_quest_mobile/src/widgets/dashboard/two_variant_option.dart';
import 'package:provider/provider.dart';

class SubtaskEntry extends StatefulWidget {
  final Session session;
  final String jwt;
  final Subtask subtask;
  final int supertaskId;
  final int taskId;

  const SubtaskEntry({
    super.key,
    required this.session,
    required this.jwt,
    required this.subtask,
    required this.supertaskId,
    required this.taskId
  });

  @override
  State<SubtaskEntry> createState() => _SubtaskEntryState();
}


class _SubtaskEntryState extends State<SubtaskEntry> {
  late UserTasksState _userTasksStateNotifier;
  UserTask? get _userTask => _userTasksStateNotifier.userTasks[widget.subtask.rotationId]?.structure[widget.subtask.id];

  late bool _isCommentError;
  late bool _isCommentSaved;
  late bool _isCompleted;
  late String _comment;

  @override
  void initState() {
    super.initState();

    _isCommentError = false;
    _isCommentSaved = true;

    _userTasksStateNotifier = Provider.of<UserTasksState>(context, listen: false);
    _userTasksStateNotifier.addListener(_onUserTasksUpdated);

    _isCompleted = _userTask?.isCompleted ?? false;
    _comment = _userTask?.comment ?? '';
  }

  @override
  void dispose() {
    _userTasksStateNotifier.removeListener(_onUserTasksUpdated);
    super.dispose();
  }

  void _onUserTasksUpdated() {
    if (_userTask != null) {
      if (_userTask!.isCompleted == _isCompleted && _userTask!.comment == _comment) {
        return;
      }

      setState(() {
        _isCompleted = _userTask!.isCompleted;
        _comment = _userTask!.comment;
      });
    }
  }

  Future<void> _optimisticUpdateUserTask(bool isCompleted, String comment) async {
    final userTasksState = Provider.of<UserTasksState>(context, listen: false);
    final wasCommentSaved = _isCommentSaved;

    setState(() {
      _isCompleted = isCompleted;
      _comment = comment;
      _isCommentSaved = true;
    });

    final success = await attemptFallible(context, () => userTasksState.updateUserTask(
      widget.jwt,
      widget.subtask.rotationId,
      widget.supertaskId,
      widget.taskId,
      widget.subtask.id,
      widget.session.user.id,
      _isCompleted,
      _comment
    ));

    if (!success) {
      setState(() {
        _isCompleted = _userTask?.isCompleted ?? false;
        _comment = _userTask?.comment ?? '';
        _isCommentSaved = wasCommentSaved;
      });
    }
  }

  void _onCommentChanged(String comment) {
    _comment = comment;

    setState(() {
      _isCommentSaved = false;
      _isCommentError = !commentRegex.hasMatch(comment);
    });
  }

  @override
  Widget build(BuildContext context) {
    final size = standardFontSize(context);

    return Padding(
      padding: const EdgeInsets.only(
        left: 31,
        right: 15,
        bottom: 20,
      ),
      child: Material(
        color: Colors.transparent,
        child: Column(
          children: <Widget>[
            Row(
              children: <Widget>[
                Icon(
                  Icons.circle,
                  color: themeColor,
                  size: size
                ),
                const SizedBox(width: 12),
                Expanded(
                  child: Text(
                    widget.subtask.title,
                    style: TextStyle(
                      color: textColor,
                      fontSize: size
                    )
                  )
                ),
                const SizedBox(width: 10),
                if (!widget.session.user.isAdmin) ...<Widget>[
                  TwoVariantOption(
                    firstColor: okColor,
                    secondColor: textColor,
                    firstIcon: Icons.done,
                    secondIcon: Icons.save,
                    firstText: 'Saved',
                    secondText: 'Not Saved',
                    context: context,
                    inFirstVariant: _isCommentSaved,
                    isDisabled: _isCommentSaved || _isCommentError,
                    onTap: () => _optimisticUpdateUserTask(_isCompleted, _comment)
                  ),
                  const SizedBox(width: 5)
                ],
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
            ),
            const SizedBox(height: 5),
            _buildCommentField(context, widget.session.user.isAdmin)
          ]
        )
      )
    );
  }

  Widget _buildCommentField(BuildContext context, bool isDisabled) {
    return Row(
      children: <Widget>[
        if (_comment.isNotEmpty) Expanded(
          child: PanelInputOption(
            isError: _isCommentError,
            defaultValue: _comment,
            backgroundColor: backgroundColor2,
            isDisabled: isDisabled,
            hintText: isDisabled ? '' : 'Enter a comment',
            onChanged: _onCommentChanged
          )
        )
      ]
    );
  }
}