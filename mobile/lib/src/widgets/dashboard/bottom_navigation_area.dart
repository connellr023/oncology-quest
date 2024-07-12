import 'package:flutter/material.dart';
import 'package:oncology_quest_mobile/src/utilities/colors.dart';
import 'package:oncology_quest_mobile/src/widgets/dashboard/basic_option.dart';

class BottomNavigationArea extends StatelessWidget {
  const BottomNavigationArea({super.key});

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
              title: 'Dashboard',
              color: textColor,
              icon: Icons.home,
              onTap: () => {}
            ),
            BasicOption(
              title: 'Search',
              color: textColor,
              icon: Icons.search_rounded,
              onTap: () => {}
            )
          ]
        ),
      )
    );
  }
}