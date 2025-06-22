use valkey_module::{Context, ValkeyError, ValkeyResult, ValkeyString, ValkeyValue};

mod attach;
mod delrole;
mod detach;
mod getrole;
mod help;
mod list;
pub(crate) mod load;
mod roles;
pub(crate) mod save;
mod setrole;

/// main rbac command handler
pub(crate) fn rbac(ctx: &Context, args: Vec<ValkeyString>) -> ValkeyResult {
    if args.len() < 2 {
        return Err(ValkeyError::WrongArity);
    }
    // command is 0 arg, subcommand is 1 arg, actual args start from 2 and pass to subcommand
    let subcommand = &args[1].to_string().to_uppercase();
    match subcommand.as_str() {
        "DELROLE" => delrole::delrole(&args[2..]),
        "GETROLE" => getrole::getrole(&args[2..]),
        "LIST" => list::list(),
        "ROLES" => roles::roles(),
        "SETROLE" => setrole::setrole(ctx, &args[2..]),
        "ATTACH" => attach::attach(ctx, &args[2..]),
        "DETACH" => detach::detach(&args[2..]),
        "SAVE" => save::save(ctx),
        "LOAD" => load::load(ctx),
        "HELP" => help::help(),
        _ => Err(ValkeyError::Str("invalid subcommand")),
    }
}

pub(crate) fn config_get_rbacfile(ctx: &Context) -> String {
    // TODO - refactor
    let dir = ctx.call("config", &["get", "dir"]).unwrap();
    // Array([SimpleString("dir"), SimpleString(".../valkey-rbac")])
    let dir = match dir {
        ValkeyValue::Array(tmp) => {
            let tmp = tmp.get(1).unwrap();
            match tmp {
                ValkeyValue::SimpleString(tmp) => tmp.to_string(),
                _ => "".to_string(),
            }
        }
        _ => "".to_string(),
    };
    let rbacfile = ctx
        .call("config", &["get", "valkey-rbac.rbacfile"])
        .unwrap();
    // Array([SimpleString("valkey-rbac.rbacfile"), SimpleString("rbac.json")])
    let rbacfile = match rbacfile {
        ValkeyValue::Array(tmp) => {
            let tmp = tmp.get(1).unwrap();
            match tmp {
                ValkeyValue::SimpleString(tmp) => tmp.to_string(),
                _ => "".to_string(),
            }
        }
        _ => "".to_string(),
    };
    let output = format!("{}/{}", dir, rbacfile);
    output
}

pub(crate) fn acl_setuser(ctx: &Context, user: String, role_acl_rules: Vec<&str>) -> ValkeyResult {
    // add "setuser" command and user to args for acl setuser command
    let mut acl_setuser_args = role_acl_rules;
    acl_setuser_args.insert(0, "setuser");
    acl_setuser_args.insert(1, user.as_str());
    // call ACL SETUSER to apply role's ACL rules to the user and return the result
    ctx.call("acl", &acl_setuser_args[..])
}
