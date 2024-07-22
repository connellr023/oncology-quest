import 'package:flutter/material.dart';
import 'package:oncology_quest_mobile/src/utilities.dart';

class SectionHeading extends StatelessWidget {
  final String title;
  final List<Widget> children;

  const SectionHeading({
    super.key,
    required this.title,
    this.children = const []
  });

  @override
  Widget build(BuildContext context) {
    return Column(
      children: <Widget>[
        Row(
          children: <Widget>[
            Text(
              title,
              textAlign: TextAlign.left,
              style: TextStyle(
                color: textColor,
                fontSize: headingFontSize(context)
              )
            ),
            if (children.isNotEmpty) ...<Widget>[
              const Spacer(),
              ...children
            ]
          ]
        ),
        const SizedBox(height: 13)
      ]
    );
  }
}