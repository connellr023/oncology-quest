import 'package:flutter/material.dart';
import 'package:oncology_quest_mobile/src/widgets/thematic_elevated_button.dart';

import '../widgets/main_logo.dart';
import '../widgets/monotone_elevated_button.dart';

class LoginView extends StatelessWidget {
  const LoginView({super.key});

  @override
  Widget build(BuildContext context) {
    const double buttonSpacing = 17;

    return Scaffold(
      body: Stack(
        children: <Widget>[
          Positioned(
            top: MediaQuery.of(context).padding.top + 40,
            left: 0,
            right: 0,
            child: Column(
              mainAxisAlignment: MainAxisAlignment.center,
              children: <Widget>[
                MainLogo(imageSize: MediaQuery.of(context).size.width * 0.24),
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
                )
              ]
            )
          ),
          Center(
            child: Column(
              mainAxisAlignment: MainAxisAlignment.center,
              children: <Widget>[
                const SizedBox(height: 30),
                ThematicElevatedButton(
                  text: 'Ok',
                  onPressed: () => {}
                ),
                const SizedBox(height: buttonSpacing),
                MonotoneElevatedButton(
                  text: 'Back',
                  onPressed: () => Navigator.pop(context)
                )
              ],
            ),
          )
        ],
      )
      // body: Center(
      //   child: Column(
      //     mainAxisAlignment: MainAxisAlignment.center,
      //     children: <Widget>[
      //       MainLogo(imageSize: MediaQuery.of(context).size.width * 0.22),
      //       const SizedBox(height: 30),
      //       ThematicElevatedButton(
      //         text: 'Ok',
      //         onPressed: () => {}
      //       ),
      //       const SizedBox(height: buttonSpacing),
      //       MonotoneElevatedButton(
      //         text: 'Back',
      //         onPressed: () => Navigator.pop(context)
      //       )
      //     ],
      //   ),
      // )
    );
  }
}