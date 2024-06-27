import 'package:flutter/material.dart';
import 'package:flutter_svg/svg.dart';

import '../widgets/credit_footer.dart';
import '../widgets/thematic_elevated_button.dart';

class HomeView extends StatelessWidget {
  const HomeView({super.key});

  @override
  Widget build(BuildContext context) {
    double buttonWidth = MediaQuery.of(context).size.width * 0.6;
    const double buttonSpacing = 15;

    return Scaffold(
      body: Center(
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: <Widget>[
            SvgPicture.asset(
              'assets/images/logo.svg',
              width: 130,
              height: 130,
              colorFilter: ColorFilter.mode(Theme.of(context).primaryColor, BlendMode.srcIn),
            ),
            const SizedBox(height: 16),
            RichText(
              text: TextSpan(
                text: 'Welcome to ',
                style: TextStyle(
                  fontSize: MediaQuery.of(context).size.width * 0.045,
                  color: Theme.of(context).textTheme.bodySmall!.color,
                ),
                children: const <TextSpan>[
                  TextSpan(text: 'Oncology Quest', style: TextStyle(fontWeight: FontWeight.bold))
                ],
              ),
            ),
            const SizedBox(height: 6),
            Text(
              'an aid for Medical Oncology Trainees',
              style: TextStyle(
                fontSize: MediaQuery.of(context).size.width * 0.027,
                color: Theme.of(context).textTheme.bodySmall!.color,
              )
            ),
            const SizedBox(height: 25),
            SizedBox(
              width: buttonWidth,
              height: 45,
              child: ThematicElevatedButton(
                text: 'Login',
                onPressed: () => {},
              ),
            ),
            const SizedBox(height: buttonSpacing),
            SizedBox(
              width: buttonWidth,
              height: 45,
              child: ThematicElevatedButton(
                text: 'Register',
                onPressed: () => {},
              ),
            ),
            const SizedBox(height: buttonSpacing),
            SizedBox(
              width: buttonWidth,
              height: 45,
              child: ThematicElevatedButton(
                text: 'Reset Password',
                onPressed: () => {},
              ),
            )
          ]
        )
      ),
      bottomNavigationBar: const Footer(),
    );
  }
}