
import 'package:oncology_quest_mobile/src/utilities/result.dart';

class Rotation {
  final int id;
  final String name;
  final DateTime lastUpdated;

  Rotation({
    required this.id,
    required this.name,
    required this.lastUpdated,
  });

  static Result<Rotation> deserialize(Map<String, dynamic> json) {
    if (json['id'].runtimeType != int ||
        json['name'].runtimeType != String ||
        json['lastUpdated'].runtimeType != String
    ) return Result.err('Invalid JSON');

    return Result.ok(Rotation(
      id: json['id'],
      name: json['name'],
      lastUpdated: DateTime.parse(json['lastUpdated']),
    ));
  }
}