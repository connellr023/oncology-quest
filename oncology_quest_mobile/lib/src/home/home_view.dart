import 'package:flutter/material.dart';
import 'package:flutter_svg/svg.dart';

class HomeView extends StatelessWidget {
  const HomeView({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: Center(
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: <Widget>[
            SvgPicture.asset(
              'assets/images/logo.svg',
              width: 100,
              height: 100,
              color: Theme.of(context).primaryColor,

            ),
            const SizedBox(height: 16),
            const Text(
              'Get started with Oncology Quest below.'
            ),
          ],
        )
      ),
    );
  }
}