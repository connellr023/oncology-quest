import 'package:flutter/material.dart';

class Rotation {
  final int id;
  final String name;
  final DateTime lastUpdated;

  Rotation({
    required this.id,
    required this.name,
    required this.lastUpdated,
  });

  factory Rotation.deserialize(Map<String, dynamic> json) {
    final id = int.tryParse(json['id'].toString());
    final name = json['name'].toString();
    final lastUpdated = DateTime.tryParse(json['lastUpdated'].toString());

    if (id == null) {
      throw ErrorDescription('Failed to parse rotation ID.');
    }

    if (name.isEmpty) {
      throw ErrorDescription('Failed to parse rotation name.');
    }

    if (lastUpdated == null) {
      throw ErrorDescription('Failed to parse last updated date.');
    }

    return Rotation(
      id: id,
      name: name,
      lastUpdated: lastUpdated,
    );
  }
}