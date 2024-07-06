import 'package:flutter/material.dart';
import 'package:oncology_quest_mobile/src/utilities/colors.dart';

class ExpandableEntryLayer extends StatefulWidget {
  final Color backgroundColor;
  final String title;
  final List<Widget> children;

  const ExpandableEntryLayer({
    super.key,
    required this.backgroundColor,
    required this.title,
    required this.children
  });

  @override
  State<ExpandableEntryLayer> createState() => _ExpandableEntryLayerState();
}

class _ExpandableEntryLayerState extends State<ExpandableEntryLayer> with SingleTickerProviderStateMixin {
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
            child: Row(
              children: [
                Expanded(
                  child: ListTile(
                    title: Text(
                      widget.title,
                      style: TextStyle(
                        color: textColor,
                        fontSize: MediaQuery.of(context).size.width * 0.044
                      )
                    ),
                    leading: Icon(
                      _isExpanded ? Icons.arrow_drop_up : Icons.arrow_drop_down,
                      color: _isExpanded ? themeColor : textColor,
                      size: MediaQuery.of(context).size.width * 0.1
                    )
                  ),
                ),
                _buildProgressIndicator() 
              ],
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

  Widget _buildProgressIndicator() {
    const double height = 8;

    return Expanded(
      child: Padding(
        padding: const EdgeInsets.only(right: 15),
        child: Row(
          children: [
            Expanded(
              child: SizedBox(
                height: height,
                child: LinearProgressIndicator(
                  borderRadius: BorderRadius.circular(height),
                  value: 0,
                  backgroundColor: textColor.withOpacity(0.3),
                  valueColor: const AlwaysStoppedAnimation(okColor)
                ),
              )
            ),
            const SizedBox(width: 13),
            Text(
              '0%',
              style: TextStyle(
                color: textColor,
                fontSize: MediaQuery.of(context).size.width * 0.04
              )
            )
          ]
        )
      )
    );
  }
}