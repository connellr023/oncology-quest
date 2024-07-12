import 'package:flutter/material.dart';
import 'package:oncology_quest_mobile/src/utilities/colors.dart';
import 'package:oncology_quest_mobile/src/utilities/sizing.dart';

class PanelInputOption extends StatelessWidget {
  final bool isError;
  final bool isDisabled;
  final String hintText;
  final String? defaultValue;
  final Color backgroundColor;
  final Function(String) onChanged;

  const PanelInputOption({
    super.key,
    this.isError = false,
    this.isDisabled = false,
    required this.hintText,
    this.defaultValue,
    this.backgroundColor = backgroundColor3,
    required this.onChanged
  });

  @override
  Widget build(BuildContext context) {
    final size = modalFontSize(context);
    final outlinedBorder = OutlineInputBorder(
      borderRadius: BorderRadius.circular(20),
      borderSide: isError ? const BorderSide(color: errorColor, width: 3) : BorderSide.none
    );

    return TextFormField(
      enabled: !isDisabled,
      initialValue: defaultValue,
      style: TextStyle(
        color: isError ? errorColor : textColor,
        fontSize: size
      ),
      decoration: InputDecoration(
        hintText: hintText,
        hintStyle: TextStyle(
          color: textColor.withOpacity(0.5),
          fontSize: size
        ),
        enabledBorder: outlinedBorder,
        disabledBorder: outlinedBorder,
        focusedBorder: outlinedBorder,
        filled: true,
        fillColor: backgroundColor
      ),
      onChanged: onChanged
    );
  }
}