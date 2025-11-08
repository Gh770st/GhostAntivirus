import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:fl_chart/fl_chart.dart';

import '../../core/theme/app_theme.dart';
import '../../core/constants/app_constants.dart';
import '../../data/models/security_stats.dart';
import '../providers/app_state_provider.dart';
import '../providers/scan_provider.dart';
import '../providers/security_provider.dart';
import '../widgets/common/custom_card.dart';
import '../widgets/common/stats_card.dart';
import '../widgets/dashboard/security_status_card.dart';
import '../widgets/dashboard/recent_threats_card.dart';
import '../widgets/dashboard/quick_actions_card.dart';
import '../widgets/common/circular_progress_indicator.dart';

class DashboardScreen extends ConsumerStatefulWidget {
  const DashboardScreen({super.key});

  @override
  ConsumerState<DashboardScreen> createState() => _DashboardScreenState();
}

class _DashboardScreenState extends ConsumerState<DashboardScreen>
    with TickerProviderStateMixin {
  late AnimationController _fadeController;
  late AnimationController _slideController;
  late Animation<double> _fadeAnimation;
  late Animation<Offset> _slideAnimation;

  @override
  void initState() {
    super.initState();
    _initializeAnimations();
    _loadData();
  }

  void _initializeAnimations() {
    _fadeController = AnimationController(
      duration: AppConstants.defaultAnimationDuration,
      vsync: this,
    );
    
    _slideController = AnimationController(
      duration: AppConstants.defaultAnimationDuration,
      vsync: this,
    );

    _fadeAnimation = Tween<double>(begin: 0.0, end: 1.0).animate(
      CurvedAnimation(parent: _fadeController, curve: Curves.easeIn),
    );

    _slideAnimation = Tween<Offset>(
      begin: const Offset(0, 0.3),
      end: Offset.zero,
    ).animate(
      CurvedAnimation(parent: _slideController, curve: Curves.easeOutCubic),
    );

    _fadeController.forward();
    _slideController.forward();
  }

  Future<void> _loadData() async {
    final scanProvider = ref.read(scanStateProvider.notifier);
    final securityProvider = ref.read(securityStateProvider.notifier);
    
    await Future.wait([
      scanProvider.refreshStats(),
      securityProvider.refreshSecurityStatus(),
    ]);
  }

  @override
  void dispose() {
    _fadeController.dispose();
    _slideController.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final appState = ref.watch(appStateProvider);
    final scanState = ref.watch(scanStateProvider);
    final securityState = ref.watch(securityStateProvider);

    return Scaffold(
      body: SafeArea(
        child: RefreshIndicator(
          onRefresh: _loadData,
          color: AppTheme.primaryColor,
          child: SingleChildScrollView(
            physics: const AlwaysScrollableScrollPhysics(),
            padding: const EdgeInsets.all(16),
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                // Header
                _buildHeader(appState),
                const SizedBox(height: 24),

                // Security Status
                FadeTransition(
                  opacity: _fadeAnimation,
                  child: SlideTransition(
                    position: _slideAnimation,
                    child: SecurityStatusCard(
                      isProtected: securityState.isProtected,
                      lastScan: securityState.lastScan,
                      threatsBlocked: securityState.threatsBlocked,
                    ),
                  ),
                ),
                const SizedBox(height: 16),

                // Stats Grid
                FadeTransition(
                  opacity: _fadeAnimation,
                  child: SlideTransition(
                    position: _slideAnimation,
                    child: _buildStatsGrid(scanState),
                  ),
                ),
                const SizedBox(height: 16),

                // Chart and Recent Threats
                Row(
                  children: [
                    // Threat Chart
                    Expanded(
                      flex: 1,
                      child: FadeTransition(
                        opacity: _fadeAnimation,
                        child: SlideTransition(
                          position: _slideAnimation,
                          child: _buildThreatChart(scanState),
                        ),
                      ),
                    ),
                    const SizedBox(width: 16),
                    // Recent Threats
                    Expanded(
                      flex: 1,
                      child: FadeTransition(
                        opacity: _fadeAnimation,
                        child: SlideTransition(
                          position: _slideAnimation,
                          child: RecentThreatsCard(
                            threats: scanState.recentThreats,
                          ),
                        ),
                      ),
                    ),
                  ],
                ),
                const SizedBox(height: 16),

                // Quick Actions
                FadeTransition(
                  opacity: _fadeAnimation,
                  child: SlideTransition(
                    position: _slideAnimation,
                    child: const QuickActionsCard(),
                  ),
                ),
              ],
            ),
          ),
        ),
      ),
    );
  }

  Widget _buildHeader(AppState appState) {
    return Row(
      mainAxisAlignment: MainAxisAlignment.spaceBetween,
      children: [
        Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Text(
              'Hello, ${appState.userName ?? 'User'}!',
              style: AppTextStyles.headline3.copyWith(
                color: AppTheme.textPrimary,
              ),
            ),
            const SizedBox(height: 4),
            Text(
              'Your device is protected',
              style: AppTextStyles.bodyText2.copyWith(
                color: AppTheme.textSecondary,
              ),
            ),
          ],
        ),
        // Notification Bell
        IconButton(
          onPressed: () => _showNotifications(),
          icon: const Icon(
            Icons.notifications_outlined,
            color: AppTheme.textPrimary,
            size: 28,
          ),
        ),
      ],
    );
  }

  Widget _buildStatsGrid(ScanState scanState) {
    return GridView.count(
      shrinkWrap: true,
      physics: const NeverScrollableScrollPhysics(),
      crossAxisCount: 2,
      mainAxisSpacing: 12,
      crossAxisSpacing: 12,
      childAspectRatio: 1.4,
      children: [
        StatsCard(
          title: 'Files Scanned',
          value: scanState.stats.filesScanned.toString(),
          icon: Icons.folder_outlined,
          color: AppTheme.primaryColor,
          onTap: () => _navigateToScan(),
        ),
        StatsCard(
          title: 'Threats Found',
          value: scanState.stats.threatsFound.toString(),
          icon: Icons.warning_amber_outlined,
          color: AppTheme.warningColor,
          onTap: () => _navigateToThreats(),
        ),
        StatsCard(
          title: 'Apps Scanned',
          value: scanState.stats.appsScanned.toString(),
          icon: Icons.apps_outlined,
          color: AppTheme.successColor,
          onTap: () => _navigateToApps(),
        ),
        StatsCard(
          title: 'VPN Status',
          value: scanState.stats.isVpnConnected ? 'On' : 'Off',
          icon: Icons.vpn_lock_outlined,
          color: scanState.stats.isVpnConnected 
              ? AppTheme.successColor 
              : AppTheme.textSecondary,
          onTap: () => _navigateToVpn(),
        ),
      ],
    );
  }

  Widget _buildThreatChart(ScanState scanState) {
    return CustomCard(
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Text(
            'Threat Activity',
            style: AppTextStyles.subtitle1.copyWith(
              color: AppTheme.textPrimary,
            ),
          ),
          const SizedBox(height: 16),
          SizedBox(
            height: 200,
            child: LineChart(
              _createThreatChartData(scanState.weeklyThreats),
            ),
          ),
        ],
      ),
    );
  }

  LineChartData _createThreatChartData(List<int> weeklyThreats) {
    return LineChartData(
      gridData: FlGridData(
        show: true,
        drawVerticalLine: false,
        horizontalInterval: 1,
        getDrawingHorizontalLine: (value) {
          return FlLine(
            color: AppTheme.dividerColor,
            strokeWidth: 1,
          );
        },
      ),
      titlesData: FlTitlesData(
        show: true,
        rightTitles: AxisTitles(sideTitles: SideTitles(showTitles: false)),
        topTitles: AxisTitles(sideTitles: SideTitles(showTitles: false)),
        bottomTitles: AxisTitles(
          sideTitles: SideTitles(
            showTitles: true,
            interval: 1,
            getTitlesWidget: (value, meta) {
              const days = ['Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat', 'Sun'];
              final dayIndex = value.toInt() % 7;
              return SideTitleWidget(
                axisSide: meta.axisSide,
                child: Text(
                  days[dayIndex],
                  style: AppTextStyles.caption.copyWith(
                    color: AppTheme.textSecondary,
                  ),
                ),
              );
            },
          ),
        ),
        leftTitles: AxisTitles(
          sideTitles: SideTitles(
            showTitles: true,
            interval: 1,
            getTitlesWidget: (value, meta) {
              return SideTitleWidget(
                axisSide: meta.axisSide,
                child: Text(
                  value.toInt().toString(),
                  style: AppTextStyles.caption.copyWith(
                    color: AppTheme.textSecondary,
                  ),
                ),
              );
            },
          ),
        ),
      ),
      borderData: FlBorderData(show: false),
      lineBarsData: [
        LineChartBarData(
          spots: weeklyThreats
              .asMap()
              .entries
              .map((e) => FlSpot(e.key.toDouble(), e.value.toDouble()))
              .toList(),
          isCurved: true,
          gradient: LinearGradient(
            colors: [
              AppTheme.primaryColor.withOpacity(0.8),
              AppTheme.primaryColor.withOpacity(0.2),
            ],
          ),
          barWidth: 3,
          isStrokeCapRound: true,
          dotData: FlDotData(
            show: true,
            getDotPainter: (spot, percent, barData, index) {
              return FlDotCirclePainter(
                radius: 4,
                color: AppTheme.primaryColor,
                strokeWidth: 2,
                strokeColor: AppTheme.backgroundColor,
              );
            },
          ),
          belowBarData: BarAreaData(
            show: true,
            gradient: LinearGradient(
              colors: [
                AppTheme.primaryColor.withOpacity(0.3),
                AppTheme.primaryColor.withOpacity(0.0),
              ],
              begin: Alignment.topCenter,
              end: Alignment.bottomCenter,
            ),
          ),
        ),
      ],
      minX: 0,
      maxX: 6,
      minY: 0,
      maxY: weeklyThreats.isEmpty ? 5 : (weeklyThreats.reduce((a, b) => a > b ? a : b) + 1).toDouble(),
    );
  }

  void _showNotifications() {
    // TODO: Navigate to notifications screen
    ScaffoldMessenger.of(context).showSnackBar(
      const SnackBar(content: Text('Notifications coming soon!')),
    );
  }

  void _navigateToScan() {
    // TODO: Navigate to scan screen
    ScaffoldMessenger.of(context).showSnackBar(
      const SnackBar(content: Text('Navigate to scan')),
    );
  }

  void _navigateToThreats() {
    // TODO: Navigate to threats screen
    ScaffoldMessenger.of(context).showSnackBar(
      const SnackBar(content: Text('Navigate to threats')),
    );
  }

  void _navigateToApps() {
    // TODO: Navigate to apps screen
    ScaffoldMessenger.of(context).showSnackBar(
      const SnackBar(content: Text('Navigate to apps')),
    );
  }

  void _navigateToVpn() {
    // TODO: Navigate to VPN screen
    ScaffoldMessenger.of(context).showSnackBar(
      const SnackBar(content: Text('Navigate to VPN')),
    );
  }
}