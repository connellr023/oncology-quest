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

  factory ClientUser.deserialize(Map<String, dynamic> json) {
    final id = json['id'];
    final username = json['username'];
    final name = json['name'];
    final isAdmin = json['is_admin'];
    final loginCount = json['login_count'];

    if (id is! int || username is! String || name is! String || isAdmin is! bool || loginCount is! int) {
      throw Error();
    }

    return ClientUser(
      id: id,
      username: username,
      name: name,
      isAdmin: isAdmin,
      loginCount: loginCount,
    );
  }
}