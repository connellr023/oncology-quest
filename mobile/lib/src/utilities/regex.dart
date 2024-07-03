final RegExp usernameRegex = RegExp(r'^[a-zA-Z0-9\-_\.]{1,25}$');
final RegExp nameRegex = RegExp(r'^[a-zA-Z\s]{1,35}$');
final RegExp passwordRegex = RegExp(r'^.{8,200}$');
final RegExp commentRegex = RegExp(r'^[a-zA-Z0-9\s.,!?"()-]{0,150}$');
final RegExp entryTitleRegex = RegExp(r'^[a-zA-Z0-9+\-/()\s]{1,100}$');
final RegExp resetTokenRegex = RegExp(r'^[a-zA-Z0-9]{4}$');