import 'package:flutter/material.dart';

class MonotoneElevatedButton extends StatelessWidget {
  final String text;
  final VoidCallback onPressed;

  const MonotoneElevatedButton({
    super.key,
    required this.text,
    required this.onPressed,
  });

  @override
  Widget build(BuildContext context) {
    double fontSize = MediaQuery.of(context).size.width * 0.05;
    double buttonWidth = MediaQuery.of(context).size.width * 0.8;
    const double buttonHeight = 60;

    return SizedBox(
      width: buttonWidth,
      height: buttonHeight,
      child: OutlinedButton(
        onPressed: onPressed,
        style: OutlinedButton.styleFrom(
          foregroundColor: Colors.white,
          backgroundColor: Theme.of(context).scaffoldBackgroundColor,
          side: const BorderSide(color: Colors.white, width: 3),
          padding: const EdgeInsets.symmetric(horizontal: 40, vertical: 5),
          textStyle: TextStyle(
            fontSize: fontSize,
          ),
          shape: RoundedRectangleBorder(
            borderRadius: BorderRadius.circular(12),
          ),
          overlayColor: Colors.white
        ),
        child: Text(text),
      )
    );
  }
}