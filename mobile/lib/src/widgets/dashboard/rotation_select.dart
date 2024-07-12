import 'package:flutter/material.dart';
import 'package:oncology_quest_mobile/src/models/rotation.dart';
import 'package:oncology_quest_mobile/src/models/session.dart';
import 'package:oncology_quest_mobile/src/state/entries_state.dart';
import 'package:oncology_quest_mobile/src/state/selected_rotation_state.dart';
import 'package:oncology_quest_mobile/src/state/session_state.dart';
import 'package:oncology_quest_mobile/src/state/user_tasks_state.dart';
import 'package:oncology_quest_mobile/src/utilities/colors.dart';
import 'package:oncology_quest_mobile/src/utilities/error_handling.dart';
import 'package:oncology_quest_mobile/src/utilities/regex.dart';
import 'package:oncology_quest_mobile/src/utilities/sizing.dart';
import 'package:oncology_quest_mobile/src/widgets/dashboard/basic_option.dart';
import 'package:oncology_quest_mobile/src/widgets/dashboard/two_variant_option.dart';
import 'package:oncology_quest_mobile/src/widgets/dashboard/input_panel.dart';
import 'package:oncology_quest_mobile/src/widgets/dashboard/section_heading.dart';
import 'package:provider/provider.dart';

class RotationSelect extends StatefulWidget {
  final Session session;

  const RotationSelect({
    super.key,
    required this.session
  });

  @override
  State<RotationSelect> createState() => _RotationSelectState();
}

class _RotationSelectState extends State<RotationSelect> {
  bool _isEditingRotations = false;

  Future<void> _attemptFetchEntries(int rotationId) async {
    final entriesState = Provider.of<EntriesState>(context, listen: false);
    final sessionState = Provider.of<SessionState>(context, listen: false);
    final userTasksState = Provider.of<UserTasksState>(context, listen: false);
    
    attemptFallible(context, () async {
      if (!sessionState.session!.user.isAdmin) {
        // User tasks must be loaded first to ensure that the user's progress is displayed correctly.
        final String? userTasksErrorMessage = await userTasksState.fetchOwnUserTasks(sessionState.jwt!, rotationId);

        if (userTasksErrorMessage != null) {
          return userTasksErrorMessage;
        }
      }

      final String? entriesErrorMessage = await entriesState.fetchEntries(sessionState.jwt!, rotationId);

      if (entriesErrorMessage != null) {
        return entriesErrorMessage;
      } 

      return null;
    });
  }

  void _selectRotation(int rotationId) {
    final selectedRotationState = Provider.of<SelectedRotationState>(context, listen: false);

    setState(() {
      _isEditingRotations = false;
      selectedRotationState.selectRotation(selectedRotationState.selectedRotationId == rotationId ? null : rotationId);
    });

    _attemptFetchEntries(rotationId);
  }

  void _toggleEditRotations() {
    final selectedRotationState = Provider.of<SelectedRotationState>(context, listen: false);

    setState(() {
      _isEditingRotations = !_isEditingRotations;
      selectedRotationState.selectRotation(null);
    });
  }

  void _attemptCreateRotation(String name) async {
    final sessionState = Provider.of<SessionState>(context, listen: false);
    attemptFallible(context, () => sessionState.createRotation(name));
  }

  void _attemptDeleteRotation(int rotationId) {
    final sessionState = Provider.of<SessionState>(context, listen: false);
    attemptFallible(context, () => sessionState.deleteRotation(rotationId));
  }

  void _showInputModal(BuildContext context) {
    setState(() {
      _isEditingRotations = false;
    });

    showInteractivePanel(context, InputPanel(
      hintText: 'Enter rotation name',
      errorMessage: 'Rotation name must contain only letters and spaces and be within 1 and 35 characters long',
      regex: nameRegex,
      onConfirm: _attemptCreateRotation
    ));
  }

  @override
  Widget build(BuildContext context) {
    return Column(
      children: <Widget>[
        Row(
          children: <Widget>[
            SectionHeading(context: context, title: 'Rotations'),
            const Expanded(child: SizedBox()),
            if (widget.session.user.isAdmin) Container(
              padding: const EdgeInsets.only(bottom: 10),
              child: Row(
                children: <Widget>[
                  BasicOption(
                    context: context,
                    title: 'New',
                    color: okColor,
                    icon: Icons.add,
                    onTap: () => _showInputModal(context)
                  ),
                  const SizedBox(width: 5),
                  TwoVariantOption(
                    firstColor: errorColor,
                    secondColor: okColor,
                    firstIcon: Icons.delete_forever,
                    secondIcon: Icons.done,
                    firstText: 'Delete',
                    secondText: 'Done',
                    context: context,
                    inFirstVariant: !_isEditingRotations,
                    onTap: () => _toggleEditRotations()
                  )
                ]
              )
            )
          ]
        ),
        Center(
          child: Consumer<SessionState>(
            builder: (context, sessionState, child) => Wrap(
              spacing: 10,
              runSpacing: 10,
              children: <Widget>[
                if (sessionState.session != null)
                  for (final rotationEntry in sessionState.session!.rotations.entries)
                    _buildRotationOption(context, rotationEntry.value)
              ]
            )
          )
        )
      ]
    );
  }

  Widget _buildRotationOption(BuildContext context, Rotation rotation) {
    final selectedRotationId = Provider.of<SelectedRotationState>(context).selectedRotationId;

    const double borderRadius = 18;
    final bool isSelected = selectedRotationId == rotation.id;
    
    double size = standardFontSize(context);

    return Material(
      color: backgroundColor2,
      borderRadius: BorderRadius.circular(borderRadius),
      child: InkWell(
        splashColor: _isEditingRotations ? errorColor : isSelected ? textColor : okColor,
        borderRadius: BorderRadius.circular(borderRadius),
        onTap: () => _isEditingRotations ? _attemptDeleteRotation(rotation.id) : _selectRotation(rotation.id),
        child: Padding(
          padding: const EdgeInsets.all(17),
          child: Row(
            mainAxisSize: MainAxisSize.min,
            children: <Widget>[
              if (isSelected) ...<Widget>[
                Icon(
                  Icons.check,
                  color: okColor,
                  size: size
                ),
                const SizedBox(width: 10)
              ]
              else if (_isEditingRotations) ...<Widget>[
                Icon(
                  Icons.delete_forever,
                  color: errorColor,
                  size: size
                ),
                const SizedBox(width: 10)
              ],
              Text(
                rotation.name,
                style: TextStyle(
                  color: isSelected ? okColor : _isEditingRotations ? errorColor : textColor,
                  fontSize: size
                )
              )
            ]
          )
        )
      )
    );
  }
}