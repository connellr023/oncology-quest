import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart';
import 'package:oncology_quest_mobile/src/utilities/colors.dart';

void showInteractivePanel(BuildContext context, Widget panel) {
  if (inMobileViewport(context)) {
    showModalBottomSheet(
      context: context,
      isScrollControlled: true,
      backgroundColor: backgroundColor2,
      builder: (BuildContext context) => panel
    );
  }
  else {
    showDialog(
      context: context,
      builder: (BuildContext context) => Dialog(
        backgroundColor: backgroundColor2,
        child: SizedBox(
          width: uiWidth(context),
          child: panel
        )
      )
    );
  }
}

bool inMobileViewport(BuildContext context) {
  return MediaQuery.of(context).size.width < 600 || !kIsWeb;
}

double modalFontSize(BuildContext context) {
  return (MediaQuery.of(context).size.width * 0.04).clamp(16, 21);
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
  return (MediaQuery.of(context).size.width * 0.04).clamp(25, 32);
}

double uiFontSize(BuildContext context) {
  return (MediaQuery.of(context).size.width * 0.04).clamp(22, 29);
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