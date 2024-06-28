import 'package:flutter/material.dart';

class FormTextField extends StatefulWidget {
  final IconData prefixIconData;
  final String hintText;
  final bool obscureText;

  const FormTextField({
    super.key,
    required this.prefixIconData,
    required this.hintText,
    required this.obscureText
  });

  @override
  State<FormTextField> createState() => _FormTextFieldState();
}

class _FormTextFieldState extends State<FormTextField> {
  @override
  Widget build(BuildContext context) {
    double fontSize = MediaQuery.of(context).size.width * 0.04;
    double fieldWidth = MediaQuery.of(context).size.width * 0.7;
    const double fieldHeight = 60;

    return SizedBox(
      width: fieldWidth,
      height: fieldHeight,
      child: TextField(
        obscureText: widget.obscureText,
        style: TextStyle(
          color: Theme.of(context).textTheme.bodySmall!.color,
          fontSize: fontSize
        ),
        decoration: InputDecoration(
          prefixIcon: Icon(
            widget.prefixIconData,
            color: Theme.of(context).primaryColor,
          ),
          hintText: widget.hintText,
          hintStyle: TextStyle(
            color: Colors.white.withAlpha(200),
            fontSize: fontSize
          ),
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
    );
  }
}