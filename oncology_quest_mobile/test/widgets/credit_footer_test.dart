import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:oncology_quest_mobile/src/widgets/credit_footer.dart';

void main() {
  group('Credit Footer Widget', () {
    testWidgets('should display developer name', (WidgetTester tester) async {
      await tester.pumpWidget(const MaterialApp(home: Scaffold(body: Footer())));
      expect(find.textContaining('Connell Reffo'), findsOneWidget);
    });
  });
}
