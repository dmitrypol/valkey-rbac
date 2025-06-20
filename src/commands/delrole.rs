use crate::RBAC_ROLES;
use valkey_module::{ValkeyError, ValkeyResult, ValkeyString, ValkeyValue};

/// delete role(s) from RBAC_ROLES, remove user to role attachments from RBAC_USER_ROLE_MAP
pub(crate) fn delrole(args: &[ValkeyString]) -> ValkeyResult {
    if args.len() < 1 {
        return Err(ValkeyError::WrongArity);
    }
    let mut output = 0;
    for role in args {
        let tmp = RBAC_ROLES.write()?.remove(&role.to_string());
        // TODO - loop through RBAC_USER_ROLE_MAP to remove entries for users attached to this role
        match tmp {
            Some(_) => output += 1, // role found and deleted
            None => {}              // role not found
        }
    }
    Ok(ValkeyValue::Integer(output))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_args() {
        let test = delrole(&vec![]);
        assert!(test.is_err());
        assert!(matches!(test, Err(ValkeyError::WrongArity)));
    }
}
