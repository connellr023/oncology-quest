import 'package:flutter/material.dart';
import 'package:oncology_quest_mobile/src/utilities/colors.dart';
import 'package:oncology_quest_mobile/src/utilities/sizing.dart';

class SectionHeading extends StatelessWidget {
  final String title;

  const SectionHeading({
    super.key,
    required this.title,
  });

  @override
  Widget build(BuildContext context) {
    return Column(
      children: <Widget>[
        Row(
          children: [
            Text(
              title,
              textAlign: TextAlign.left,
              style: TextStyle(
                color: textColor,
                fontSize: headingFontSize(context)
              )
            )
          ]
        ),
        const SizedBox(height: 13)
      ]
    );
  }
}