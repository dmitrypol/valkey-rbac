mod utils;

use crate::utils::*;
use anyhow::Context;
use redis::{Connection, RedisError};

#[test]
fn test_rbac() -> anyhow::Result<()> {
    let _guards = vec![start_server().with_context(|| "failed to start server")?];
    let mut con = get_server_connection().with_context(|| "failed to connect to server")?;

    let test: Result<String, RedisError> = redis::cmd("rbac").arg(&["invalid"]).query(&mut con);
    if test.is_err() {
        assert_eq!(test.err().unwrap().to_string(), "invalid: subcommand");
    }

    let _ = test_setrole(&mut con)?;
    let _ = test_list_roles(&mut con)?;
    let _ = test_getrole(&mut con)?;
    let _ = test_attach(&mut con)?;
    let _ = test_detach(&mut con)?;
    let _ = test_delrole(&mut con)?;
    Ok(())
}

fn test_setrole(mut con: &mut Connection) -> anyhow::Result<()> {
    // "+GET -@dangerous ~cache:* reset -SET -flushdb -@admin +@read -@write resetkeys resetpass resetchannels allcommands nochannels";
    let test: String = redis::cmd("rbac")
        .arg(&["setrole", "rolea", "allkeys", "allcommands", "allchannels"])
        .query(&mut con)?;
    assert_eq!(test, "OK");
    let test: String = redis::cmd("rbac")
        .arg(&["setrole", "roleb", "+get", "-@dangerous", "~*", "&*"])
        .query(&mut con)?;
    assert_eq!(test, "OK");
    // missing role name
    let test: Result<String, RedisError> = redis::cmd("rbac").arg(&["setrole"]).query(&mut con);
    assert!(test.is_err());
    // missing rules
    let test: Result<String, RedisError> = redis::cmd("rbac")
        .arg(&["setrole", "rolea"])
        .query(&mut con);
    assert!(test.is_err());
    // invalid rules
    let test: Result<String, RedisError> = redis::cmd("rbac")
        .arg(&["setrole", "rolea", "invalid"])
        .query(&mut con);
    assert!(test.is_err());
    assert_eq!(
        test.err().unwrap().to_string(),
        "Invalid: ACL rule: invalid"
    );
    // invalid category
    let test: Result<String, RedisError> = redis::cmd("rbac")
        .arg(&["setrole", "rolea", "+@invalid-cat"])
        .query(&mut con);
    assert!(test.is_err());
    // invalid command
    let test: Result<String, RedisError> = redis::cmd("rbac")
        .arg(&["setrole", "rolea", "-invalid-cmd"])
        .query(&mut con);
    assert!(test.is_err());
    // TODO update role with attached user and verify user is updated
    Ok(())
}

fn test_list_roles(mut con: &mut Connection) -> anyhow::Result<()> {
    // list
    let test: Vec<String> = redis::cmd("rbac").arg(&["list"]).query(&mut con)?;
    assert_eq!(
        test,
        vec![
            "rolea",
            "allkeys allcommands allchannels",
            "roleb",
            "+get -@dangerous ~* &*"
        ]
    );
    // roles
    let mut test: Vec<String> = redis::cmd("rbac").arg(&["roles"]).query(&mut con)?;
    assert_eq!(test.sort(), vec!["rolea", "roleb"].sort());
    Ok(())
}

fn test_getrole(mut con: &mut Connection) -> anyhow::Result<()> {
    let test: Vec<String> = redis::cmd("rbac")
        .arg(&["getrole", "rolea"])
        .query(&mut con)?;
    assert_eq!(test, vec!["rolea", "allkeys allcommands allchannels"]);
    let test: Vec<String> = redis::cmd("rbac")
        .arg(&["getrole", "invalid"])
        .query(&mut con)?;
    assert_eq!(test.len(), 0);
    let test: Result<String, RedisError> = redis::cmd("rbac").arg(&["getrole"]).query(&mut con);
    assert!(test.is_err());
    Ok(())
}

fn test_attach(mut con: &mut Connection) -> anyhow::Result<()> {
    // create a user
    let _: Result<String, RedisError> =
        redis::cmd("acl").arg(&["setuser", "user1"]).query(&mut con);
    let test: Result<String, RedisError> = redis::cmd("rbac")
        .arg(&["attach", "invalid-user", "rolea"])
        .query(&mut con);
    assert!(test.is_err());
    let test: Result<String, RedisError> = redis::cmd("rbac")
        .arg(&["attach", "user1", "invalid-role"])
        .query(&mut con);
    assert!(test.is_err());
    let test: String = redis::cmd("rbac")
        .arg(&["attach", "user1", "rolea"])
        .query(&mut con)?;
    assert_eq!(test, "OK".to_string());
    // TODO check if user1 has rolea permissions, acl getuser user1
    Ok(())
}

fn test_detach(mut con: &mut Connection) -> anyhow::Result<()> {
    let test: Result<String, RedisError> = redis::cmd("rbac")
        .arg(&["detach", "invalid-user", "rolea"])
        .query(&mut con);
    assert!(test.is_err());
    let test: Result<String, RedisError> = redis::cmd("rbac")
        .arg(&["attach", "user1", "invalid-role"])
        .query(&mut con);
    assert!(test.is_err());
    let test: String = redis::cmd("rbac")
        .arg(&["detach", "user1", "rolea"])
        .query(&mut con)?;
    assert_eq!(test, "OK".to_string());
    // TODO check if user1 still retains rolea permissions, acl getuser user1
    Ok(())
}

fn test_delrole(mut con: &mut Connection) -> anyhow::Result<()> {
    let test: i8 = redis::cmd("rbac")
        .arg(&["delrole", "rolea", "roleb", "invalid"])
        .query(&mut con)?;
    assert_eq!(test, 2);
    let test: Result<String, RedisError> = redis::cmd("rbac").arg(&["delrole"]).query(&mut con);
    assert!(test.is_err());
    let test: Vec<String> = redis::cmd("rbac").arg(&["roles"]).query(&mut con)?;
    assert_eq!(test.len(), 0);
    let test: Vec<String> = redis::cmd("rbac").arg(&["list"]).query(&mut con)?;
    assert_eq!(test.len(), 0);
    // TODO check if user1 still retains rolea permissions, acl getuser user1
    Ok(())
}
