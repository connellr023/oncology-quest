import 'package:flutter/material.dart';

class SelectedRotationState extends ChangeNotifier {
  int? selectedRotationId;

  void selectRotation(int? rotationId) {
    selectedRotationId = rotationId;
    notifyListeners();
  }
}