import 'package:flutter/material.dart';
import 'package:oncology_quest_mobile/src/models/rotation.dart';
import 'package:oncology_quest_mobile/src/state/session_state.dart';
import 'package:oncology_quest_mobile/src/utilities/colors.dart';
import 'package:oncology_quest_mobile/src/utilities/regex.dart';
import 'package:oncology_quest_mobile/src/widgets/dashboard/bottom_panel.dart';
import 'package:oncology_quest_mobile/src/widgets/dashboard/basic_option.dart';
import 'package:oncology_quest_mobile/src/widgets/dashboard/dashboard_app_bar.dart';
import 'package:oncology_quest_mobile/src/widgets/dashboard/edit_option.dart';
import 'package:oncology_quest_mobile/src/widgets/dashboard/graphic.dart';
import 'package:oncology_quest_mobile/src/widgets/dashboard/input_panel.dart';
import 'package:provider/provider.dart';

class DashboardView extends StatefulWidget {
  const DashboardView({super.key});

  @override
  State<DashboardView> createState() => _DashboardViewState();
}

class _DashboardViewState extends State<DashboardView> {
  bool _isEditingRotations = false;

  int? _selectedRotationId;

  void _selectRotation(int rotationId) {
    setState(() {
      _selectedRotationId = _selectedRotationId == rotationId ? null : rotationId;
      _isEditingRotations = false;
    });
  }

  void _toggleEditRotations() {
    setState(() {
      _isEditingRotations = !_isEditingRotations;
      _selectedRotationId = null;
    });
  }

  void _showBottomPanel(BuildContext context) {
    showModalBottomSheet(
      context: context,
      backgroundColor: backgroundColor2,
      builder: (BuildContext context) {
        return const BottomPanel();
      }
    );
  }

  void _showInputModal(BuildContext context) {
    setState(() {
      _isEditingRotations = false;
    });

    showModalBottomSheet(
      context: context,
      backgroundColor: backgroundColor2,
      isScrollControlled: true, // Make the modal resizable
      builder: (BuildContext context) {
        return SingleChildScrollView( // Make the content scrollable
          padding: EdgeInsets.only(
            bottom: MediaQuery.of(context).viewInsets.bottom, // Adjust padding based on the keyboard
          ),
          child: InputPanel(
            hintText: 'Enter rotation name',
            errorMessage: 'Rotation name must contain only letters and spaces and be within 1 and 35 characters long',
            regex: nameRegex,
          ),
        );
      },
    );
  }

  @override
  Widget build(BuildContext context) {
    final session = Provider.of<SessionState>(context, listen: false).session;

    if (session == null) {
      return const Scaffold(
        body: Center(
          child: CircularProgressIndicator(),
        )
      );
    }

    return Scaffold(
      appBar: DashboardAppBar(
        session: session,
        onProfileTap: () => _showBottomPanel(context)
      ),
      body: Padding(
        padding: const EdgeInsets.all(15),
        child: SingleChildScrollView(
          child: Column(
            mainAxisAlignment: MainAxisAlignment.start,
            children: <Widget>[
              Row(
                children: <Widget>[
                  _buildHeading(context, 'Rotations'),
                  const Expanded(child: SizedBox()),
                  if (session.user.isAdmin) Container(
                    padding: const EdgeInsets.only(bottom: 10),
                    child: Row(
                      children: <Widget>[
                        BasicOption(
                          context: context,
                          title: 'New',
                          color: okColor,
                          icon: Icons.add_box,
                          onTap: () => _showInputModal(context)
                        ),
                        const SizedBox(width: 5),
                        EditOption(
                          context: context,
                          isEditing: _isEditingRotations,
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
              ),
              if (_selectedRotationId != null) ...<Widget>[
                const SizedBox(height: 35),
                _buildHeading(context, 'My Progress')
              ]
              else ...<Widget>[
                const SizedBox(height: 60),
                _buildNoRotationSelected(context)
              ]
            ]
          ),
        )
      )
    );
  }

  Widget _buildNoRotationSelected(BuildContext context) {
    return Column(
      children: <Widget>[
        Graphic(imageWidth: MediaQuery.of(context).size.width * 0.65),
        Padding(
          padding: const EdgeInsets.all(20),
          child: Text(
            'Select a rotation from the above list to get started.',
            textAlign: TextAlign.center,
            style: TextStyle(
              color: textColor,
              fontSize: MediaQuery.of(context).size.width * 0.043
            )
          )
        ),
        const SizedBox(height: 40)
      ]
    );
  }

  Widget _buildHeading(BuildContext context, String title) {
    return Column(
      children: <Widget>[
        Row(
          children: [
            Text(
              title,
              textAlign: TextAlign.left,
              style: TextStyle(
                color: textColor,
                fontSize: MediaQuery.of(context).size.width * 0.068
              )
            )
          ]
        ),
        const SizedBox(height: 13)
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
        splashColor: _isEditingRotations ? errorColor : okColor,
        borderRadius: BorderRadius.circular(borderRadius),
        onTap: () => _selectRotation(rotation.id),
        child: Container(
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