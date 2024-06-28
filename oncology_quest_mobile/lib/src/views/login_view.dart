import 'package:flutter/material.dart';
import 'package:oncology_quest_mobile/src/widgets/thematic_elevated_button.dart';

import '../widgets/main_logo.dart';
import '../widgets/form_text_field.dart';
import '../utilities/regex.dart';

class LoginView extends StatelessWidget {
  const LoginView({super.key});

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
                errorMessage: 'Invalid username'
              ),
              const SizedBox(height: 12),
              FormTextField(
                obscureText: true,
                labelText: 'Password',
                validationRegex: passwordRegex,
                errorMessage: 'Invalid password'
              ),
              const SizedBox(height: 25),
              ThematicElevatedButton(
                text: 'Ok',
                onPressed: () => {}
              )
            ],
          ),
        )
      )
    );
  }
}