import 'package:flutter/material.dart';
import 'package:oncology_quest_mobile/src/models/session.dart';
import 'package:oncology_quest_mobile/src/utilities/endpoint.dart';
import 'package:oncology_quest_mobile/src/widgets/thematic_elevated_button.dart';
import 'package:http/http.dart' as http;

import 'dart:convert';

import '../widgets/main_logo.dart';
import '../widgets/form_text_field.dart';
import '../utilities/regex.dart';

class LoginView extends StatefulWidget {
  const LoginView({super.key});

  @override
  State<LoginView> createState() => _LoginViewState();
}

class _LoginViewState extends State<LoginView> {
  bool _isUsernameValid = false;
  bool _isPasswordValid = false;

  String? _loginError;

  void _updateUsernameError(bool isError) {
    setState(() {
      _isUsernameValid = !isError;
    });
  }

  void _updatePasswordError(bool isError) {
    setState(() {
      _isPasswordValid = !isError;
    });
  }

  void _updateLoginError(String? error) {
    setState(() {
      _loginError = error;
    });
  }

  Future<void> _login(String username, String plaintextPassword) async {
    try {
      _updateLoginError(null);

      final response = await http.post(apiEndpoint.resolve('/api/users/login'), body: {
        'username': username,
        'password': plaintextPassword
      });

      if (response.statusCode == 200) {
        final session = Session.deserialize(json.decode(response.body));

        if (session.isErr) {
          _updateLoginError('Failed to deserialize session response.');
          return;
        }
      }
      else {
        switch (response.statusCode) {
          case 401:
            _updateLoginError('Invalid username or password');
            break;
          case 429:
            _updateLoginError('Too many login attempts. Please try again later.');
            break;
          case 500:
            _updateLoginError('Internal server error. Please try again later.');
            break;
          default:
            _updateLoginError('An unknown error occurred. Please try again later.');
            break;
        }
      }
    }
    catch (_) {
      _updateLoginError('Failed to connect to server. Please try again later.');
    }
  }

  @override
  Widget build(BuildContext context) {
    double backButtonSize = MediaQuery.of(context).size.width * 0.08;
    double backButtonFontSize = backButtonSize * 0.7;

    return Scaffold(
      appBar: AppBar(
        backgroundColor: Theme.of(context).scaffoldBackgroundColor,
        leading: IconButton(
          icon: const Icon(Icons.arrow_back),
          onPressed: () => Navigator.pop(context),
          color: Theme.of(context).textTheme.bodySmall!.color,
          iconSize: backButtonSize
        ),
        title: Text(
          'Back',
          style: TextStyle(
            color: Theme.of(context).textTheme.bodySmall!.color,
            fontSize: backButtonFontSize,
          ),
        ),
        centerTitle: false
      ),
      body: Center(
        child: SingleChildScrollView(
          child: Column(
            mainAxisAlignment: MainAxisAlignment.center,
            children: <Widget>[
              MainLogo(imageSize: MediaQuery.of(context).size.width * 0.22),
              const SizedBox(height: 25),
              RichText(
                textAlign: TextAlign.center,
                text: TextSpan(
                  text: 'Login to ',
                  style: TextStyle(
                    fontSize: MediaQuery.of(context).size.width * 0.06,
                    color: Theme.of(context).textTheme.bodySmall!.color,
                  ),
                  children: const <TextSpan>[
                    TextSpan(text: 'Oncology Quest', style: TextStyle(fontWeight: FontWeight.bold))
                  ],
                ),
              ),
              const SizedBox(height: 50),
              FormTextField(
                obscureText: false,
                labelText: 'Username',
                validationRegex: usernameRegex,
                errorMessage: 'Invalid username',
                onErrorChanged: _updateUsernameError,
              ),
              const SizedBox(height: 12),
              FormTextField(
                obscureText: true,
                labelText: 'Password',
                validationRegex: passwordRegex,
                errorMessage: 'Password must be at withing 8 to 200 characters long',
                onErrorChanged: _updatePasswordError,
              ),
              const SizedBox(height: 25),
              ThematicElevatedButton(
                isDisabled: !_isUsernameValid || !_isPasswordValid,
                text: 'Ok',
                onPressed: () => {}
              ),
              if (_loginError != null) Text(
                _loginError!,
                style: TextStyle(
                  color: Theme.of(context).textTheme.bodySmall!.color,
                  fontSize: MediaQuery.of(context).size.width * 0.04,
                )
              )
            ],
          ),
        )
      )
    );
  }
}