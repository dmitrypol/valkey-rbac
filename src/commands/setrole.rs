use crate::RBAC_ROLES;
use valkey_module::{ValkeyError, ValkeyResult, ValkeyString};

pub fn setrole(args: &[ValkeyString]) -> ValkeyResult {
    if args.len() < 2 {
        return Err(ValkeyError::WrongArity);
    }
    let role = args[0].to_string();
    let rules = args[1].to_string();
    RBAC_ROLES.write().unwrap().insert(role, rules);
    Ok(().into())
}
