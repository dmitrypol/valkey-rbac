# valkey-rbac module

Module for managing Role-Based Access Control (RBAC) in Valkey.

## Features

Follows the pattern of Valkey ACL https://valkey.io/topics/acl/.

Has the main `RBAC` command which has the following sub-commands:

* SETROLE - similar to ACL SETUSER, but for roles.  Allows you to set permissions in ACL format but has no password.
* DELROLE - similar to ACL DELUSER, allows you to delete a role.
* GETROLE - similar to ACL GETUSER, allows you to get the permissions of a role.
* LIST - similar to ACL LIST, allows you to list all roles with their permissions.
* ROLES - similar to ACL USERS, allows you to list all role names.
* SAVE - saves the current RBAC configuration to disk.
* LOAD - loads the RBAC configuration from disk.
* ATTACH - attaches existing user to a role, any permissions of the role override what user has.  After that it's ACL permissions can no longer be modified directly.  Updates to the role's permissions will apply to the user.  User can be attached to only one role.
* DETACH - detaches user from a role.  User retains permissions role has at the time but now it's permissions can be modified directly.  
* ROLEUSERS - similar to ACL USERS, allows you to list all users assigned to a specific role.
* HELP - shows help for the RBAC command.

Backwards compatible to Redis >= 7.2.0

Built using https://github.com/valkey-io/valkeymodule-rs