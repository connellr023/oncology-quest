import 'package:flutter/material.dart';
import 'package:flutter_svg/svg.dart';

import '../widgets/credit_footer.dart';

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
              width: 130,
              height: 130,
              colorFilter: ColorFilter.mode(Theme.of(context).primaryColor, BlendMode.srcIn),
            ),
            const SizedBox(height: 16),
            RichText(
              text: TextSpan(
                text: 'Get started with ',
                style: TextStyle(
                  fontSize: MediaQuery.of(context).size.width * 0.045,
                  color: Theme.of(context).textTheme.bodySmall!.color,
                ),
                children: const <TextSpan>[
                  TextSpan(text: 'Oncology Quest', style: TextStyle(fontWeight: FontWeight.bold)),
                  TextSpan(text: ' below.'),
                ],
              ),
            ),
            const SizedBox(height: 20),
            ElevatedButton(
              onPressed: () {},
              style: ElevatedButton.styleFrom(
                backgroundColor: Theme.of(context).primaryColor,
                padding: const EdgeInsets.symmetric(horizontal: 40, vertical: 16),
                shape: RoundedRectangleBorder(
                  borderRadius: BorderRadius.circular(30),
                ),
              ),
              child: const Text('Login'),
            ),
          ],
        )
      ),
      bottomNavigationBar: const Footer(),
    );
  }
}