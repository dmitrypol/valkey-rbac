use crate::RBAC_ROLES;
use valkey_module::{ValkeyError, ValkeyResult, ValkeyString, ValkeyValue};

pub fn delrole(args: &[ValkeyString]) -> ValkeyResult {
    if args.len() < 1 {
        return Err(ValkeyError::WrongArity);
    }
    let mut output = 0;
    for role in args {
        let tmp = RBAC_ROLES.write().unwrap().remove(&role.to_string());
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
