import 'package:flutter/material.dart';
import 'package:flutter_dotenv/flutter_dotenv.dart';
import 'package:oncology_quest_mobile/src/state/session_state.dart';
import 'package:provider/provider.dart';

import 'src/app.dart';

Future<void> main() async {
  runApp(
    ChangeNotifierProvider(
      create: (context) => SessionState(),
      child: _Initializer()
    )
  );
}

class _Initializer extends StatelessWidget {
  Future<void> _initApp(BuildContext context) async {
    final sessionState = Provider.of<SessionState>(context, listen: false);

    await dotenv.load(fileName: '.env');
    await sessionState.loadJwt();
    await sessionState.fetchSession();
  }

  @override
  Widget build(BuildContext context) {
    return FutureBuilder(
      future: _initApp(context),
      builder: (context, snapshot) {
        if (snapshot.connectionState == ConnectionState.done) {
          return const App();
        }

        return const MaterialApp(
          home: Scaffold(
            body: Center(
              child: CircularProgressIndicator(),
            ),
          ),
        );
      }
    );
  }
}