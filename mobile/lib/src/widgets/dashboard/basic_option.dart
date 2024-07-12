import 'package:flutter/material.dart';
import 'package:oncology_quest_mobile/src/utilities/sizing.dart';

class BasicOption extends StatelessWidget {
  final Color backgroundColor;
  final EdgeInsetsGeometry padding;
  final double borderRadius;
  final String title;
  final Color color;
  final IconData icon;
  final void Function() onTap;

  const BasicOption({
    super.key,
    required this.title,
    required this.color,
    required this.icon,
    required this.onTap,
    this.backgroundColor = Colors.transparent,
    this.padding = const EdgeInsets.all(7),
    this.borderRadius = 15
  });

  @override
  Widget build(BuildContext context) {
    final size = standardFontSize(context);

    return Material(
      color: backgroundColor,
      borderRadius: BorderRadius.circular(borderRadius),
      child: InkWell(
        borderRadius: BorderRadius.circular(borderRadius),
        onTap: onTap,
        splashColor: color,
        child: Padding(
          padding: padding,
          child: Row(
            mainAxisAlignment: MainAxisAlignment.center,
            children: <Widget>[
              Icon(
                icon,
                color: color,
                size: size
              ),
              const SizedBox(width: 5),
              Text(
                title,
                style: TextStyle(
                  color: color,
                  fontSize: size
                )
              )
            ]
          ),
        )
      ),
    );
  }
}