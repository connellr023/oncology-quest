import 'package:flutter/material.dart';
import 'package:oncology_quest_mobile/src/models/user_task.dart';
import 'package:oncology_quest_mobile/src/utilities/endpoint.dart';

import 'package:http/http.dart' as http;
import 'dart:convert';

class UserTasksState extends ChangeNotifier {
  /// Memoized user tasks.
  /// Map of rotation ID to map of subtask ID to user task.
  final Map<int, UserTaskStructure> _userTasksMemo = {};
  Map<int, UserTaskStructure> get userTasks => _userTasksMemo;

  Future<String?> fetchOwnUserTasks(String jwt, int rotationId) async {
    try {
      final response = await http.get(apiEndpoint.resolve('/api/tasks/$rotationId'),
        headers: {
          'content-type': 'application/json',
          'authorization': jwt
        }
      );

      if (response.statusCode == 200) {
        final body = json.decode(response.body);
        final userTasks = UserTaskStructure.deserialize(body);

        _userTasksMemo[rotationId] = userTasks;
        notifyListeners();
      }
      else {
        return 'Failed to fetch user tasks. Please try again later.';
      }
    }
    catch (_) {
      return 'Failed to connect to server. Please try again later.';
    }

    return null;
  }

  Future<String?> updateUserTask(String jwt, int rotationId, int subtaskId, int userId, bool isCompleted, String comment) async {
    try {
      if (_userTasksMemo[rotationId]?.structure[subtaskId] == null) {
        final response = await http.post(apiEndpoint.resolve('/api/tasks/create'),
          headers: {
            'content-type': 'application/json',
            'authorization': jwt
          },
          body: json.encode({
            'rotationId': rotationId,
            'subtaskId': subtaskId,
            'isCompleted': isCompleted,
            'comment': comment
          })
        );

        if (response.statusCode == 201) {
          final body = json.decode(response.body);
          final userTaskId = int.parse(body['id'].toString());

          _userTasksMemo[rotationId]!.structure[subtaskId] = UserTask(
            id: userTaskId,
            userId: userId,
            subtaskId: subtaskId,
            isCompleted: isCompleted,
            comment: comment
          );

          notifyListeners();
        }
        else {
          return 'Failed to create user task. Please try again later.';
        }
      }
      else {
        final response = await http.patch(apiEndpoint.resolve('/api/tasks/update'),
          headers: {
            'content-type': 'application/json',
            'authorization': jwt
          },
          body: json.encode({
            'id': _userTasksMemo[rotationId]!.structure[subtaskId]!.id,
            'isCompleted': isCompleted,
            'comment': comment
          })
        );

        if (response.statusCode == 200) {
          _userTasksMemo[rotationId]!.structure[subtaskId]!.isCompleted = isCompleted;
          _userTasksMemo[rotationId]!.structure[subtaskId]!.comment = comment;

          notifyListeners();
        }
        else {
          return 'Failed to update user task. Please try again later.';
        }
      }
    }
    catch (_) {
      return 'Failed to connect to server. Please try again later.';
    }

    return null;
  }
}