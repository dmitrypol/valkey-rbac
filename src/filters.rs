use crate::RBAC_USER_ROLE_MAP;
use valkey_module::logging::log_notice;
use valkey_module::{
    CommandFilterCtx, Context, RedisModuleCommandFilterCtx, ValkeyError, ValkeyResult, ValkeyString,
};

/// restrict updating user if attached to a role
pub(crate) fn acl_setuser_filter(ctx: *mut RedisModuleCommandFilterCtx) {
    // ACL SETUSER user1 on nopass allkeys allcommands ...
    let cf_ctx = CommandFilterCtx::new(ctx);
    //if cf_ctx.args_count() < 3 { return; }
    // check if cmd (first arg) is ACL
    let cmd = cf_ctx.cmd_get_try_as_str().unwrap().to_lowercase();
    if cmd.ne("acl") {
        return;
    }
    // check if subcmd (next arg) is SETUSER
    let subcommand = cf_ctx.arg_get_try_as_str(1).unwrap().to_lowercase();
    if subcommand.ne("setuser") {
        return;
    }
    // check if user is attached to role
    let user = cf_ctx.arg_get_try_as_str(2).unwrap().to_lowercase();
    if RBAC_USER_ROLE_MAP.read().unwrap().contains_key(&user) {
        // if so, do not allow the command to proceed
        // TODO - only prevent updating ACL permissions, allow password and status
        let all_args = cf_ctx.get_all_args_wo_cmd();
        log_notice(&format!(
            "cannot update user as it is attached to role - {:?}",
            all_args
        ));
        cf_ctx.arg_replace(0, "rbac_filter");
    }
}

// TODO - find a better way to stop command execution from filter
// command called from filter restrict updating user if attached to a role
pub(crate) fn rbac_filter_cmd(_ctx: &Context, _args: Vec<ValkeyString>) -> ValkeyResult {
    log_notice(&format!("rbac_filter_cmd"));
    Err(ValkeyError::Str(
        "user is attached to a role, cannot update ACL permissions",
    ))
}
