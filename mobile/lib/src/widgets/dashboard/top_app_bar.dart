import 'package:flutter/material.dart';
import 'package:oncology_quest_mobile/src/models/session.dart';
import 'package:oncology_quest_mobile/src/utilities/colors.dart';
import 'package:oncology_quest_mobile/src/utilities/sizing.dart';
import 'package:oncology_quest_mobile/src/widgets/dashboard/default_profile_icon.dart';

class TopAppBar extends StatelessWidget implements PreferredSizeWidget {
  final double padding;
  final Session session;
  final void Function() onProfileTap;

  const TopAppBar({
    super.key,
    required this.session,
    required this.onProfileTap,
    this.padding = 8.0
  });

  @override
  Size get preferredSize => Size.fromHeight(kToolbarHeight + padding);

  @override
  Widget build(BuildContext context) {
    final size = standardFontSize(context);

    return AppBar(
      scrolledUnderElevation: 0.0,
      automaticallyImplyLeading: false,
      backgroundColor: backgroundColor1,
      elevation: 0,
      title: Padding(
        padding: EdgeInsets.only(top: padding),
        child: Row(
          children: <Widget>[
            DefaultProfileIcon(
              size: size,
              name: session.user.name,
              onTap: () => onProfileTap(),
            ),
            const SizedBox(width: 10),
            RichText(
              text: TextSpan(
                style: TextStyle(
                  fontSize: size,
                  color: textColor,
                ),
                children: <TextSpan>[
                  TextSpan(
                    text: session.user.name,
                    style: const TextStyle(fontWeight: FontWeight.bold)
                  ),
                  TextSpan(text: ' (${session.user.username})')
                ]
              )
            )
          ]
        )
      )
    );
  }
}