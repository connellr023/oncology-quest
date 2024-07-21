import 'package:flutter/material.dart';
import 'package:flutter/foundation.dart';
import 'package:flutter_dotenv/flutter_dotenv.dart';

final Uri apiEndpoint = Uri.parse(dotenv.env['API_ENDPOINT']!);

const themeColor = Color(0xFF331BBF);
const textColor = Color(0xFFE7E7E7);
const errorColor = Color(0xFFE60A1C);
const warningColor = Color(0xFFD7A51B);
const okColor = Color(0xFF11B814);
const backgroundColor1 = Color(0xFF080808);
const backgroundColor2 = Color(0xFF181818);
const backgroundColor3 = Color(0xFF1F1F1F);

final RegExp usernameRegex = RegExp(r'^[a-zA-Z0-9\-_\.]{1,25}$');
final RegExp nameRegex = RegExp(r'^[a-zA-Z\s]{1,35}$');
final RegExp passwordRegex = RegExp(r'^.{8,200}$');
final RegExp commentRegex = RegExp(r'^[a-zA-Z0-9\s.,!?"()-]{0,150}$');
final RegExp entryTitleRegex = RegExp(r'^[a-zA-Z0-9+\-/()\s]{1,100}$');
final RegExp resetTokenRegex = RegExp(r'^[a-zA-Z0-9]{4}$');

void displayError(BuildContext context, String errorMessage) {
  ScaffoldMessenger.of(context).showSnackBar(SnackBar(
    content: Text(errorMessage),
    backgroundColor: errorColor
  ));
}

Future<bool> attemptFallible(BuildContext context, Future<String?> Function() future) async {
  final String? errorMessage = await future();

  if (errorMessage != null && context.mounted) {
    displayError(context, errorMessage);
    return false;
  }

  return true;
}


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

double drawerUiWidth(BuildContext context) {
  return (MediaQuery.of(context).size.width * 0.85).clamp(200, 500);
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