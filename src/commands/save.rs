use crate::commands::config_get_rbacfile;
use crate::{RBAC_ROLES, RBAC_USER_ROLE_MAP, RbacStore};
use std::fs::File;
use std::io::Write;
use valkey_module::{Context, ValkeyResult};

/// save RBAC_ROLES to file
pub(crate) fn save(ctx: &Context) -> ValkeyResult {
    let rbac_roles = RBAC_ROLES.read()?;
    let rbac_user_role_map = RBAC_USER_ROLE_MAP.read()?;
    let rbac_store = RbacStore {
        roles: rbac_roles.clone(),
        user_role_map: rbac_user_role_map.clone(),
    };
    let json = serde_json::to_string_pretty(&rbac_store)?;
    let path = config_get_rbacfile(ctx);
    let mut file = File::create(path)?;
    file.write_all(json.as_bytes())?;
    Ok("OK".into())
}
