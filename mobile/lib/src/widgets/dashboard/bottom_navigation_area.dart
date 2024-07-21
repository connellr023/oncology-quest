import 'package:flutter/material.dart';
import 'package:oncology_quest_mobile/src/utilities.dart';
import 'package:oncology_quest_mobile/src/widgets/dashboard/basic_option.dart';

class BottomNavigationArea extends StatelessWidget {
  final void Function() onSearchTap;

  const BottomNavigationArea({
    super.key,
    required this.onSearchTap
  });

  @override
  Widget build(BuildContext context) {
    return Material(
      color: backgroundColor1,
      child: Padding(
        padding: const EdgeInsets.only(
          top: 9,
          bottom: 7
        ),
        child: Row(
          mainAxisAlignment: MainAxisAlignment.spaceEvenly,
          children: <Widget>[
            BasicOption(
              title: 'Search Users',
              color: textColor,
              icon: Icons.search_rounded,
              onTap: onSearchTap
            )
          ]
        )
      )
    );
  }
}