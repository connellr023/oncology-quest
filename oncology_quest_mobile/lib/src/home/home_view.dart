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
              color: Theme.of(context).primaryColor,
            ),
            const SizedBox(height: 16),
            // const Text(
            //   'Get started with Oncology Quest below.'
            // ),
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
          ],
        )
      ),
      bottomNavigationBar: const Footer(),
    );
  }
}