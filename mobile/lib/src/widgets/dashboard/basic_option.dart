import 'package:flutter/material.dart';

class BasicOption extends StatelessWidget {
  final Color backgroundColor;
  final EdgeInsetsGeometry padding;
  final double borderRadius;

  const BasicOption({
    super.key,
    required this.context,
    required this.title,
    required this.color,
    required this.icon,
    required this.onTap,
    this.backgroundColor = Colors.transparent,
    this.padding = const EdgeInsets.all(7),
    this.borderRadius = 15
  });

  final BuildContext context;
  final String title;
  final Color color;
  final IconData icon;
  final void Function() onTap;

  @override
  Widget build(BuildContext context) {
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
                size: MediaQuery.of(context).size.width * 0.06
              ),
              const SizedBox(width: 5),
              Text(
                title,
                style: TextStyle(
                  color: color,
                  fontSize: MediaQuery.of(context).size.width * 0.042
                )
              )
            ]
          ),
        )
      ),
    );
  }
}