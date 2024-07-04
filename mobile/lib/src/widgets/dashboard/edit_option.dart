import 'package:flutter/material.dart';
import 'package:oncology_quest_mobile/src/utilities/colors.dart';

class EditOption extends StatelessWidget {
  const EditOption({
    super.key,
    required this.context,
    required this.isEditing,
    required this.onTap,
  });

  final BuildContext context;
  final bool isEditing;
  final void Function() onTap;

  @override
  Widget build(BuildContext context) {
    return InkWell(
      borderRadius: BorderRadius.circular(15),
      onTap: onTap,
      splashColor: isEditing ? textColor : okColor,
      child: Padding(
        padding: const EdgeInsets.all(7),
        child: Row(
          children: <Widget>[
            Icon(
              isEditing ? Icons.done : Icons.edit,
              color: isEditing ? okColor : textColor,
              size: MediaQuery.of(context).size.width * 0.06
            ),
            const SizedBox(width: 5),
            Text(
              isEditing ? 'Done' : 'Edit',
              style: TextStyle(
                color: isEditing ? okColor : textColor,
                fontSize: MediaQuery.of(context).size.width * 0.042
              )
            )
          ]
        ),
      )
    );
  }
}