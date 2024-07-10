import 'package:flutter/material.dart';
import 'package:oncology_quest_mobile/src/models/session.dart';
import 'package:oncology_quest_mobile/src/state/user_tasks_state.dart';
import 'package:oncology_quest_mobile/src/utilities/colors.dart';
import 'package:oncology_quest_mobile/src/utilities/sizing.dart';
import 'package:oncology_quest_mobile/src/widgets/dashboard/basic_option.dart';
import 'package:provider/provider.dart';

class ProgressableEntryLayer extends StatefulWidget {
  final Session session;
  final Color backgroundColor;
  final String title;
  final List<Widget> children;
  final double Function(UserTasksState) calculateProgress;

  const ProgressableEntryLayer({
    super.key,
    required this.session,
    required this.backgroundColor,
    required this.title,
    required this.children,
    required this.calculateProgress
  });

  @override
  State<ProgressableEntryLayer> createState() => _ProgressableEntryLayerState();
}

class _ProgressableEntryLayerState extends State<ProgressableEntryLayer> with SingleTickerProviderStateMixin {
  late AnimationController _slideController;
  late Animation<double> _slideAnimation;

  bool _isExpanded = false;

  @override
  void initState() {
    super.initState();

    _slideController = AnimationController(
      duration: const Duration(milliseconds: 150),
      vsync: this,
    );

    _slideAnimation = CurvedAnimation(
      parent: _slideController,
      curve: Curves.fastOutSlowIn
    );
  }

  @override
  void dispose() {
    _slideController.dispose();
    super.dispose();
  }

  void _toggleExpand() {
    _isExpanded = !_isExpanded;

    if (_isExpanded) {
      _slideController.forward();
    }
    else {
      _slideController.reverse();
    }
  }

  @override
  Widget build(BuildContext context) {
    final size = standardFontSize(context);

    return Material(
      color: widget.backgroundColor,
      child: Column(
        children: [
          StatefulBuilder(
            builder: (BuildContext context, StateSetter setState) => InkWell(
              splashColor: _isExpanded ? textColor.withOpacity(0.3) : themeColor,
              onTap: () => setState(() => _toggleExpand()),
              child: Padding(
                padding: const EdgeInsets.only(
                  top: 7,
                  bottom: 7
                ),
                child: Row(
                  children: [
                    Expanded(
                      flex: 2,
                      child: ListTile(
                        mouseCursor: MouseCursor.uncontrolled,
                        title: Text(
                          widget.title,
                          style: TextStyle(
                            color: textColor,
                            fontSize: size
                          )
                        ),
                        leading: Icon(
                          _isExpanded ? Icons.arrow_drop_up : Icons.arrow_drop_down,
                          color: _isExpanded ? themeColor : textColor,
                          size: size * 2.3
                        )
                      )
                    ),
                    if (widget.session.user.isAdmin) Padding(
                      padding: const EdgeInsets.only(
                        right: 10,
                        left: 10
                      ),
                      child: BasicOption(
                        context: context,
                        title: 'Edit',
                        color: textColor,
                        icon: Icons.edit,
                        onTap: () => {}
                      ),
                    ),
                    _buildProgressIndicator() 
                  ]
                )
              )
            )
          ),
          SizeTransition(
            axisAlignment: 1.0,
            sizeFactor: _slideAnimation,
            child: Column(children: widget.children)
          )
        ]
      )
    );
  }

  Widget _buildProgressIndicator() {
    return Expanded(
      child: Padding(
        padding: const EdgeInsets.only(right: 17),
        child: Consumer<UserTasksState>(
          builder: (context, userTasksState, child) {
            final progress = widget.calculateProgress(userTasksState);
            return _ProgressIndicator(progress: progress);
          }
        )
      )
    );
  }
}

class _ProgressIndicator extends StatefulWidget {
  final double progress;

  const _ProgressIndicator({required this.progress});

  @override
  State<_ProgressIndicator> createState() => _ProgressIndicatorState();
}

class _ProgressIndicatorState extends State<_ProgressIndicator> with SingleTickerProviderStateMixin {
  late AnimationController _controller;
  late Animation<double> _progressAnimation;

  @override
  void initState() {
    super.initState();

    _controller = AnimationController(
      duration: const Duration(milliseconds: 300),
      vsync: this
    );

    _progressAnimation = Tween<double>(
      begin: widget.progress,
      end: widget.progress
    ).animate(_controller);
  }

  @override
  void didUpdateWidget(_ProgressIndicator oldWidget) {
    super.didUpdateWidget(oldWidget);

    if (oldWidget.progress != widget.progress) {
      _progressAnimation = Tween<double>(
        begin: oldWidget.progress,
        end: widget.progress
      ).animate(CurvedAnimation(
        parent: _controller,
        curve: Curves.fastOutSlowIn
      ));

      _controller
        ..reset()
        ..forward();
    }
  }

  @override
  void dispose() {
    _controller.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    double size = standardFontSize(context);
    const double height = 8;

    return AnimatedBuilder(
      animation: _progressAnimation,
      builder: (context, child) => Row(
        children: [
          Expanded(
            flex: 1,
            child: SizedBox(
              height: height,
              child: LinearProgressIndicator(
                borderRadius: BorderRadius.circular(height),
                value: _progressAnimation.value,
                backgroundColor: textColor.withOpacity(0.3),
                valueColor: AlwaysStoppedAnimation(
                  _progressAnimation.value < 0.5
                    ? Color.lerp(errorColor, warningColor, _progressAnimation.value * 2)
                    : Color.lerp(warningColor, okColor, (_progressAnimation.value - 0.5) * 2),
                )
              )
            )
          ),
          SizedBox(width: size * 0.3),
          SizedBox(
            width: size * 3,
            child: Text(
              '${(_progressAnimation.value * 100).round()}%',
              textAlign: TextAlign.right,
              style: TextStyle(
                color: textColor,
                fontSize: size,
                fontStyle: FontStyle.italic
              )
            ),
          )
        ]
      )
    );
  }
}