import 'package:flutter/material.dart';
import 'package:oncology_quest_mobile/src/models/session.dart';
import 'package:oncology_quest_mobile/src/utilities/colors.dart';
import 'package:oncology_quest_mobile/src/widgets/default_profile_icon.dart';

class DashboardAppBar extends StatelessWidget implements PreferredSizeWidget {
  final Session session;
  final void Function() onProfileTap;

  const DashboardAppBar({
    super.key,
    required this.session,
    required this.onProfileTap
  });

  @override
  Size get preferredSize => const Size.fromHeight(kToolbarHeight);

  @override
  Widget build(BuildContext context) {
    return AppBar(
      automaticallyImplyLeading: false,
      backgroundColor: backgroundColor1,
      title: Row(
        children: <Widget>[
          DefaultProfileIcon(
            name: session.user.name,
            onTap: () => onProfileTap(),
          ),
          const SizedBox(width: 10),
          RichText(
            text: TextSpan(
              style: TextStyle(
                fontSize: MediaQuery.of(context).size.width * 0.05,
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
    );
  }
}