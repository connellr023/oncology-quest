import 'package:flutter/material.dart';
import 'package:oncology_quest_mobile/src/models/client_user.dart';
import 'package:oncology_quest_mobile/src/models/entry_levels.dart';
import 'package:oncology_quest_mobile/src/models/user_task.dart';
import 'package:oncology_quest_mobile/src/utilities.dart';

import 'package:http/http.dart' as http;
import 'dart:convert';

class UserTasksState extends ChangeNotifier {
  ClientUser? _selectedUser;
  ClientUser? get selectedUser => _selectedUser;

  /// Memoized user tasks.
  /// Map of rotation ID to map of subtask ID to user task.
  final Map<int, UserTaskStructure> _userTasksMemo = {};
  Map<int, UserTaskStructure> get userTasks => _userTasksMemo;

  /// Memoized progress.
  /// Map of supertask ID to progress memo.
  final Map<int, _ProgressMemo> _progressMemo = {};

  void invalidateTaskProgressMemo(int supertaskId, int taskId) {
    if (_progressMemo[supertaskId]?.tasksProgress[taskId] != null) {
      _progressMemo[supertaskId]!.supertaskProgress = null;
      _progressMemo[supertaskId]!.tasksProgress.remove(taskId);
    }
  }

  void selectUser(ClientUser? user) {
    _selectedUser = user;
    notifyListeners();
  }

  void clearMemo() {
    _userTasksMemo.clear();
    _progressMemo.clear();
  }

  void clearProgressMemo() {
    _progressMemo.clear();
  }

  double calculateTaskProgress(int rotationId, int supertaskId, int taskId, List<Subtask> subtasks) {
    final taskProgressMemo = _progressMemo[supertaskId]?.tasksProgress[taskId];

    if (taskProgressMemo != null) {
      return taskProgressMemo;
    }

    if (_progressMemo[supertaskId] == null) {
      _progressMemo[supertaskId] = _ProgressMemo();
    }

    if (subtasks.isEmpty) {
      _progressMemo[supertaskId]!.tasksProgress[taskId] = 0.0;
      return 0.0;
    }

    int completedSubtasks = 0;

    for (final subtask in subtasks) {
      final correspondingUserTask = _userTasksMemo[rotationId]?.structure[subtask.id];

      if (correspondingUserTask != null && correspondingUserTask.isCompleted) {
        completedSubtasks++;
      }
    }

    final progress = completedSubtasks / subtasks.length;
    _progressMemo[supertaskId]!.tasksProgress[taskId] = progress;

    return progress;
  }

  double calculateSupertaskProgress(int rotationId, EntryHierarchy supertaskLevel) {
    final supertaskProgressMemo = _progressMemo[supertaskLevel.hierarchy.entry.id]?.supertaskProgress;

    if (supertaskProgressMemo != null) {
      return supertaskProgressMemo;
    }

    if (_progressMemo[supertaskLevel.hierarchy.entry.id] == null) {
      _progressMemo[supertaskLevel.hierarchy.entry.id] = _ProgressMemo();
    }
    
    if (supertaskLevel.hierarchy.children.isEmpty) {
      _progressMemo[supertaskLevel.hierarchy.entry.id]!.supertaskProgress = 0.0;
      return 0.0;
    }

    int totalTasks = 0;
    double totalProgress = 0.0;

    for (final taskLevel in supertaskLevel.hierarchy.children) {
      final progress = calculateTaskProgress(rotationId, supertaskLevel.hierarchy.entry.id, taskLevel.entry.id, taskLevel.children);

      totalProgress += progress;
      totalTasks++;
    }

    final progress = totalProgress / totalTasks;
    _progressMemo[supertaskLevel.hierarchy.entry.id]!.supertaskProgress = progress;

    return progress;
  }

  Future<String?> fetchOwnUserTasks(String jwt, int rotationId) async {
    if (_userTasksMemo[rotationId] != null) {
      return null;
    }

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
      }
      else {
        return 'Failed to fetch user tasks. Please try again later.';
      }
    }
    catch (_) {
      return 'Failed to connect to server. Please try again later.';
    }

    notifyListeners();
    return null;
  }

  Future<String?> fetchUserTasks(String jwt, int rotationId, ClientUser user) async {
    // if (_userTasksMemo[rotationId] != null && _selectedUser == user) {
    //   return null;
    // }

    try {
      final response = await http.get(apiEndpoint.resolve('/api/tasks/${user.id}/$rotationId'),
        headers: {
          'content-type': 'application/json',
          'authorization': jwt
        }
      );

      if (response.statusCode == 200) {
        final body = json.decode(response.body);
        final userTasks = UserTaskStructure.deserialize(body);

        _userTasksMemo[rotationId] = userTasks;
        _selectedUser = user;
      }
      else {
        return 'Failed to fetch user tasks. Please try again later.';
      }
    }
    catch (_) {
      return 'Failed to connect to server. Please try again later.';
    }

    notifyListeners();
    return null;
  }

  Future<String?> updateUserTask(String jwt, int rotationId, int supertaskId, int taskId, int subtaskId, int userId, bool isCompleted, String comment) async {
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
        }
        else {
          return 'Failed to update user task. Please try again later.';
        }
      }
    }
    catch (_) {
      return 'Failed to connect to server. Please try again later.';
    }

    invalidateTaskProgressMemo(supertaskId, taskId);
    notifyListeners();

    return null;
  }
}

class _ProgressMemo {
  /// Map of task ID to progress.
  final Map<int, double> tasksProgress;
  double? supertaskProgress;

  _ProgressMemo() : tasksProgress = {};
}