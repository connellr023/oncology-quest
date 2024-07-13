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
          child: Column(
            children: <Widget>[
              const Row(
                mainAxisAlignment: MainAxisAlignment.center,
                children: <Widget>[
                  SectionHeading(title: 'Search Users'),
                ]
              ),
              const SizedBox(height: 12),
              _buildSearchField(context)
            ]
          )
        )
      )
    );
  }

  Widget _buildSearchField(BuildContext context) {
    final size = standardFontSize(context);

    return Container(
      decoration: BoxDecoration(
        color: backgroundColor3,
        borderRadius: BorderRadius.circular(18)
      ),
      child: TextField(
        style: TextStyle(
          color: textColor,
          fontSize: size
        ),
        decoration: InputDecoration(
          hintText: 'Search...',
          hintStyle: TextStyle(
            color: textColor.withOpacity(0.5),
            fontSize: size
          ),
          border: InputBorder.none,
          contentPadding: const EdgeInsets.symmetric(
            horizontal: 17,
            vertical: 17
          ),
          suffixIcon: Icon(
            Icons.search,
            color: textColor.withOpacity(0.5),
            size: size * 1.65
          )
        )
      )
    );
  }
}