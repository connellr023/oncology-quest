class Rotation {
  final int id;
  final String name;
  final DateTime lastUpdated;

  Rotation({
    required this.id,
    required this.name,
    required this.lastUpdated,
  });

  factory Rotation.deserialize(Map<String, dynamic> json) {
    final id = json['id'];
    final name = json['name'];
    final lastUpdated = json['lastUpdated'];

    if (id is! int || name is! String || lastUpdated is! String) {
      throw Error();
    }

    return Rotation(
      id: id,
      name: name,
      lastUpdated: DateTime.parse(lastUpdated),
    );
  }
}