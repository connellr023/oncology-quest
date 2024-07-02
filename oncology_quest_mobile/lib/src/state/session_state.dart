import 'package:flutter/material.dart';
import 'package:shared_preferences/shared_preferences.dart';
import 'package:oncology_quest_mobile/src/models/session.dart';
import 'package:oncology_quest_mobile/src/utilities/endpoint.dart';
import 'package:http/http.dart' as http;

import 'dart:convert';
 
class SessionState with ChangeNotifier {
  Session? _session;
  Session? get session => _session;

  String? _jwt;
  String? get jwt => _jwt;

  SessionState() {
    init();
  }

  Future<void> init() async {
    await loadJwt();
    await fetchSession();
  }

  Future<void> fetchSession() async {
    if (_jwt == null) {
      return;
    }

    try {
      final response = await http.get(apiEndpoint.resolve('/api/users/session'), headers: {
        'authorization': _jwt!
      });

      if (response.statusCode == 200) {
        final session = Session.deserialize(json.decode(response.body));
        _session = session;

        notifyListeners();
        return;
      }
    }
    catch (_) {}

    _session = null;

    await storeJwt(null);
    notifyListeners();
  }

  Future<String?> login(String username, String plaintextPassword) async {
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
        catch (e) {
          return e.toString();
          //return json.decode(response.body).toString();
          //return 'Failed to parse server response.';
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