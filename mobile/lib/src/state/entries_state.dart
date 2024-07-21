import 'package:flutter/material.dart';
import 'package:oncology_quest_mobile/src/models/entry_levels.dart';
import 'package:oncology_quest_mobile/src/utilities.dart';
import 'package:http/http.dart' as http;

import 'dart:convert';

class EntriesState extends ChangeNotifier {
  /// Memoized entries.
  /// Map of rotation ID to list of entries.
  final Map<int, List<EntryHierarchy>> _entriesMemo = {};
  Map<int, List<EntryHierarchy>> get entries => _entriesMemo;

  Future<void> cacheAndMemoizeEntries(int rotationId, List<EntryHierarchy> entries) async {
    // TODO: Cache entries in local storage
    _entriesMemo[rotationId] = entries;
  }

  void clearMemo() {
    _entriesMemo.clear();
  }

  Future<String?> fetchEntries(String jwt, int rotationId) async {
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

        await cacheAndMemoizeEntries(rotationId, entries);
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

  Future<String?> createSupertask(String jwt, String title, int rotationId) async {
    try {
      final response = await http.post(apiEndpoint.resolve('/api/entries/supertasks/create'),
        headers: {
          'content-type': 'application/json',
          'authorization': jwt
        },
        body: json.encode({
          'title': title,
          'rotationId': rotationId
        })
      );

      if (response.statusCode == 201) {
        final body = json.decode(response.body);
        final supertaskId = int.parse(body['entryId'].toString());

        _entriesMemo[rotationId]!.add(
          EntryHierarchy(
            hierarchy: EntryLevel(
              entry: Supertask(
                id: supertaskId,
                title: title,
                rotationId: rotationId
              ),
              children: []
            )
          )
        );

        await cacheAndMemoizeEntries(rotationId, _entriesMemo[rotationId]!);
        notifyListeners();
      }
      else {
        return 'Failed to create supertask. Please try again later.';
      }
    }
    catch (_) {
      return 'Failed to connect to server. Please try again later.';
    }

    return null;
  }

  Future<String?> createTask(String jwt, String title, int rotationId, int supertaskId, int supertaskIndex) async {
    try {
      final response = await http.post(apiEndpoint.resolve('/api/entries/tasks/create'),
        headers: {
          'content-type': 'application/json',
          'authorization': jwt
        },
        body: json.encode({
          'title': title,
          'rotationId': rotationId,
          'parentId': supertaskId
        })
      );

      if (response.statusCode == 201) {
        final body = json.decode(response.body);
        final taskId = int.parse(body['entryId'].toString());

        _entriesMemo[rotationId]![supertaskIndex].hierarchy.children.add(
          EntryLevel(
            entry: Task(
              id: taskId,
              title: title,
              rotationId: rotationId,
              supertaskId: supertaskId
            ),
            children: []
          )
        );

        await cacheAndMemoizeEntries(rotationId, _entriesMemo[rotationId]!);
        notifyListeners();
      }
      else {
        return 'Failed to create task. Please try again later.';
      }
    }
    catch (_) {
      return 'Failed to connect to server. Please try again later.';
    }

    return null;
  }

  Future<String?> createSubtask(String jwt, String title, int rotationId, int taskId, int supertaskIndex, int taskIndex) async {
    try {
      final response = await http.post(apiEndpoint.resolve('/api/entries/subtasks/create'),
        headers: {
          'content-type': 'application/json',
          'authorization': jwt
        },
        body: json.encode({
          'title': title,
          'rotationId': rotationId,
          'parentId': taskId
        })
      );

      if (response.statusCode == 201) {
        final body = json.decode(response.body);
        final subtaskId = int.parse(body['entryId'].toString());

        _entriesMemo[rotationId]![supertaskIndex].hierarchy.children[taskIndex].children.add(
          Subtask(
            id: subtaskId,
            title: title,
            rotationId: rotationId,
            taskId: taskId
          )
        );

        await cacheAndMemoizeEntries(rotationId, _entriesMemo[rotationId]!);
        notifyListeners();
      }
      else {
        return 'Failed to create subtask. Please try again later.';
      }
    }
    catch (_) {
      return 'Failed to connect to server. Please try again later.';
    }

    return null;
  }
}