import 'package:flutter/material.dart';

class DefaultProfileIcon extends StatelessWidget {
  final String name;

  const DefaultProfileIcon({
    super.key,
    required this.name
  });

  @override
  Widget build(BuildContext context) {
    String initials = name.isNotEmpty ? name.substring(0, 2) : "?";

    return CircleAvatar(
      backgroundColor: Theme.of(context).primaryColor,
      radius: MediaQuery.of(context).size.width * 0.05,
      child: Text(
        initials,
        style: TextStyle(
          fontSize: MediaQuery.of(context).size.width * 0.04,
          color: Theme.of(context).textTheme.bodySmall!.color
        ) // Customize the text style
      )
    );
  }
}