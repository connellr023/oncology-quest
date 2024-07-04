import 'package:flutter/material.dart';
import 'package:oncology_quest_mobile/src/utilities/colors.dart';
import 'package:oncology_quest_mobile/src/widgets/dashboard/panel_input_option.dart';
import 'package:oncology_quest_mobile/src/widgets/dashboard/panel_option.dart';

class InputPanel extends StatefulWidget {
  const InputPanel({super.key});

  @override
  State<InputPanel> createState() => _InputPanelState();
}

class _InputPanelState extends State<InputPanel> {
  @override
  Widget build(BuildContext context) {
    return SizedBox(
      height: 220,
      child: ListView(
        padding: const EdgeInsets.all(10),
        children: <Widget>[
          PanelInputOption(
            hintText: 'Enter text',
            splashColor: okColor,
            onChanged: (input) => {},
          ),
          const SizedBox(height: 20),
          PanelOption(
            text: 'Confirm',
            icon: Icons.done,
            onTap: () => Navigator.pop(context),
            color: okColor
          ),
          PanelOption(
            text: 'Cancel',
            icon: Icons.close,
            onTap: () => Navigator.pop(context),
            color: errorColor
          )
        ],
      ),
    );
  }
}