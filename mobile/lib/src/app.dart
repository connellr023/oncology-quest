import 'package:flutter/material.dart';
import 'package:flutter_localizations/flutter_localizations.dart';
import 'package:google_fonts/google_fonts.dart';
import 'package:oncology_quest_mobile/src/utilities.dart';
import 'package:oncology_quest_mobile/src/views/home_view.dart';
import 'package:oncology_quest_mobile/src/views/login_view.dart';
import 'package:oncology_quest_mobile/src/views/dashboard_view.dart';

class App extends StatelessWidget {
  const App({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      debugShowCheckedModeBanner: false,
      theme: ThemeData(
        dividerColor: Colors.transparent,
        primaryColor: themeColor,
        scaffoldBackgroundColor: backgroundColor1,
        textTheme: GoogleFonts.interTextTheme(
          Theme.of(context).textTheme.copyWith(
            bodySmall: const TextStyle(color: textColor),
            bodyMedium: const TextStyle(color: textColor),
            bodyLarge: const TextStyle(color: textColor)
          )
        )
      ),
      restorationScopeId: 'app',
      localizationsDelegates: const [
        GlobalMaterialLocalizations.delegate,
        GlobalWidgetsLocalizations.delegate,
        GlobalCupertinoLocalizations.delegate
      ],
      supportedLocales: const [
        Locale('en', 'CA')
      ],
      initialRoute: '/',
      routes: {
        '/': (context) => const HomeView(),
        '/login': (context) => const LoginView(),
        '/dashboard': (context) => const DashboardView()
      }
    );
  }
}
