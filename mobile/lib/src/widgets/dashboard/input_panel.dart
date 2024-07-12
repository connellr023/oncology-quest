import 'package:flutter/material.dart';
import 'package:oncology_quest_mobile/src/utilities/colors.dart';
import 'package:oncology_quest_mobile/src/utilities/sizing.dart';
import 'package:oncology_quest_mobile/src/widgets/dashboard/panel_input_option.dart';
import 'package:oncology_quest_mobile/src/widgets/dashboard/panel_option.dart';

class InputPanel extends StatefulWidget {
  final String hintText;
  final String errorMessage;
  final RegExp regex;
  final void Function(String) onConfirm;

  const InputPanel({
    super.key,
    required this.hintText,
    required this.errorMessage,
    required this.regex,
    required this.onConfirm
  });

  @override
  State<InputPanel> createState() => _InputPanelState();
}

class _InputPanelState extends State<InputPanel> {
  bool _isError = true;

  String _input = '';

  void _validateInput(String input) {
    setState(() {
      _isError = !widget.regex.hasMatch(input);
      _input = input;
    });
  }

  @override
  Widget build(BuildContext context) {
    return SingleChildScrollView(
      padding: EdgeInsets.only(
        bottom: MediaQuery.of(context).viewInsets.bottom
      ),
      child: Padding(
        padding: const EdgeInsets.all(10),
        child: Column(
          mainAxisSize: MainAxisSize.min,
          children: <Widget>[
            PanelInputOption(
              hintText: widget.hintText,
              isError: _isError,
              onChanged: _validateInput
            ),
            if (_isError) ...<Widget>[
              const SizedBox(height: 10),
              Text(
                widget.errorMessage,
                style: TextStyle(
                  color: errorColor,
                  fontSize: modalFontSize(context)
                ),
                textAlign: TextAlign.center
              )
            ],
            const SizedBox(height: 20),
            PanelOption(
              text: 'Confirm',
              icon: Icons.done,
              onTap: () {
                widget.onConfirm(_input);
                Navigator.pop(context);
              },
              splashColor: okColor,
              isDisabled: _isError,
            ),
            PanelOption(
              text: 'Cancel',
              icon: Icons.close,
              onTap: () => Navigator.pop(context),
              splashColor: errorColor
            )
          ]
        )
      ),
    );
  }
}