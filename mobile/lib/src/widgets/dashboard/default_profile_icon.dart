import 'package:flutter/material.dart';

class DefaultProfileIcon extends StatelessWidget {
  final double size;
  final String name;
  final Function()? onTap;

  const DefaultProfileIcon({
    super.key,
    required this.size,
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
        padding: const EdgeInsets.all(4),
        decoration: BoxDecoration(
          shape: BoxShape.circle,
          color: Theme.of(context).primaryColor,
        ),
        child: Container(
          width: size,
          height: size,
          alignment: Alignment.center,
          child: Text(
            initials,
            style: TextStyle(
              color: Theme.of(context).textTheme.bodyLarge!.color,
              fontSize: size * 0.5
            ),
            maxLines: 1,
            overflow: TextOverflow.clip
          ),
        )
      )
    );
  }
}