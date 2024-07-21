import 'package:flutter/material.dart';
import 'package:oncology_quest_mobile/src/models/client_user.dart';

class SelectedUserState extends ChangeNotifier {
  ClientUser? _selectedUser;
  ClientUser? get selectedUser => _selectedUser;

  void selectUser(ClientUser? user) {
    _selectedUser = user;
    notifyListeners();
  }
}