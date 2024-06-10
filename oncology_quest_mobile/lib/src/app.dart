import 'package:flutter/material.dart';
import 'package:flutter_gen/gen_l10n/app_localizations.dart';
import 'package:flutter_localizations/flutter_localizations.dart';

import 'home/home_view.dart';

class App extends StatelessWidget {
  const App({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      debugShowCheckedModeBanner: false,
      home: const HomeView(),
      theme: ThemeData(
        primaryColor: const Color(0xFF331BBF),
        scaffoldBackgroundColor: const Color(0xFF030303),
        textTheme: const TextTheme(
          bodySmall: TextStyle(color: Color(0xFFE7E7E7)),
          bodyMedium: TextStyle(color: Color(0xFFE7E7E7)),
          bodyLarge: TextStyle(color: Color(0xFFE7E7E7))
        )
      ),
      restorationScopeId: 'app',
      localizationsDelegates: const [
        AppLocalizations.delegate,
        GlobalMaterialLocalizations.delegate,
        GlobalWidgetsLocalizations.delegate,
        GlobalCupertinoLocalizations.delegate,
      ],
      supportedLocales: const [
        Locale('en', 'CA'),
      ],
      onGenerateTitle: (BuildContext context) => AppLocalizations.of(context)!.appTitle
    );
  }
}
