import 'package:flutter/material.dart';

class ThematicElevatedButton extends StatelessWidget {
  final String text;
  final VoidCallback onPressed;
  final bool isDisabled;

  const ThematicElevatedButton({
    super.key,
    required this.text,
    required this.onPressed,
    this.isDisabled = false,
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
        onPressed: () => { if (!isDisabled) onPressed() },
        style: ElevatedButton.styleFrom(
          backgroundColor: isDisabled ? Theme.of(context).primaryColor.withOpacity(0.4) : Theme.of(context).primaryColor,
          foregroundColor: Theme.of(context).textTheme.bodySmall!.color,
          padding: const EdgeInsets.symmetric(horizontal: 40, vertical: 5),
          textStyle: TextStyle(
            fontSize: fontSize,
          ),
          shape: RoundedRectangleBorder(
            borderRadius: BorderRadius.circular(12),
          ),
          overlayColor: Colors.black
        ),
        child: Text(text),
      )
    );
  }
}