use crate::RBAC_ROLES;
use valkey_module::{ValkeyResult, ValkeyValue};

/// return array of role names
pub(crate) fn roles() -> ValkeyResult {
    let guard = RBAC_ROLES.read()?;
    let role_list_vs: Vec<String> = guard.keys().cloned().collect();
    let role_list_vvv: Vec<ValkeyValue> = role_list_vs.iter().map(|s| s.as_str().into()).collect();
    let output = ValkeyValue::Array(role_list_vvv);
    Ok(output)
}
