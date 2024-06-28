import 'package:flutter/material.dart';

class FormTextField extends StatefulWidget {
  final String labelText;
  final RegExp validationRegex;
  final String errorMessage;
  final bool obscureText;

  const FormTextField({
    super.key,
    required this.labelText,
    required this.validationRegex,
    required this.errorMessage,
    required this.obscureText
  });

  @override
  State<FormTextField> createState() => _FormTextFieldState();
}

class _FormTextFieldState extends State<FormTextField> {
  String? errorMessage;

  void _validateInput(String input) {
    if (!widget.validationRegex.hasMatch(input)) {
      setState(() {
        errorMessage = widget.errorMessage;
      });
    } else {
      setState(() {
        errorMessage = null;
      });
    }
  }

  @override
  Widget build(BuildContext context) {
    double fontSize = MediaQuery.of(context).size.width * 0.04;
    double fieldWidth = MediaQuery.of(context).size.width * 0.8;

    const double fieldHeight = 60;

    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: <Widget>[
        Text(
          widget.labelText,
          style: TextStyle(
            fontStyle: FontStyle.italic,
            color: Theme.of(context).textTheme.bodySmall!.color,
            fontSize: fontSize
          ),
        ),
        if (errorMessage != null) Text(
          errorMessage!,
          style: TextStyle(
            color: Colors.red,
            fontSize: fontSize,
          )
        ),
        const SizedBox(height: 6),
        SizedBox(
          width: fieldWidth,
          height: fieldHeight,
          child: TextField(
            onChanged: _validateInput,
            obscureText: widget.obscureText,
            style: TextStyle(
              color: Theme.of(context).textTheme.bodySmall!.color,
              fontSize: fontSize
            ),
            decoration: InputDecoration(
              enabledBorder: OutlineInputBorder(
                borderSide: BorderSide(
                  color: Theme.of(context).primaryColor.withAlpha(150),
                  width: 3,
                ),
                borderRadius: BorderRadius.circular(10),
              ),
              focusedBorder: OutlineInputBorder(
                borderSide: BorderSide(
                  color: Theme.of(context).primaryColor,
                  width: 3,
                ),
                borderRadius: BorderRadius.circular(10),
              ),
            ),
          )
        )
      ],
    );
  }
}