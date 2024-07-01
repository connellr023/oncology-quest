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
    loadJwt();

    // TODO: Ping the server to check if the JWT is still valid and get the session data
  }

  Future<String?> login(String username, String plaintextPassword) async {
    try {
      final response = await http.post(apiEndpoint.resolve('/api/users/login'), body: {
        'username': username,
        'password': plaintextPassword
      });

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
            return 'An unknown error occurred. Please try again later.';
        }
      }
    }
    catch (_) {
      return 'Failed to connect to server. Please try again later.';
    }

    return null;
  }

  void logout() {
    _session = null;
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