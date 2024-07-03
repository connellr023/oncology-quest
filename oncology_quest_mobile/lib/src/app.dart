import 'package:flutter/material.dart';
import 'package:flutter_localizations/flutter_localizations.dart';
import 'package:google_fonts/google_fonts.dart';
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
        primaryColor: const Color(0xFF331BBF),
        scaffoldBackgroundColor: const Color(0xFF030303),
        textTheme: GoogleFonts.interTextTheme(
          Theme.of(context).textTheme.copyWith(
            bodySmall: const TextStyle(color: Color(0xFFE7E7E7)),
            bodyMedium: const TextStyle(color: Color(0xFFE7E7E7)),
            bodyLarge: const TextStyle(color: Color(0xFFE7E7E7)) 
          )
        )
      ),
      restorationScopeId: 'app',
      localizationsDelegates: const [
        GlobalMaterialLocalizations.delegate,
        GlobalWidgetsLocalizations.delegate,
        GlobalCupertinoLocalizations.delegate,
      ],
      supportedLocales: const [
        Locale('en', 'CA'),
      ],
      initialRoute: '/',
      routes: {
        '/': (context) => const HomeView(),
        '/login': (context) => const LoginView(),
        '/dashboard': (context) => const DashboardView(),
      }
    );
  }
}