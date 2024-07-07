import 'package:flutter/material.dart';
import 'package:oncology_quest_mobile/src/models/rotation.dart';
import 'package:oncology_quest_mobile/src/models/session.dart';
import 'package:oncology_quest_mobile/src/state/entries_state.dart';
import 'package:oncology_quest_mobile/src/state/session_state.dart';
import 'package:oncology_quest_mobile/src/utilities/colors.dart';
import 'package:oncology_quest_mobile/src/utilities/display_error.dart';
import 'package:oncology_quest_mobile/src/utilities/regex.dart';
import 'package:oncology_quest_mobile/src/widgets/dashboard/basic_option.dart';
import 'package:oncology_quest_mobile/src/widgets/dashboard/two_variant_option.dart';
import 'package:oncology_quest_mobile/src/widgets/dashboard/input_panel.dart';
import 'package:oncology_quest_mobile/src/widgets/dashboard/section_heading.dart';
import 'package:provider/provider.dart';

class RotationSelect extends StatefulWidget {
  final Session session;
  final Function(int?) onRotationSelect;

  const RotationSelect({
    super.key,
    required this.session,
    required this.onRotationSelect
  });

  @override
  State<RotationSelect> createState() => _RotationSelectState();
}

class _RotationSelectState extends State<RotationSelect> {
  bool _isEditingRotations = false;
  int? _selectedRotationId;

  Future<void> _attemptFetchEntries(int rotationId) async {
    final entriesState = Provider.of<EntriesState>(context, listen: false);
    final sessionState = Provider.of<SessionState>(context, listen: false);
    final String? errorMessage = await entriesState.fetchEntries(sessionState.jwt, rotationId);

    if (errorMessage != null && mounted) {
      displayError(context, errorMessage);
    }
  }

  void _selectRotation(int rotationId) {
    setState(() {
      _selectedRotationId = _selectedRotationId == rotationId ? null : rotationId;
      _isEditingRotations = false;

      widget.onRotationSelect(_selectedRotationId);
    });

    _attemptFetchEntries(rotationId);
  }

  void _toggleEditRotations() {
    setState(() {
      _isEditingRotations = !_isEditingRotations;
      _selectedRotationId = null;

      widget.onRotationSelect(_selectedRotationId);
    });
  }

  Future<void> _attemptCreateRotation(String name) async {
    final sessionState = Provider.of<SessionState>(context, listen: false);
    final String? errorMessage = await sessionState.createRotation(name);

    if (errorMessage != null && mounted) {
      displayError(context, errorMessage);
    }
  }

  Future<void> _attemptDeleteRotation(int rotationId) async {
    final sessionState = Provider.of<SessionState>(context, listen: false);
    final String? errorMessage = await sessionState.deleteRotation(rotationId);

    if (errorMessage != null && mounted) {
      displayError(context, errorMessage);
    }
  }

  void _showInputModal(BuildContext context) {
    setState(() {
      _isEditingRotations = false;
    });

    showModalBottomSheet(
      context: context,
      backgroundColor: backgroundColor2,
      isScrollControlled: true,
      builder: (BuildContext context) {
        return SingleChildScrollView(
          padding: EdgeInsets.only(
            bottom: MediaQuery.of(context).viewInsets.bottom
          ),
          child: InputPanel(
            hintText: 'Enter rotation name',
            errorMessage: 'Rotation name must contain only letters and spaces and be within 1 and 35 characters long',
            regex: nameRegex,
            onConfirm: _attemptCreateRotation
          ),
        );
      },
    );
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
        SizedBox(
          width: double.infinity,
          child: Consumer<SessionState>(
            builder: (context, sessionState, child) => Wrap(
              spacing: 10,
              runSpacing: 10,
              children: [
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
    const double borderRadius = 18;
    final bool isSelected = _selectedRotationId == rotation.id;
    
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
                  size: MediaQuery.of(context).size.width * 0.06
                ),
                const SizedBox(width: 10)
              ]
              else if (_isEditingRotations) ...<Widget>[
                Icon(
                  Icons.delete_forever,
                  color: errorColor,
                  size: MediaQuery.of(context).size.width * 0.06
                ),
                const SizedBox(width: 10)
              ],
              Text(
                rotation.name,
                style: TextStyle(
                  color: isSelected ? okColor : _isEditingRotations ? errorColor : textColor,
                  fontSize: MediaQuery.of(context).size.width * 0.042
                )
              )
            ]
          )
        )
      )
    );
  }
}