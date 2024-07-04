import 'package:flutter/material.dart';
import 'package:flutter_svg/svg.dart';

class Graphic extends StatelessWidget {
  final double imageWidth;

  const Graphic({
    super.key,
    required this.imageWidth,
  });

  @override
  Widget build(BuildContext context) {
    return SvgPicture.asset(
      'assets/images/graphic.svg',
      width: imageWidth
    );
  }
}