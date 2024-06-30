import 'package:oncology_quest_mobile/src/models/client_user.dart';
import 'package:oncology_quest_mobile/src/models/rotation.dart';
import 'package:oncology_quest_mobile/src/utilities/result.dart';

class Session {
  final ClientUser user;
  final Map<int, Rotation> rotations;

  Session({
    required this.user,
    required this.rotations,
  });

  static Result<Session> deserialize(Map<String, dynamic> json) {
    final user = ClientUser.deserialize(json['user']);

    if (user.isErr) {
      return Result.err(user.error);
    }

    final rotations = Map<int, Rotation>.fromEntries(json['rotations'].map((entry) {
      final rotation = Rotation.deserialize(entry['rotation']);

      if (rotation.isErr) {
        return Result.err(rotation.error);
      }

      return MapEntry(rotation.data!.id, rotation);
    }));

    return Result.ok(Session(
      user: user.data!,
      rotations: rotations,
    ));
  }
}