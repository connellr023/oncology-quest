import 'package:flutter/material.dart';
import 'package:oncology_quest_mobile/src/state/session_state.dart';
import 'package:oncology_quest_mobile/src/utilities/colors.dart';
import 'package:oncology_quest_mobile/src/utilities/sizing.dart';
import 'package:oncology_quest_mobile/src/widgets/buttons.dart';
import 'package:provider/provider.dart';

import '../widgets/main_logo.dart';
import '../widgets/form_text_field.dart';
import '../utilities/regex.dart';

class LoginView extends StatefulWidget {
  const LoginView({super.key});

  @override
  State<LoginView> createState() => _LoginViewState();
}

class _LoginViewState extends State<LoginView> {
  String _username = '';
  String _password = '';

  bool _isUsernameValid = false;
  bool _isPasswordValid = false;
  bool _isLoading = false;

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

  void _updateLoading(bool isLoading) {
    setState(() {
      _isLoading = isLoading;
    });
  }

  Future<void> _attemptLogin(String username, String plaintextPassword) async {
    _updateLoading(true);
    String? error = await Provider.of<SessionState>(context, listen: false).login(username, plaintextPassword);
    _updateLoading(false);

    if (error == null && mounted) {
      Navigator.pushNamed(context, '/dashboard');
      return;
    }
    
    _updateLoginError(error);
  }

  @override
  Widget build(BuildContext context) {
    double buttonWidth = uiWidth(context);
    double buttonHeight = secondaryUiButtonHeight(context);
    double backButtonSize = standardFontSize(context);

    bool isMobile = inMobileViewport(context);

    return Scaffold(
      appBar: isMobile ? AppBar(
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
            fontSize: backButtonSize,
          ),
        ),
        centerTitle: false
      ) : null,
      body: Center(
        child: SingleChildScrollView(
          child: Column(
            mainAxisAlignment: MainAxisAlignment.center,
            children: <Widget>[
              MainLogo(imageSize: secondaryViewMainLogoSize(context)),
              const SizedBox(height: 25),
              RichText(
                textAlign: TextAlign.center,
                text: TextSpan(
                  text: 'Login to ',
                  style: TextStyle(
                    fontSize: headingFontSize(context),
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
                onChanged: (String input) => _username = input
              ),
              const SizedBox(height: 18),
              FormTextField(
                obscureText: true,
                labelText: 'Password',
                validationRegex: passwordRegex,
                errorMessage: 'Password must be at withing 8 to 200 characters long',
                onErrorChanged: _updatePasswordError,
                onChanged: (String input) => _password = input
              ),
              const SizedBox(height: 20),
              if (_loginError != null) Text(
                _loginError!,
                style: TextStyle(
                  color: errorColor,
                  fontSize: MediaQuery.of(context).size.width * 0.04
                )
              ),
              const SizedBox(height: 15),
              ThematicElevatedButton(
                width: buttonWidth,
                height: buttonHeight,
                isDisabled: !_isUsernameValid || !_isPasswordValid,
                isLoading: _isLoading,
                text: 'Ok',
                onPressed: () => _attemptLogin(_username, _password)
              ),
              if (!isMobile) ...<Widget>[
                const SizedBox(height: 20),
                MonotoneElevatedButton(
                  width: buttonWidth,
                  height: buttonHeight,
                  text: 'Back',
                  onPressed: () => Navigator.pop(context)
                )
              ]
            ],
          ),
        )
      )
    );
  }
}