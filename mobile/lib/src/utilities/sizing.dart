import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart';

bool inMobileViewport(BuildContext context) {
  return MediaQuery.of(context).size.width < 600 || !kIsWeb;
}

double uiElementVerticalSpacing(BuildContext context) {
  return (MediaQuery.of(context).size.height * 0.02).clamp(12, 27);
}

double homeViewMainLogoSize(BuildContext context) {
  return (MediaQuery.of(context).size.width * 0.35).clamp(100, 200);
}

double secondaryViewMainLogoSize(BuildContext context) {
  return (MediaQuery.of(context).size.width * 0.3).clamp(80, 170);
}

double graphicWidth(BuildContext context) {
  return (MediaQuery.of(context).size.width * 0.65).clamp(200, 400);
}

double headingFontSize(BuildContext context) {
  return (MediaQuery.of(context).size.width * 0.04).clamp(20, 30);
}

double uiFontSize(BuildContext context) {
  return (MediaQuery.of(context).size.width * 0.04).clamp(19, 29);
}

double uiWidth(BuildContext context) {
  return (MediaQuery.of(context).size.width * 0.8).clamp(200, 500);
}

double mainUiButtonHeight(BuildContext context) {
  return (MediaQuery.of(context).size.height * 0.06).clamp(70, 85);
}

double secondaryUiButtonHeight(BuildContext context) {
  return (MediaQuery.of(context).size.height * 0.04).clamp(64, 70);
}

double standardFontSize(BuildContext context) {
  return (MediaQuery.of(context).size.width * 0.04).clamp(18, 23);
}