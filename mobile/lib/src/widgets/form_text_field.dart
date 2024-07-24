import 'package:flutter/material.dart';
import 'package:oncology_quest_mobile/src/utilities.dart';

class FormTextField extends StatefulWidget {
  final String labelText;
  final RegExp validationRegex;
  final String errorMessage;
  final bool obscureText;
  final void Function(bool isError) onErrorChanged;
  final void Function(String input) onChanged;

  const FormTextField({
    super.key,
    required this.labelText,
    required this.validationRegex,
    required this.errorMessage,
    required this.obscureText,
    required this.onErrorChanged,
    required this.onChanged,
  });

  @override
  State<FormTextField> createState() => _FormTextFieldState();
}

class _FormTextFieldState extends State<FormTextField> {
  final int unselectedAlpha = 150;

  String? errorMessage;

  void _validateInput(String input) {
    if (!widget.validationRegex.hasMatch(input)) {
      setState(() {
        errorMessage = widget.errorMessage;
        widget.onErrorChanged(true);
      });
    }
    else {
      setState(() {
        errorMessage = null;
        widget.onErrorChanged(false);
      });
    }

    widget.onChanged(input);
  }

  @override
  Widget build(BuildContext context) {
    double fontSize = standardFontSize(context);
    double fieldWidth = uiWidth(context);

    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: <Widget>[
        SizedBox(
          width: fieldWidth,
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: <Widget>[
              Text(
                widget.labelText,
                style: TextStyle(
                  fontStyle: FontStyle.italic,
                  color: textColor,
                  fontSize: fontSize
                )
              ),
              if (errorMessage != null) Text(
                errorMessage!,
                style: TextStyle(
                  color: errorColor,
                  fontSize: fontSize,
                )
              )
            ]
          )
        ),
        const SizedBox(height: 8),
        SizedBox(
          width: fieldWidth,
          child: TextField(
            cursorColor: textColor,
            onChanged: _validateInput,
            obscureText: widget.obscureText,
            style: TextStyle(
              color: textColor,
              fontSize: fontSize
            ),
            decoration: InputDecoration(
              enabledBorder: OutlineInputBorder(
                borderSide: BorderSide(
                  color: errorMessage != null ? errorColor.withAlpha(unselectedAlpha) : Theme.of(context).primaryColor.withAlpha(unselectedAlpha),
                  width: 3
                ),
                borderRadius: BorderRadius.circular(10),
              ),
              focusedBorder: OutlineInputBorder(
                borderSide: BorderSide(
                  color: errorMessage != null ? errorColor : Theme.of(context).primaryColor,
                  width: 3
                ),
                borderRadius: BorderRadius.circular(10)
              )
            )
          )
        )
      ],
    );
  }
}