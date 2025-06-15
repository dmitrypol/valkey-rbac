mod utils;

use crate::utils::*;
use anyhow::{Context, Result};
use redis::RedisError;

#[test]
fn test_rbac() -> Result<()> {
    let _guards = vec![start_server().with_context(|| "failed to start server")?];
    let mut con = get_server_connection().with_context(|| "failed to connect to server")?;

    let test: Result<String, RedisError> = redis::cmd("rbac").arg(&["invalid"]).query(&mut con);
    if test.is_err() {
        assert_eq!(test.err().unwrap().to_string(), "invalid: subcommand");
    }

    // setrole
    let test: String = redis::cmd("rbac")
        .arg(&["setrole", "rolea", "set"])
        .query(&mut con)?;
    assert_eq!(test, "OK");
    let test: String = redis::cmd("rbac")
        .arg(&["setrole", "roleb", "get"])
        .query(&mut con)?;
    assert_eq!(test, "OK");
    let test: Result<String, RedisError> = redis::cmd("rbac").arg(&["setrole"]).query(&mut con);
    assert!(test.is_err());
    let test: Result<String, RedisError> = redis::cmd("rbac")
        .arg(&["setrole", "rolea"])
        .query(&mut con);
    assert!(test.is_err());

    // list / roles
    let test: Vec<String> = redis::cmd("rbac").arg(&["list"]).query(&mut con)?;
    assert_eq!(test, vec!["rolea", "set", "roleb", "get"]);
    let mut test: Vec<String> = redis::cmd("rbac").arg(&["roles"]).query(&mut con)?;
    assert_eq!(test.sort(), vec!["rolea", "roleb"].sort());

    // getrole
    let test: Vec<String> = redis::cmd("rbac")
        .arg(&["getrole", "rolea"])
        .query(&mut con)?;
    assert_eq!(test, vec!["rolea", "set"]);
    let test: Vec<String> = redis::cmd("rbac")
        .arg(&["getrole", "invalid"])
        .query(&mut con)?;
    assert_eq!(test.len(), 0);
    let test: Result<String, RedisError> = redis::cmd("rbac").arg(&["getrole"]).query(&mut con);
    assert!(test.is_err());

    // delrole
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

    Ok(())
}
