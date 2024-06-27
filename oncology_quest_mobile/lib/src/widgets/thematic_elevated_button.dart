import 'package:flutter/material.dart';

class ThematicElevatedButton extends StatelessWidget {
  final String text;
  final VoidCallback onPressed;

  const ThematicElevatedButton({
    super.key,
    required this.text,
    required this.onPressed,
  });

  @override
  Widget build(BuildContext context) {
    return ElevatedButton(
      onPressed: onPressed,
      style: ElevatedButton.styleFrom(
        backgroundColor: Theme.of(context).primaryColor,
        foregroundColor: Theme.of(context).textTheme.bodySmall!.color,
        padding: const EdgeInsets.symmetric(horizontal: 40, vertical: 5),
        textStyle: TextStyle(
          fontSize: MediaQuery.of(context).size.width * 0.035,
        ),
        shape: RoundedRectangleBorder(
          borderRadius: BorderRadius.circular(12),
        ),
        overlayColor: Colors.black
      ),
      child: Text(text),
    );
  }
}