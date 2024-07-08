import 'package:flutter/material.dart';
import 'package:oncology_quest_mobile/src/utilities/colors.dart';

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