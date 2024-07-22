import 'package:flutter/material.dart';
import 'package:oncology_quest_mobile/src/models/client_user.dart';
import 'package:oncology_quest_mobile/src/state/selected_rotation_state.dart';
import 'package:oncology_quest_mobile/src/state/user_tasks_state.dart';
import 'package:oncology_quest_mobile/src/utilities.dart';
import 'package:oncology_quest_mobile/src/widgets/dashboard/default_profile_icon.dart';
import 'package:oncology_quest_mobile/src/widgets/dashboard/section_heading.dart';

import 'package:http/http.dart' as http;
import 'dart:convert';

import 'package:provider/provider.dart';

class SearchUsersDrawer extends StatefulWidget {
  final String jwt;

  const SearchUsersDrawer({
    super.key,
    required this.jwt
  });

  @override
  State<SearchUsersDrawer> createState() => _SearchUsersDrawerState();
}

class _SearchUsersDrawerState extends State<SearchUsersDrawer> {
  Map<int, ClientUser> _searchResults = {};
  String _query = "";

  Future<String?> _searchUsers() async {
    if (_query.isEmpty) {
      return null;
    }

    try {
      final response = await http.get(apiEndpoint.resolve('/api/users/search/$_query'), headers: {
        'authorization': widget.jwt,
        'content-type': 'application/json'
      });

      if (response.statusCode == 200) {
        final body = json.decode(response.body);
        final results = deserializeSearchResults(body);

        setState(() {
          _searchResults = results;
        });
      }
      else {
        return "Failed to search for users. Please try again later.";
      }
    }
    catch (_) {
      return "Failed to connect to the server. Please try again later.";
    }

    return null;
  }

  bool _isUserSelected(ClientUser user) {
    final userTasksState = Provider.of<UserTasksState>(context, listen: false);
    return userTasksState.selectedUser?.id == user.id;
  }

  void _selectUser(ClientUser? user) {
    final userTasksState = Provider.of<UserTasksState>(context, listen: false);
    final selectedRotationState = Provider.of<SelectedRotationState>(context, listen: false);

    attemptFallible(context, () async {
      // Update displayed progress if a rotation is selected.
      if (selectedRotationState.selectedRotationId != null && user != null) {
        userTasksState.clearProgressMemo();
        String? error = await userTasksState.fetchUserTasks(widget.jwt, selectedRotationState.selectedRotationId!, user);
        
        if (error != null) {
          return error;
        }
      }
      
      return null;
    });
  }

  @override
  Widget build(BuildContext context) {
    return SizedBox(
      width: drawerUiWidth(context),
      height: double.infinity,
      child: Material(
        color: backgroundColor2,
        child: SingleChildScrollView(
          padding: EdgeInsets.only(
            top: inMobileViewport(context) ? kToolbarHeight : 20,
            left: 15,
            right: 15
          ),
          child: Column(
            children: <Widget>[
              const Row(
                mainAxisAlignment: MainAxisAlignment.center,
                children: <Widget>[
                  SectionHeading(title: 'Search Users'),
                ]
              ),
              const SizedBox(height: 12),
              _buildSearchField(context),
              const SizedBox(height: 20),
              if (_searchResults.isNotEmpty) Consumer<UserTasksState>(
                builder: (context, selectedUserState, _) => Column(
                    children: _searchResults.values
                      .map((user) => _buildSearchResult(context, user))
                      .toList()
                  )
              )
              else Text(
                'No users found.',
                style: TextStyle(
                  color: textColor.withOpacity(0.6),
                  fontSize: standardFontSize(context)
                )
              )
            ]
          )
        )
      )
    );
  }

  Widget _buildSearchResult(BuildContext context, ClientUser user) {
    final size = standardFontSize(context);
    final borderRadius = BorderRadius.circular(18);
    final isSelected = _isUserSelected(user);

    const splashOpacity = 0.3;

    return Padding(
      padding: const EdgeInsets.only(
        top: 5,
        bottom: 5
      ),
      child: Material(
        color: backgroundColor3,
        borderRadius: borderRadius,
        child: InkWell(
          borderRadius: borderRadius,
          onTap: () => isSelected ? _selectUser(null) : _selectUser(user),
          splashColor: isSelected ? textColor.withOpacity(splashOpacity) : okColor.withOpacity(splashOpacity),
          child: Padding(
            padding: const EdgeInsets.all(15),
            child: Row(
              children: <Widget>[
                DefaultProfileIcon(
                  size: size * 2,
                  name: user.name,
                  onTap: () => {}
                ),
                const SizedBox(width: 10),
                if (isSelected) ...<Widget>[
                  Icon(
                    Icons.check,
                    color: okColor,
                    size: size * 1.65
                  ),
                  const SizedBox(width: 10)
                ],
                RichText(
                  text: TextSpan(
                    style: TextStyle(
                      fontSize: size,
                      color: isSelected ? okColor : textColor,
                    ),
                    children: <TextSpan>[
                      TextSpan(
                        text: user.name,
                        style: const TextStyle(fontWeight: FontWeight.bold)
                      ),
                      TextSpan(text: ' (${user.username})')
                    ]
                  )
                )
              ]
            )
          )
        )
      )
    );
  }

  Widget _buildSearchField(BuildContext context) {
    final size = standardFontSize(context);

    return Container(
      decoration: BoxDecoration(
        color: backgroundColor3,
        borderRadius: BorderRadius.circular(18)
      ),
      child: TextField(
        onChanged: (value) => _query = value,
        style: TextStyle(
          color: textColor,
          fontSize: size
        ),
        decoration: InputDecoration(
          hintText: 'Search...',
          hintStyle: TextStyle(
            color: textColor.withOpacity(0.5),
            fontSize: size
          ),
          border: InputBorder.none,
          contentPadding: const EdgeInsets.symmetric(
            horizontal: 17,
            vertical: 17
          ),
          suffixIcon: Padding(
            padding: const EdgeInsets.only(right: 5),
            child: IconButton(
              hoverColor: textColor.withOpacity(0.1),
              focusColor: textColor.withOpacity(0.3),
              color: textColor.withOpacity(0.5),
              icon: Icon(
                Icons.search,
                size: size * 1.65
              ),
              onPressed: () => attemptFallible(context, () => _searchUsers())
            ),
          )
        )
      )
    );
  }
}