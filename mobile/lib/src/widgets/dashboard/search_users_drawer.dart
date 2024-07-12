import 'package:flutter/material.dart';
import 'package:oncology_quest_mobile/src/utilities/colors.dart';
import 'package:oncology_quest_mobile/src/utilities/sizing.dart';
import 'package:oncology_quest_mobile/src/widgets/dashboard/section_heading.dart';

class SearchUsersDrawer extends StatelessWidget {
  const SearchUsersDrawer({super.key});

  @override
  Widget build(BuildContext context) {
    return SizedBox(
      width: drawerUiWidth(context),
      child: Material(
        color: backgroundColor2,
        child: Padding(
          padding: EdgeInsets.only(
            top: inMobileViewport(context) ? kToolbarHeight : 20,
            left: 20,
            right: 20,
            bottom: 15
          ),
          child: const Column(
            children: <Widget>[
              Row(
                mainAxisAlignment: MainAxisAlignment.center,
                children: <Widget>[
                  SectionHeading(title: 'Search Users'),
                ]
              )
            ]
          ),
        )
      ),
    );
  }
}