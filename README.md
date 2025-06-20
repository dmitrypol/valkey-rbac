# valkey-rbac module

Module for managing Role-Based Access Control (RBAC) in Valkey.

## Features

Follows the pattern of Valkey ACL https://valkey.io/topics/acl/.

Has the main `RBAC` command which has the following sub-commands:

* SETROLE - similar to ACL SETUSER, but for roles.  
* * Allows to set permissions in ACL format but has no password.
* DELROLE - similar to ACL DELUSER, allows to delete a role.  
* * Users that were attached to the role will retain permissions.
* GETROLE - similar to ACL GETUSER, allows to get the permissions of a role.
* LIST - similar to ACL LIST, allows to list all roles with their permissions.
* ROLES - similar to ACL USERS, allows to list all role names.
* SAVE - saves the current RBAC roles and users attachments to disk.
* LOAD - loads the RBAC roles and users attachments from disk.
* ATTACH - attaches existing user to a role, permissions of the role will override what user has.  
* * After that it's ACL permissions can no longer be modified directly on the user. 
* * Any updates to the role's permissions will apply to all users attached to that role.  
* * User can be attached to only one role.
* DETACH - detaches user from a role.  
* * User retains permissions role has at the time but now it's permissions can be modified directly.
* * Any updates to the role's permissions will not apply to the user anymore.
* ROLEUSERS - similar to ACL USERS, allows to list all users assigned to a specific role.
* HELP - shows help for the RBAC command.

These commands have to be executed on each node to apply the changes to the whole cluster or primary/replica.  

Backwards compatible to Redis >= 7.2.0

Built using https://github.com/valkey-io/valkeymodule-rs