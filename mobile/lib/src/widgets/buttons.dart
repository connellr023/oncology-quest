import 'package:flutter/material.dart';
import 'package:oncology_quest_mobile/src/utilities.dart';

class MonotoneElevatedButton extends StatelessWidget {
  final double width;
  final double height;
  final String text;
  final VoidCallback onPressed;
  final bool isDisabled;
  final bool isLoading;

  const MonotoneElevatedButton({
    super.key,
    required this.width,
    required this.height,
    required this.text,
    required this.onPressed,
    this.isDisabled = false,
    this.isLoading = false
  });

  @override
  Widget build(BuildContext context) {
    return ThematicElevatedButton(
      width: width,
      height: height,
      text: text,
      onPressed: onPressed,
      isLoading: isLoading,
      isDisabled: isDisabled,
      fontColor: textColor,
      backgroundColor: Colors.transparent,
      splashColor: textColor,
      border: const BorderSide(
        color: textColor,
        width: 3
      )
    );
  }
}

class ThematicElevatedButton extends StatelessWidget {
  final double width;
  final double height;
  final String text;
  final VoidCallback onPressed;
  final bool isDisabled;
  final bool isLoading;
  final Color fontColor;
  final Color backgroundColor;
  final Color splashColor;
  final BorderSide border;

  const ThematicElevatedButton({
    super.key,
    required this.width,
    required this.height,
    required this.text,
    required this.onPressed,
    this.isDisabled = false,
    this.isLoading = false,
    this.fontColor = textColor,
    this.backgroundColor = themeColor,
    this.splashColor = Colors.black,
    this.border = BorderSide.none
  });

  @override
  Widget build(BuildContext context) {
    double fontSize = uiFontSize(context);

    return SizedBox(
      width: width,
      height: height,
      child: ElevatedButton(
        onPressed: () => { if (!isDisabled && !isLoading) onPressed() },
        style: ElevatedButton.styleFrom(
          backgroundColor: (isDisabled || isLoading) ? backgroundColor.withOpacity(0.4) : backgroundColor,
          foregroundColor: fontColor,
          padding: const EdgeInsets.symmetric(horizontal: 40, vertical: 5),
          textStyle: TextStyle(
            fontSize: fontSize,
          ),
          shape: RoundedRectangleBorder(
            borderRadius: BorderRadius.circular(12),
            side: border
          ),
          overlayColor: splashColor
        ),
        child: Row(
          mainAxisAlignment: MainAxisAlignment.center,
          children: <Widget>[
            if (isLoading) SizedBox(
              width: fontSize,
              height: fontSize,
              child: const CircularProgressIndicator(
                valueColor: AlwaysStoppedAnimation<Color>(textColor),
                strokeWidth: 2,
              )
            )
            else Text(text)
          ]
        )
      )
    );
  }
}