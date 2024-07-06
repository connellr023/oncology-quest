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

class _ExpandableSectionState extends State<ExpandableSection> {
  bool _isExpanded = false;

  void _toggleExpanded() {
    setState(() {
      _isExpanded = !_isExpanded;
    });
  }

  @override
  Widget build(BuildContext context) {
    const double borderRadius = 18;

    return Material(
      color: widget.backgroundColor,
      borderRadius: BorderRadius.circular(borderRadius),
      child: InkWell(
        splashColor: _isExpanded ? textColor.withOpacity(0.3) : themeColor,
        borderRadius: BorderRadius.circular(borderRadius),
        onTap: () => _toggleExpanded(),
        child: Column(
          children: <Widget>[
            ListTile(
              title: Text(
                widget.title,
                style: TextStyle(
                  color: textColor,
                  fontSize: MediaQuery.of(context).size.width * 0.042
                ),
              ),
              leading: Icon(
                _isExpanded ? Icons.arrow_drop_up : Icons.arrow_drop_down,
                color: _isExpanded ? themeColor : textColor,
                size: MediaQuery.of(context).size.width * 0.1
              )
            ),
            if (_isExpanded) ...widget.children
          ]
        )
      )
    );
  }
}