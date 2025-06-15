use crate::RBAC_ROLES;
use std::collections::BTreeMap;
use valkey_module::{ValkeyResult, ValkeyString, ValkeyValue};

/// return map of roles and rules in each role
pub fn list(_args: &[ValkeyString]) -> ValkeyResult {
    let mut output = BTreeMap::new();
    let guard = RBAC_ROLES.read().unwrap();
    for (role, rules) in guard.iter() {
        output.insert(role.clone().into(), rules.clone().into());
    }
    Ok(ValkeyValue::OrderedMap(output))
}
