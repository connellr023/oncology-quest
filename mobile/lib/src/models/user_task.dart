import 'package:flutter/material.dart';

class UserTaskStructure {
  /// User task structure.
  /// Map of subtasks ID to user task.
  final Map<int, UserTask> structure;

  UserTaskStructure({required this.structure});

  factory UserTaskStructure.deserialize(Map<String, dynamic> json) {
    final structure = <int, UserTask>{};

    for (final entry in json.entries) {
      final key = int.tryParse(entry.key);
      final value = UserTask.deserialize(entry.value);

      if (key == null) {
        throw ErrorDescription('Failed to parse user task subtask ID.');
      }

      structure[key] = value;
    }

    return UserTaskStructure(structure: structure);
  }
}

class UserTask {
  final int id;
  final int userId;
  final int subtaskId;
  bool isCompleted;
  String comment;

  UserTask({
    required this.id,
    required this.userId,
    required this.subtaskId,
    required this.isCompleted,
    required this.comment,
  });

  factory UserTask.deserialize(Map<String, dynamic> json) {
    final id = int.tryParse(json['id'].toString());
    final userId = int.tryParse(json['userId'].toString());
    final subtaskId = int.tryParse(json['subtaskId'].toString());
    final isCompleted =  bool.tryParse(json['isCompleted'].toString());
    final comment = json['comment'].toString();

    if (id == null) {
      throw ErrorDescription('Failed to parse user task ID.');
    }

    if (userId == null) {
      throw ErrorDescription('Failed to parse user ID.');
    }

    if (subtaskId == null) {
      throw ErrorDescription('Failed to parse subtask ID.');
    }

    if (isCompleted == null) {
      throw ErrorDescription('Failed to parse completion status.');
    }

    return UserTask(
      id: id,
      userId: userId,
      subtaskId: subtaskId,
      isCompleted: isCompleted,
      comment: comment,
    );
  }
}