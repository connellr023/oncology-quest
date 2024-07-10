import 'package:flutter/material.dart';
import 'package:oncology_quest_mobile/src/models/rotation.dart';
import 'package:shared_preferences/shared_preferences.dart';
import 'package:oncology_quest_mobile/src/models/session.dart';
import 'package:oncology_quest_mobile/src/utilities/endpoint.dart';
import 'package:http/http.dart' as http;

import 'dart:convert';
 
class SessionState extends ChangeNotifier {
  Session? _session;
  Session? get session => _session;

  String? _jwt;
  String? get jwt => _jwt;

  Future<String?> createRotation(String name) async {
    if (_jwt == null || session == null) {
      return 'You must be logged in to create a rotation.';
    }

    try {
      final response = await http.post(apiEndpoint.resolve('/api/rotations/create'),
        headers: {
          'content-type': 'application/json',
          'authorization': _jwt!
        },
        body: jsonEncode({
          'name': name
        })
      );

      if (response.statusCode == 201) {
        final body = json.decode(response.body);

        final int rotationId = int.parse(body['rotationId'].toString());
        final DateTime lastUpdated = DateTime.parse(body['lastUpdated'].toString());

        final rotation = Rotation(
          id: rotationId,
          name: name,
          lastUpdated: lastUpdated
        );

        session!.rotations.putIfAbsent(rotationId, () => rotation);
        notifyListeners();
      }
      else {
        return 'Failed to create rotation. Please try again later.';
      }
    }
    catch (_) {
      return 'Failed to connect to server. Please try again later.';
    }

    return null;
  }

  Future<String?> deleteRotation(int rotationId) async {
    if (_jwt == null || session == null) {
      return 'You must be logged in to delete a rotation.';
    }

    try {
      final response = await http.delete(apiEndpoint.resolve('/api/rotations/delete'),
        headers: {
          'content-type': 'application/json',
          'authorization': _jwt!
        },
        body: jsonEncode({
          'rotationId': rotationId
        })
      );

      if (response.statusCode == 200) {
        session!.rotations.remove(rotationId);
        notifyListeners();
      }
      else {
        return 'Failed to delete rotation. Please try again later.';
      }
    }
    catch (_) {
      return 'Failed to connect to server. Please try again later.';
    }

    return null;
  }

  Future<String?> fetchSession() async {
    if (_jwt == null) {
      return null;
    }

    try {
      final response = await http.get(apiEndpoint.resolve('/api/users/session'), headers: {
        'authorization': _jwt!
      });

      if (response.statusCode == 200) {
        try {
          final session = Session.deserialize(json.decode(response.body));
          _session = session;

          notifyListeners();
          return null;
        }
        catch (_) {
          return 'Failed to parse server response.';
        }
      }
    }
    catch (_) {
      return 'Failed to connect to server. Please try again later.';
    }

    _session = null;

    await storeJwt(null);
    notifyListeners();
    return null;
  }

  Future<String?> login(String username, String plaintextPassword) async {
    // TODO: Clear user tasks and entries memoized data.

    try {
      final response = await http.post(apiEndpoint.resolve('/api/users/login'),
        headers: {
          'content-type': 'application/json'
        },
        body: jsonEncode({
          'username': username,
          'password': plaintextPassword
        })
      );

      if (response.statusCode == 200) {
        final jwt = response.headers['authorization']?.split(' ').last;

        if (jwt == null) {
          return 'Failed to parse JWT from server response.';
        }

        try {
          final session = Session.deserialize(json.decode(response.body));

          _session = session;

          await storeJwt(jwt);
          notifyListeners();
        }
        catch (_) {
          return 'Failed to parse server response.';
        }
      }
      else {
        switch (response.statusCode) {
          case 401:
            return 'Invalid username or password.';
          case 429:
            return 'Too many login attempts. Please try again later.';
          case 500:
            return 'Internal server error. Please try again later.';
          default:
            return 'Received response code ${response.statusCode}. Please try again later.';
        }
      }
    }
    catch (_) {
      return 'Failed to connect to server. Please try again later.';
    }

    return null;
  }

  Future<void> logout() async {
    _session = null;

    await storeJwt(null);
    notifyListeners();
  }

  Future<void> storeJwt(String? jwt) async {
    final SharedPreferences prefs = await SharedPreferences.getInstance();

    if (jwt == null) {
      await prefs.remove('jwt');
    }
    else {
      await prefs.setString('jwt', jwt);
    }

    _jwt = jwt;
  }

  Future<void> loadJwt() async {
    final SharedPreferences prefs = await SharedPreferences.getInstance();
    final jwt = prefs.getString('jwt');

    if (jwt != null) {
      _jwt = jwt;
    }
  }
}