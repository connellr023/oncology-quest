A sample admin account represented by the following **JSON** string:
```json
{
    "username": "admin",
    "name": "dasiad",
    "email": "dioa@isodf.ca",
    "can_reset_password": false,
    "is_admin": true,
    "tasks": {},
    "salt": 2738682831769974000,
    "password": "$2b$12$kOcOS4bX3rE3xMwUofzyGucaKMgtP4UL49DNP6WtDN8AYD9wVQ8z."
}
```
The above admin account (to be used for testing only) has a username of *admin* and a password
of *aaaaaaaa*. Ensure the account key (in this case `user:admin`) is added to the `user_keys` set.

Can be inserted into **REDIS** with the following command:
```bash
redis-cli -a <password> SET user:admin "$(jq -c . < admin.json)"
```