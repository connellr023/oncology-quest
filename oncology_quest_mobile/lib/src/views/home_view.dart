import 'package:flutter/material.dart';

import '../widgets/credit_footer.dart';
import '../widgets/thematic_elevated_button.dart';
import '../widgets/main_logo.dart';

class HomeView extends StatelessWidget {
  const HomeView({super.key});

  @override
  Widget build(BuildContext context) {
    const double buttonSpacing = 17;

    return Scaffold(
      body: Center(
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: <Widget>[
            MainLogo(imageSize: MediaQuery.of(context).size.width * 0.35),
            const SizedBox(height: 10),
            Padding(
              padding: const EdgeInsets.all(18.0),
              child: RichText(
                textAlign: TextAlign.center,
                text: TextSpan(
                  text: 'Get started with ',
                  style: TextStyle(
                    fontSize: MediaQuery.of(context).size.width * 0.06,
                    color: Theme.of(context).textTheme.bodySmall!.color,
                  ),
                  children: const <TextSpan>[
                    TextSpan(text: 'Oncology Quest', style: TextStyle(fontWeight: FontWeight.bold)),
                    TextSpan(text: ' below')
                  ],
                ),
              ),
            ),
            const SizedBox(height: 50),
            ThematicElevatedButton(
              text: 'Login',
              onPressed: () => Navigator.pushNamed(context, '/login')
            ),
            const SizedBox(height: buttonSpacing),
            ThematicElevatedButton(
              text: 'Register',
              onPressed: () => {}
            ),
            const SizedBox(height: buttonSpacing),
            ThematicElevatedButton(
              text: 'Reset Password',
              onPressed: () => {}
            )
          ]
        )
      ),
      bottomNavigationBar: const CreditFooter(),
    );
  }
}