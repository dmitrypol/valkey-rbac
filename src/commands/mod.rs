use valkey_module::{Context, ValkeyError, ValkeyResult, ValkeyString};

mod attach;
mod delrole;
mod detach;
mod getrole;
mod list;
mod roles;
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
        "LIST" => list::list(&args[2..]),
        "ROLES" => roles::roles(&args[2..]),
        "SETROLE" => setrole::setrole(ctx, &args[2..]),
        "ATTACH" => attach::attach(ctx, &args[2..]),
        "DETACH" => detach::detach(&args[2..]),
        _ => Err(ValkeyError::Str("invalid subcommand")),
    }
}
