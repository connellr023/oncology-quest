import 'package:flutter/material.dart';
import 'package:oncology_quest_mobile/src/utilities/colors.dart';

class PanelOption extends StatelessWidget {
  final String text;
  final IconData icon;
  final Function() onTap;
  final Color splashColor;
  final bool isDisabled;

  const PanelOption({
    super.key,
    required this.text,
    required this.icon,
    required this.onTap,
    required this.splashColor,
    this.isDisabled = false
  });

  @override
  Widget build(BuildContext context) {
    final renderColor = isDisabled ? textColor.withOpacity(0.5) : textColor;

    return InkWell(
      splashColor: isDisabled ? textColor.withOpacity(0.3) : splashColor,
      borderRadius: BorderRadius.circular(20),
      onTap: () => { if (!isDisabled) onTap() },
      child: ListTile(
        leading: Icon(icon, color: renderColor),
        title: Text(
          text,
          style: TextStyle(color: renderColor),
        ),
      ),
    );
  }
}