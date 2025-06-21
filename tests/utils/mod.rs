use anyhow::Context;
use redis::Connection;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::time::Duration;

pub(crate) struct ChildGuard {
    name: &'static str,
    child: std::process::Child,
}

impl ChildGuard {
    pub(crate) fn new(name: &'static str, child: std::process::Child) -> Self {
        ChildGuard { name, child }
    }
}

impl Drop for ChildGuard {
    fn drop(&mut self) {
        if let Err(e) = self.child.kill() {
            println!("Could not kill {}: {e}", self.name);
        }
        if let Err(e) = self.child.wait() {
            println!("Could not wait for {}: {e}", self.name);
        }
    }
}
pub(crate) fn start_server() -> anyhow::Result<ChildGuard> {
    let module_path = get_module_path()?;
    let args = &[
        "--port",
        "6379",
        "--loadmodule",
        module_path.as_str(),
        "--enable-debug-command",
        "yes",
        "--enable-module-command",
        "yes",
    ];

    let server = Command::new("valkey-server")
        .args(args)
        .spawn()
        .map(|c| ChildGuard::new("server", c))?;
    Ok(server)
}

pub(crate) fn get_module_path() -> anyhow::Result<String> {
    let extension = if cfg!(target_os = "macos") {
        "dylib"
    } else {
        "so"
    };

    let profile = if cfg!(not(debug_assertions)) {
        "release"
    } else {
        "debug"
    };

    let module_path: PathBuf = [
        std::env::current_dir()?,
        PathBuf::from(format!("target/{profile}/libvalkey_rbac.{extension}")),
    ]
    .iter()
    .collect();

    assert!(
        fs::metadata(&module_path)
            .with_context(|| format!("Loading valkey module: {}", module_path.display()))?
            .is_file()
    );

    let module_path = format!("{}", module_path.display());
    Ok(module_path)
}

#[allow(dead_code)]
pub(crate) fn get_server_connection() -> anyhow::Result<Connection> {
    let client = redis::Client::open("redis://127.0.0.1:6379")?;
    loop {
        let res = client.get_connection();
        match res {
            Ok(con) => return Ok(con),
            Err(e) => {
                if e.is_connection_refusal() {
                    // server not ready yet, sleep and retry
                    std::thread::sleep(Duration::from_millis(50));
                } else {
                    return Err(e.into());
                }
            }
        }
    }
}
