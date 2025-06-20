use crate::RBAC_ROLES;
use std::collections::BTreeMap;
use valkey_module::{ValkeyError, ValkeyResult, ValkeyString, ValkeyValue};

/// get role by name, return its rules
pub(crate) fn getrole(args: &[ValkeyString]) -> ValkeyResult {
    if args.len() < 1 {
        return Err(ValkeyError::WrongArity);
    }
    let role = args[0].to_string();
    let guard = RBAC_ROLES.write()?;
    let mut output = BTreeMap::new();
    match guard.get(&role) {
        Some(tmp) => {
            // TODO - return rules as ValkeyValue
            output.insert(role.clone().into(), tmp.clone().into());
            Ok(ValkeyValue::OrderedMap(output))
        }
        None => Ok(ValkeyValue::Null),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_args() {
        let test = getrole(&vec![]);
        assert!(test.is_err());
        assert!(matches!(test, Err(ValkeyError::WrongArity)));
    }
}
