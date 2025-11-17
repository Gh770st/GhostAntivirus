import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:provider/provider.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:go_router/go_router.dart';

import 'core/theme/app_theme.dart';
import 'core/services/notification_service.dart';
import 'core/services/security_service.dart';
import 'core/services/vpn_service.dart';
import 'presentation/providers/app_state_provider.dart';
import 'presentation/providers/scan_provider.dart';
import 'presentation/providers/vpn_provider.dart';
import 'presentation/providers/security_provider.dart';
import 'presentation/screens/splash_screen.dart';
import 'presentation/router/app_router.dart';

void main() async {
  WidgetsFlutterBinding.ensureInitialized();
  
  // Initialize services
  await _initializeServices();
  
  // Set preferred orientations
  await SystemChrome.setPreferredOrientations([
    DeviceOrientation.portraitUp,
    DeviceOrientation.portraitDown,
  ]);
  
  // Set system UI overlay style
  SystemChrome.setSystemUIOverlayStyle(
    const SystemUiOverlayStyle(
      statusBarColor: Colors.transparent,
      statusBarIconBrightness: Brightness.light,
      systemNavigationBarColor: Color(0xFF1a1a1a),
      systemNavigationBarIconBrightness: Brightness.light,
    ),
  );

  runApp(
    const ProviderScope(
      child: GhostAntivirusApp(),
    ),
  );
}

Future<void> _initializeServices() async {
  try {
    // Initialize notification service
    await NotificationService.initialize();
    
    // Initialize security service
    await SecurityService.initialize();
    
    // Initialize VPN service
    await VPNService.initialize();
    
    debugPrint('All services initialized successfully');
  } catch (e) {
    debugPrint('Failed to initialize services: $e');
  }
}

class GhostAntivirusApp extends ConsumerStatefulWidget {
  const GhostAntivirusApp({super.key});

  @override
  ConsumerState<GhostAntivirusApp> createState() => _GhostAntivirusAppState();
}

class _GhostAntivirusAppState extends ConsumerState<GhostAntivirusApp> {
  late GoRouter _router;

  @override
  void initState() {
    super.initState();
    _initializeRouter();
    _checkFirstLaunch();
  }

  void _initializeRouter() {
    _router = AppRouter.router;
  }

  Future<void> _checkFirstLaunch() async {
    final appState = ref.read(appStateProvider.notifier);
    await appState.checkFirstLaunch();
  }

  @override
  Widget build(BuildContext context) {
    final appState = ref.watch(appStateProvider);
    
    return MaterialApp.router(
      title: 'GhostAntivirus',
      debugShowCheckedModeBanner: false,
      theme: AppTheme.darkTheme,
      darkTheme: AppTheme.darkTheme,
      themeMode: ThemeMode.dark,
      
      // Show splash screen or main app
      routeInformationProvider: _router.routeInformationProvider,
      routeInformationParser: _router.routeInformationParser,
      routerDelegate: _router.routerDelegate,
      
      builder: (context, child) {
        // Show splash screen on first launch
        if (appState.isFirstLaunch) {
          return const SplashScreen();
        }
        
        // Show main app
        return child!;
      },
    );
  }
}