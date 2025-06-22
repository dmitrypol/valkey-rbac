use valkey_module::{ValkeyResult, ValkeyValue};

pub(crate) fn help() -> ValkeyResult {
    let output = vec![
        ValkeyValue::SimpleString("ATTACH - <user> <role>".into()),
        ValkeyValue::SimpleString("DELROLE - <role>, similar to ACL DELUSER".into()),
        ValkeyValue::SimpleString("DETACH - <user> <role>".into()),
        ValkeyValue::SimpleString("GETROLE - <user>, similar to ACL GETUSER".into()),
        ValkeyValue::SimpleString("HELP - print this message".into()),
        ValkeyValue::SimpleString("LIST - similar to ACL LIST".into()),
        ValkeyValue::SimpleString("LOAD - similar to ACL LOAD".into()),
        ValkeyValue::SimpleString("ROLES - similar to ACL USERS".into()),
        ValkeyValue::SimpleString("SAVE - similar to ACL SAVE".into()),
        ValkeyValue::SimpleString("SETROLE - <role> <ACL string>, similar to ACL SETUSER".into()),
    ];
    Ok(output.into())
}
