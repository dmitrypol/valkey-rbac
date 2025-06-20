use crate::{ACL_CATEGORIES, ACL_FLAGS, COMMAND_LIST, RBAC_ROLES};
use valkey_module::{ValkeyError, ValkeyResult, ValkeyString};

/// validates rules to follow ACL syntax and creates or updates RBAC_ROLES map
pub(crate) fn setrole(args: &[ValkeyString]) -> ValkeyResult {
    if args.len() < 2 {
        return Err(ValkeyError::WrongArity);
    }
    let role = args[0].to_string();
    let mut acl_rules_vec = Vec::new();
    for arg in args.iter().skip(1) {
        acl_rules_vec.push(arg.to_string());
    }
    let acl_rules = acl_rules_vec.join(" ");
    match validate_acl_string(acl_rules.clone()) {
        Ok(_) => {
            RBAC_ROLES.write()?.insert(role, acl_rules);
            // TODO apply rules to the users attached to this role
            Ok("OK".into())
        }
        Err(err) => Err(ValkeyError::String(err)),
    }
}

fn validate_acl_string(acl_rules: String) -> Result<(), String> {
    for token in acl_rules.split_whitespace() {
        if !is_valid_acl_token(token) {
            return Err(format!("Invalid ACL rule: {}", token));
        }
    }
    Ok(())
}

fn is_valid_acl_token(token: &str) -> bool {
    // "+GET -@dangerous ~cache:* reset -SET -flushdb -@admin +@read -@write resetkeys resetpass resetchannels allcommands nochannels";
    let token = token.trim().to_ascii_lowercase();
    if ACL_FLAGS.contains(&token.as_str()) {
        return true;
    }
    if token.starts_with('+') || token.starts_with('-') {
        let cmd_or_cat = &token[1..];
        if cmd_or_cat.is_empty() {
            return false; // Empty command or category is invalid
        }
        if cmd_or_cat.starts_with('@') {
            // Category
            return ACL_CATEGORIES
                .read()
                .unwrap()
                .contains(&cmd_or_cat[1..].to_string());
        } else {
            // Command
            return COMMAND_LIST
                .read()
                .unwrap()
                .contains(&cmd_or_cat.to_string());
        }
    } else if token.starts_with('~') {
        // key pattern
        return true;
    } else if token.starts_with('&') {
        // channel pattern
        return true;
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_args() {
        let test = setrole(&vec![]);
        assert!(test.is_err());
        assert!(matches!(test, Err(ValkeyError::WrongArity)));
    }
}
