use crate::{RBAC_ROLES, RBAC_USER_ROLE_MAP};
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
    acl_setuser(ctx, user, role_acl_rules)
}

pub(crate) fn acl_setuser(ctx: &Context, user: String, role_acl_rules: Vec<&str>) -> ValkeyResult {
    // add "setuser" command and user to args for acl setuser command
    let mut acl_setuser_args = role_acl_rules;
    acl_setuser_args.insert(0, "setuser");
    acl_setuser_args.insert(1, user.as_str());
    // call ACL SETUSER to apply role's ACL rules to the user and return the result
    ctx.call("acl", &acl_setuser_args[..])
}
