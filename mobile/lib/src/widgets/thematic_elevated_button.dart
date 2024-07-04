import 'package:flutter/material.dart';
import 'package:oncology_quest_mobile/src/utilities/colors.dart';

class ThematicElevatedButton extends StatelessWidget {
  final String text;
  final VoidCallback onPressed;
  final bool isDisabled;
  final bool isLoading;

  const ThematicElevatedButton({
    super.key,
    required this.text,
    required this.onPressed,
    this.isDisabled = false,
    this.isLoading = false
  });

  @override
  Widget build(BuildContext context) {
    double fontSize = MediaQuery.of(context).size.width * 0.05;
    double buttonWidth = MediaQuery.of(context).size.width * 0.8;
    const double buttonHeight = 60;

    return SizedBox(
      width: buttonWidth,
      height: buttonHeight,
      child: ElevatedButton(
        onPressed: () => { if (!isDisabled && !isLoading) onPressed() },
        style: ElevatedButton.styleFrom(
          backgroundColor: (isDisabled || isLoading) ? themeColor.withOpacity(0.4) : themeColor,
          foregroundColor: textColor,
          padding: const EdgeInsets.symmetric(horizontal: 40, vertical: 5),
          textStyle: TextStyle(
            fontSize: fontSize,
          ),
          shape: RoundedRectangleBorder(
            borderRadius: BorderRadius.circular(12),
          ),
          overlayColor: Colors.black
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