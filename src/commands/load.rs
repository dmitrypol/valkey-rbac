use crate::commands::config_get_rbacfile;
use crate::{RBAC_ROLES, RBAC_USER_ROLE_MAP, RbacStore};
use std::fs::File;
use std::io::{Error, ErrorKind, Read};
use valkey_module::{Context, ValkeyResult};

/// load RBAC_ROLES from file
pub(crate) fn load(ctx: &Context) -> ValkeyResult {
    let path = config_get_rbacfile(ctx);
    let mut file = File::open(path)?;
    let mut json = String::new();
    file.read_to_string(&mut json)?;

    let parsed: RbacStore =
        serde_json::from_str(&json).map_err(|err| Error::new(ErrorKind::InvalidData, err))?;
    let mut roles = RBAC_ROLES.write().unwrap();
    *roles = parsed.roles;
    let mut user_role_map = RBAC_USER_ROLE_MAP.write().unwrap();
    *user_role_map = parsed.user_role_map;
    Ok("OK".into())
}
