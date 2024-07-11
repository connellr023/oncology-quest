import 'package:flutter/material.dart';

class SelectedRotationState extends ChangeNotifier {
  int? _selectedRotationId;
  int? get selectedRotationId => _selectedRotationId;

  void selectRotation(int? rotationId) {
    _selectedRotationId = rotationId;
    notifyListeners();
  }
}