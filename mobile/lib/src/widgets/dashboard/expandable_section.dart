import 'package:flutter/material.dart';
import 'package:oncology_quest_mobile/src/utilities/colors.dart';

class ExpandableSection extends StatefulWidget {
  final Color backgroundColor;
  final String title;
  final List<Widget> children;

  const ExpandableSection({
    super.key,
    required this.backgroundColor,
    required this.title,
    required this.children
  });

  @override
  State<ExpandableSection> createState() => _ExpandableSectionState();
}

class _ExpandableSectionState extends State<ExpandableSection> with SingleTickerProviderStateMixin {
  late AnimationController _controller;
  late Animation<double> _animation;
  bool _isExpanded = false;

  @override
  void initState() {
    super.initState();

    _controller = AnimationController(
      duration: const Duration(milliseconds: 150),
      vsync: this,
    );

    _animation = CurvedAnimation(parent: _controller, curve: Curves.fastOutSlowIn);
  }

  @override
  void dispose() {
    _controller.dispose();
    super.dispose();
  }

  void _toggleExpand() {
    setState(() {
      _isExpanded = !_isExpanded;

      if (_isExpanded) {
        _controller.forward();
      }
      else {
        _controller.reverse();
      }
    });
  }

  @override
  Widget build(BuildContext context) {
    return Material(
      color: widget.backgroundColor,
      child: Column(
        children: [
          InkWell(
            splashColor: _isExpanded ? textColor.withOpacity(0.3) : themeColor,
            onTap: () => _toggleExpand(),
            child: ListTile(
              title: Text(
                widget.title,
                style: TextStyle(
                  color: textColor,
                  fontSize: MediaQuery.of(context).size.width * 0.044
                ),
              ),
              leading: Icon(
                _isExpanded ? Icons.arrow_drop_up : Icons.arrow_drop_down,
                color: _isExpanded ? themeColor : textColor,
                size: MediaQuery.of(context).size.width * 0.1
              )
            )
          ),
          SizeTransition(
            axisAlignment: 1.0,
            sizeFactor: _animation,
            child: Column(children: widget.children)
          )
        ]
      )
    );
  }
}