import 'package:flutter/material.dart';
import 'package:oncology_quest_mobile/src/utilities/sizing.dart';

class CreditFooter extends StatelessWidget {
  const CreditFooter({super.key});

  @override
  Widget build(BuildContext context) {
    return Container(
      padding: const EdgeInsets.all(16),
      color: Colors.transparent,
      child: Row(
        mainAxisAlignment: MainAxisAlignment.center,
        children: <Widget>[
          Text(
            '\u00A9 Connell Reffo 2024',
            style: TextStyle(
              color: Theme.of(context).textTheme.bodySmall!.color,
              fontSize: standardFontSize(context)
            ),
          ),
        ],
      ),
    );
  }
}