import 'package:flutter/material.dart';
import 'package:flutter_svg/svg.dart';

class MainLogo extends StatelessWidget {
  final double imageSize;

  const MainLogo({
    super.key,
    required this.imageSize,
  });

  @override
  Widget build(BuildContext context) {
    return SvgPicture.asset(
      'assets/images/logo.svg',
      width: imageSize,
      height: imageSize,
      colorFilter: ColorFilter.mode(
        Theme.of(context).primaryColor,
        BlendMode.srcIn,
      )
    );
  }
}