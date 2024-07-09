import 'package:flutter/material.dart';
import 'package:oncology_quest_mobile/src/state/session_state.dart';
import 'package:oncology_quest_mobile/src/utilities/sizing.dart';
import 'package:provider/provider.dart';

import '../widgets/credit_footer.dart';
import '../widgets/buttons.dart';
import '../widgets/main_logo.dart';

class HomeView extends StatelessWidget {
  const HomeView({super.key});

  @override
  Widget build(BuildContext context) {
    Future.delayed(Duration.zero, () {
      final session = Provider.of<SessionState>(context, listen: false).session;

      if (session != null) {
        Navigator.pushReplacementNamed(context, '/dashboard');
      }
    });

    double buttonWidth = uiWidth(context);
    double buttonHeight = mainUiButtonHeight(context);
    const double buttonSpacing = 25;

    return Scaffold(
      body: Center(
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: <Widget>[
            MainLogo(imageSize: homeViewMainLogoSize(context)),
            const SizedBox(height: 10),
            Padding(
              padding: const EdgeInsets.all(18.0),
              child: RichText(
                textAlign: TextAlign.center,
                text: TextSpan(
                  text: 'Get started with ',
                  style: TextStyle(
                    fontSize: headingFontSize(context),
                    color: Theme.of(context).textTheme.bodySmall!.color,
                  ),
                  children: const <TextSpan>[
                    TextSpan(text: 'Oncology Quest', style: TextStyle(fontWeight: FontWeight.bold)),
                    TextSpan(text: ' below')
                  ]
                )
              )
            ),
            const SizedBox(height: 50),
            ThematicElevatedButton(
              width: buttonWidth,
              height: buttonHeight,
              text: 'Login',
              onPressed: () => Navigator.pushNamed(context, '/login')
            ),
            const SizedBox(height: buttonSpacing),
            ThematicElevatedButton(
              width: buttonWidth,
              height: buttonHeight,
              text: 'Register',
              onPressed: () => {}
            ),
            const SizedBox(height: buttonSpacing),
            ThematicElevatedButton(
              width: buttonWidth,
              height: buttonHeight,
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