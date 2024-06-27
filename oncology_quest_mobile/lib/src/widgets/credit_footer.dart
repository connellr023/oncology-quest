import 'package:flutter/material.dart';

class Footer extends StatelessWidget {
  const Footer({super.key});

  @override
  Widget build(BuildContext context) {
    return Container(
      padding: const EdgeInsets.all(16),
      color: Theme.of(context).scaffoldBackgroundColor,
      child: Row(
        mainAxisAlignment: MainAxisAlignment.center,
        children: <Widget>[
          Text(
            '\u00A9 Connell Reffo 2024',
            style: TextStyle(
              color: Theme.of(context).textTheme.bodySmall!.color,
              fontSize: MediaQuery.of(context).size.width * 0.035
            ),
          ),
        ],
      ),
    );
  }
}