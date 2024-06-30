import 'package:flutter_dotenv/flutter_dotenv.dart';

final Uri apiEndpoint = Uri.parse(dotenv.env['API_ENDPOINT']!);