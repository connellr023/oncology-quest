import 'package:flutter/material.dart';
import 'package:flutter_dotenv/flutter_dotenv.dart';
import 'package:oncology_quest_mobile/src/state/session_state.dart';
import 'package:provider/provider.dart';

import 'src/app.dart';

Future<void> main() async {
  await dotenv.load(fileName: '.env');

  runApp(
    ChangeNotifierProvider(
      create: (context) => SessionState(),
      child: const App(),
    )
  );
}