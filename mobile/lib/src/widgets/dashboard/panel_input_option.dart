import 'package:flutter/material.dart';
import 'package:oncology_quest_mobile/src/utilities/colors.dart';

class PanelInputOption extends StatelessWidget {
  final bool isError;
  final String hintText;
  final Color splashColor;
  final Function(String) onChanged;

  const PanelInputOption({
    super.key,
    this.isError = false,
    required this.hintText,
    required this.onChanged,
    required this.splashColor,
  });

  @override
  Widget build(BuildContext context) {
    final outlinedBorder = OutlineInputBorder(
      borderRadius: BorderRadius.circular(20),
      borderSide: isError ? const BorderSide(color: errorColor, width: 3) : BorderSide.none
    );

    return TextField(
      style: TextStyle(
        color: isError ? errorColor : textColor
      ),
      decoration: InputDecoration(
        hintText: hintText,
        hintStyle: TextStyle(
          color: textColor.withOpacity(0.5)
        ),
        enabledBorder: outlinedBorder,
        focusedBorder: outlinedBorder,
        filled: true,
        fillColor: backgroundColor3
      ),
      onChanged: onChanged
    );
  }
}