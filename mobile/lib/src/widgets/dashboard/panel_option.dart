import 'package:flutter/material.dart';
import 'package:oncology_quest_mobile/src/utilities/colors.dart';

class PanelOption extends StatelessWidget {
  const PanelOption({
    super.key,
    required this.text,
    required this.icon,
    required this.onTap,
    required this.color,
  });

  final String text;
  final IconData icon;
  final Function() onTap;
  final Color color;

  @override
  Widget build(BuildContext context) {
    return InkWell(
      splashColor: color,
      borderRadius: BorderRadius.circular(20), // Set the border radius for the splash effect
      onTap: onTap,
      child: ListTile(
        leading: Icon(icon, color: textColor),
        title: Text(
          text,
          style: const TextStyle(color: textColor),
        ),
      ),
    );
  }
}