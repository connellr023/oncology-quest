import 'package:flutter/material.dart';

class ClientUser {
  final int id;
  final String username;
  final String name;
  final bool isAdmin;
  final int loginCount;

  ClientUser({
    required this.id,
    required this.username,
    required this.name,
    required this.isAdmin,
    required this.loginCount,
  });

  factory ClientUser.deserialize(Map<String, dynamic> json) {
    final id = int.tryParse(json['id'].toString());
    final username = json['username'].toString();
    final name = json['name'].toString();
    final isAdmin = bool.tryParse(json['isAdmin'].toString());
    final loginCount = int.tryParse(json['loginCount'].toString());

    if (id == null) {
      throw ErrorDescription('Failed to parse user ID.');
    }

    if (username.isEmpty) {
      throw ErrorDescription('Failed to parse username.');
    }

    if (name.isEmpty) {
      throw ErrorDescription('Failed to parse user name.');
    }

    if (isAdmin == null) {
      throw ErrorDescription('Failed to parse admin status.');
    }

    if (loginCount == null) {
      throw ErrorDescription('Failed to parse login count.');
    }

    return ClientUser(
      id: id,
      username: username,
      name: name,
      isAdmin: isAdmin,
      loginCount: loginCount,
    );
  }
}