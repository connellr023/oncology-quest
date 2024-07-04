import 'package:flutter/material.dart';
import 'package:oncology_quest_mobile/src/models/rotation.dart';
import 'package:oncology_quest_mobile/src/state/session_state.dart';
import 'package:oncology_quest_mobile/src/utilities/colors.dart';
import 'package:oncology_quest_mobile/src/widgets/bottom_panel.dart';
import 'package:oncology_quest_mobile/src/widgets/dashboard_app_bar.dart';
import 'package:oncology_quest_mobile/src/widgets/graphic.dart';
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
      body: Center(
        child: Padding(
          padding: const EdgeInsets.all(15),
          child: Column(
            mainAxisAlignment: MainAxisAlignment.start,
            children: <Widget>[
              Row(
                crossAxisAlignment: CrossAxisAlignment.center,
                children: <Widget>[
                  Expanded(
                    child: _buildHeading(context, 'Rotations'),
                  ),
                  if (session.user.isAdmin) ...<Widget>[
                    _buildBasicOption(context,
                      'New',
                      okColor,
                      Icons.add_box,
                      () => {}
                    ),
                    const SizedBox(width: 5),
                    _buildEditOption(
                      context,
                      _isEditingRotations,
                      () => _toggleEditRotations()
                    )
                  ]
                ]
              ),
              SizedBox(
                width: double.infinity,
                child: Wrap(
                  spacing: 10,
                  runSpacing: 10,
                  children: session.rotations.entries.map<Widget>((entry) {
                    return _buildRotationOption(context, entry.value);
                  }).toList()
                ),
              ),
              if (_selectedRotationId != null) ...<Widget>[
                const SizedBox(height: 35),
                _buildHeading(context, 'My Progress')
              ]
              else ...<Widget>[
                const Expanded(child: SizedBox()),
                _buildNoRotationSelected(context)
              ]
            ]
          )
        )
      )
    );
  }

  Widget _buildBasicOption(BuildContext context, String title, Color color, IconData icon, void Function() onTap) {
    return InkWell(
      borderRadius: BorderRadius.circular(15),
      onTap: onTap,
      splashColor: color,
      child: Padding(
        padding: const EdgeInsets.all(7),
        child: Row(
          children: <Widget>[
            Icon(
              icon,
              color: color,
              size: MediaQuery.of(context).size.width * 0.06
            ),
            const SizedBox(width: 5),
            Text(
              title,
              style: TextStyle(
                color: color,
                fontSize: MediaQuery.of(context).size.width * 0.042
              )
            )
          ]
        ),
      )
    );
  }

  Widget _buildEditOption(BuildContext context, bool isEditing, void Function() onTap) {
    return InkWell(
      borderRadius: BorderRadius.circular(15),
      onTap: onTap,
      splashColor: isEditing ? textColor : okColor,
      child: Padding(
        padding: const EdgeInsets.all(7),
        child: Row(
          children: <Widget>[
            Icon(
              isEditing ? Icons.done : Icons.edit,
              color: isEditing ? okColor : textColor,
              size: MediaQuery.of(context).size.width * 0.06
            ),
            const SizedBox(width: 5),
            Text(
              isEditing ? 'Done' : 'Edit',
              style: TextStyle(
                color: isEditing ? okColor : textColor,
                fontSize: MediaQuery.of(context).size.width * 0.042
              )
            )
          ]
        ),
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
      children: [
        SizedBox(
          width: double.infinity,
          child: Text(
            title,
            textAlign: TextAlign.left,
            style: TextStyle(
              color: textColor,
              fontSize: MediaQuery.of(context).size.width * 0.068
            )
          )
        ),
        const SizedBox(height: 13)
      ],
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

  void _showBottomPanel(BuildContext context) {
    showModalBottomSheet(
      context: context,
      backgroundColor: backgroundColor2,
      builder: (BuildContext context) {
        return const BottomPanel();
      }
    );
  }
}