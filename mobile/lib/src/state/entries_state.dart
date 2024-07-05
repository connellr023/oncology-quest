import 'package:flutter/material.dart';
import 'package:oncology_quest_mobile/src/models/entry_levels.dart';
import 'package:oncology_quest_mobile/src/utilities/endpoint.dart';
import 'package:http/http.dart' as http;

import 'dart:convert';

class EntriesState extends ChangeNotifier {
  final Map<int, List<EntryHierarchy>> _entries = {};
  Map<int, List<EntryHierarchy>> get entries => _entries;

  Future<String?> fetchEntries(String? jwt, int rotationId) async {
    if (jwt == null) {
      return 'You must be logged in to fetch entries.';
    }

    if (_entries.containsKey(rotationId)) {
      return null;
    }

    try {
      final response = await http.get(apiEndpoint.resolve('/api/entries/$rotationId'),
        headers: {
          'content-type': 'application/json',
          'authorization': jwt
        }
      );

      if (response.statusCode == 200) {
        final body = json.decode(response.body);

        final entries = (body as List<dynamic>).map((entryJson) => EntryHierarchy.deserialize(entryJson)).toList();

        _entries[rotationId] = entries;
        notifyListeners();
      }
      else {
        return 'Failed to fetch entries. Please try again later.';
      }
    }
    catch (_) {
      return 'Failed to connect to server. Please try again later.';
    }

    return null;
  }
}