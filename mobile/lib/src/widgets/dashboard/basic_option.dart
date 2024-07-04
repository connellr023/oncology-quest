import 'package:flutter/material.dart';

class BasicOption extends StatelessWidget {
  const BasicOption({
    super.key,
    required this.context,
    required this.title,
    required this.color,
    required this.icon,
    required this.onTap,
  });

  final BuildContext context;
  final String title;
  final Color color;
  final IconData icon;
  final void Function() onTap;

  @override
  Widget build(BuildContext context) {
    return InkWell(
      borderRadius: BorderRadius.circular(15),
      onTap: onTap,
      splashColor: color,
      child: Padding(
        padding: const EdgeInsets.all(7),
        child: Row(
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
    );
  }
}