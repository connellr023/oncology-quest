import 'package:flutter/material.dart';
import 'package:oncology_quest_mobile/src/utilities/sizing.dart';

class TwoVariantOption extends StatelessWidget {
  final Color firstColor;
  final Color secondColor;
  final IconData firstIcon;
  final IconData secondIcon;
  final String firstText;
  final String secondText;
  final bool inFirstVariant;
  final bool isDisabled;
  final BuildContext context;
  final void Function() onTap;

  const TwoVariantOption({
    super.key,
    required this.firstColor,
    required this.secondColor,
    required this.firstIcon,
    required this.secondIcon,
    required this.firstText,
    required this.secondText,
    required this.context,
    required this.inFirstVariant,
    this.isDisabled = false,
    required this.onTap,
  });

  @override
  Widget build(BuildContext context) {
    final size = standardFontSize(context);
    final color = inFirstVariant ? firstColor : secondColor;
    const double disabledOpacity = 0.8;

    return InkWell(
      borderRadius: BorderRadius.circular(15),
      onTap: () => { if (!isDisabled) onTap() },
      splashColor: isDisabled ? color.withOpacity(0.3) : inFirstVariant ? secondColor : firstColor,
      child: Padding(
        padding: const EdgeInsets.all(7),
        child: Row(
          children: <Widget>[
            Icon(
              inFirstVariant ? firstIcon : secondIcon,
              color: isDisabled ? color.withOpacity(disabledOpacity) : color,
              size: size
            ),
            const SizedBox(width: 5),
            Text(
              inFirstVariant ? firstText : secondText,
              style: TextStyle(
                color: isDisabled ? color.withOpacity(disabledOpacity) : color,
                fontSize: size
              )
            )
          ]
        ),
      )
    );
  }
}