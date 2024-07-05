import 'package:flutter/material.dart';

class EntryLevel<T, U> {
  final T entry;
  final List<U> children;

  EntryLevel({
    required this.entry,
    required this.children
  });
}

class EntryHierarchy {
  final EntryLevel<Supertask, EntryLevel<Task, Subtask>> hierarchy;

  EntryHierarchy({required this.hierarchy});

  factory EntryHierarchy.deserialize(Map<String, dynamic> json) {
    return EntryHierarchy(
      hierarchy: EntryLevel<Supertask, EntryLevel<Task, Subtask>>(
        entry: Supertask.deserialize(json['entry']),
        children: (json['children'] as List<dynamic>).map((childJson) => EntryLevel<Task, Subtask>(
          entry: Task.deserialize(childJson['entry']),
          children: (childJson['children'] as List<dynamic>).map((subchildJson) => Subtask.deserialize(subchildJson)).toList(),
        )).toList(),
      )
    );
  }
}

class Supertask {
  final int id;
  final String title;
  final int rotationId;

  Supertask({
    required this.id,
    required this.title,
    required this.rotationId,
  });

  factory Supertask.deserialize(Map<String, dynamic> json) {
    final id = int.tryParse(json['id'].toString());
    final title = json['title'].toString();
    final rotationId = int.tryParse(json['rotationId'].toString());

    if (id == null) {
      throw ErrorDescription('Failed to parse supertask ID.');
    }

    if (title.isEmpty) {
      throw ErrorDescription('Failed to parse supertask title.');
    }

    if (rotationId == null) {
      throw ErrorDescription('Failed to parse rotation ID.');
    }

    return Supertask(
      id: id,
      title: title,
      rotationId: rotationId,
    );
  }
}

class Task {
  final int id;
  final int supertaskId;
  final String title;
  final int rotationId;

  Task({
    required this.id,
    required this.supertaskId,
    required this.title,
    required this.rotationId,
  });

  factory Task.deserialize(Map<String, dynamic> json) {
    final id = int.tryParse(json['id'].toString());
    final supertaskId = int.tryParse(json['supertaskId'].toString());
    final title = json['title'].toString();
    final rotationId = int.tryParse(json['rotationId'].toString());

    if (id == null || supertaskId == null) {
      throw ErrorDescription('Failed to parse task ID or supertask ID.');
    }

    if (title.isEmpty) {
      throw ErrorDescription('Failed to parse task title.');
    }

    if (rotationId == null) {
      throw ErrorDescription('Failed to parse rotation ID.');
    }

    return Task(
      id: id,
      supertaskId: supertaskId,
      title: title,
      rotationId: rotationId,
    );
  }
}

class Subtask {
  final int id;
  final int taskId;
  final String title;
  final int rotationId;

  Subtask({
    required this.id,
    required this.taskId,
    required this.title,
    required this.rotationId,
  });

  factory Subtask.deserialize(Map<String, dynamic> json) {
    final id = int.tryParse(json['id'].toString());
    final taskId = int.tryParse(json['taskId'].toString());
    final title = json['title'].toString();
    final rotationId = int.tryParse(json['rotationId'].toString());

    if (id == null || taskId == null) {
      throw ErrorDescription('Failed to parse subtask ID or task ID.');
    }

    if (title.isEmpty) {
      throw ErrorDescription('Failed to parse subtask title.');
    }

    if (rotationId == null) {
      throw ErrorDescription('Failed to parse rotation ID.');
    }

    return Subtask(
      id: id,
      taskId: taskId,
      title: title,
      rotationId: rotationId,
    );
  }
}