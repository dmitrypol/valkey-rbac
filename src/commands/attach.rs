use crate::{RBAC_ROLES, RBAC_USER_ROLE_MAP, commands};
use valkey_module::{Context, ValkeyError, ValkeyResult, ValkeyString, ValkeyValue};

/// attach user to role, applies role's ACL rules to the user
pub(crate) fn attach(ctx: &Context, args: &[ValkeyString]) -> ValkeyResult {
    if args.len() < 2 {
        return Err(ValkeyError::WrongArity);
    }
    let user = args[0].to_string();
    let role = args[1].to_string();
    // check if user exists
    let user_check = ctx.call("acl", &["getuser", user.as_str()])?;
    match user_check {
        ValkeyValue::Null => return Err(ValkeyError::Str("User does not exist")),
        _ => {}
    }
    // check if role exists
    let guard = RBAC_ROLES.read()?;
    if !guard.contains_key(&role) {
        return Err(ValkeyError::Str("Role does not exist"));
    }
    // attach user to role
    RBAC_USER_ROLE_MAP
        .write()?
        .insert(user.clone(), role.clone());
    // get ACL rules for the role
    let role_acl_rules: Vec<&str> = guard.get(&role).unwrap().split_whitespace().collect();
    commands::acl_setuser(ctx, user, role_acl_rules)
}
