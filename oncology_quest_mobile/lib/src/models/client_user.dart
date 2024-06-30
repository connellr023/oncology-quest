import 'package:oncology_quest_mobile/src/utilities/result.dart';

class ClientUser {
  final int id;
  final String username;
  final String name;
  final bool isAdmin;
  final int loginCount;

  ClientUser({
    required this.id,
    required this.username,
    required this.name,
    required this.isAdmin,
    required this.loginCount,
  });

  static Result<ClientUser> deserialize(Map<String, dynamic> json) {
    if (json['id'].runtimeType != int ||
        json['username'].runtimeType != String ||
        json['name'].runtimeType != String ||
        json['isAdmin'].runtimeType != bool ||
        json['loginCount'].runtimeType != int
    ) return Result.err('Invalid JSON');

    return Result.ok(ClientUser(
      id: json['id'],
      username: json['username'],
      name: json['name'],
      isAdmin: json['isAdmin'],
      loginCount: json['loginCount'],
    ));
  }
}