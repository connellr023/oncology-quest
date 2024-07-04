import 'package:flutter/material.dart';

class DefaultProfileIcon extends StatelessWidget {
  final String name;
  final Function()? onTap;

  const DefaultProfileIcon({
    super.key,
    required this.name,
    required this.onTap
  });

  @override
  Widget build(BuildContext context) {
    String initials = name.isNotEmpty ? name.substring(0, 2) : "?";

    return InkWell(
      onTap: onTap,
      borderRadius: BorderRadius.circular(50),
      splashColor: Colors.white,
      child: Ink(
        decoration: BoxDecoration(
          shape: BoxShape.circle,
          color: Theme.of(context).primaryColor,
        ),
        child: Padding(
          padding: const EdgeInsets.all(10.0),
          child: Text(
            initials,
            style: TextStyle(
              color: Theme.of(context).textTheme.bodyLarge!.color,
              fontSize: MediaQuery.of(context).size.width * 0.04,
            ),
          ),
        ),
      ),
    );
  }
}