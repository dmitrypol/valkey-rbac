use crate::{RBAC_ROLES, RBAC_USER_ROLE_MAP};
use valkey_module::{ValkeyError, ValkeyResult, ValkeyString};

/// detach user from role, leaves role's ACL rules for the user
pub(crate) fn detach(args: &[ValkeyString]) -> ValkeyResult {
    if args.len() < 2 {
        return Err(ValkeyError::WrongArity);
    }
    let user = args[0].to_string();
    let role = args[1].to_string();
    // check if user attachment exists
    if !RBAC_USER_ROLE_MAP.read()?.contains_key(&user) {
        return Err(ValkeyError::Str("User is not attached to any role"));
    }
    // check if role exists
    if !RBAC_ROLES.read()?.contains_key(&role) {
        return Err(ValkeyError::Str("Role does not exist"));
    }
    // detach user from role
    RBAC_USER_ROLE_MAP.write()?.remove(&user);
    Ok("OK".into())
}
