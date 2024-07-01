import 'package:oncology_quest_mobile/src/models/client_user.dart';
import 'package:oncology_quest_mobile/src/models/rotation.dart';

class Session {
  final ClientUser user;
  final Map<int, Rotation> rotations;

  Session({
    required this.user,
    required this.rotations,
  });

  factory Session.deserialize(Map<String, dynamic> json) {
    final user = ClientUser.deserialize(json['user']);
    final rotations = <int, Rotation>{};

    for (final entry in json['rotations'].entries) {
      rotations[int.parse(entry.key)] = Rotation.deserialize(entry.value);
    }

    return Session(
      user: user,
      rotations: rotations,
    );
  }
}