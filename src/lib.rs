mod commands;
mod filters;

use std::collections::HashMap;
use std::sync::{LazyLock, RwLock};
use valkey_module::alloc::ValkeyAlloc;
use valkey_module::{Context, Status, ValkeyString, Version, valkey_module};

static MIN_VALID_SERVER_VERSION: &[i64; 3] = &[7, 2, 0];
static RBAC_ROLES: LazyLock<RwLock<HashMap<String, String>>> =
    LazyLock::new(|| RwLock::new(HashMap::new()));

pub(crate) fn valid_server_version(version: Version) -> bool {
    let server_version = &[
        version.major.into(),
        version.minor.into(),
        version.patch.into(),
    ];
    server_version >= MIN_VALID_SERVER_VERSION
}

fn preload(ctx: &Context, _args: &[ValkeyString]) -> Status {
    let ver = ctx.get_server_version().expect("can't get_server_version");
    if !valid_server_version(ver) {
        ctx.log_notice(format!("min valid server version {:?}", MIN_VALID_SERVER_VERSION).as_str());
        Status::Err
    } else {
        Status::Ok
    }
}

fn init(_ctx: &Context, _args: &[ValkeyString]) -> Status {
    Status::Ok
}

valkey_module! {
    name: "valkey-rbac",
    version: 1,
    allocator: (ValkeyAlloc, ValkeyAlloc),
    data_types: [],
    preload: preload,
    init: init,
    commands: [
        ["rbac",commands::rbac, "", 0, 0, 0],
    ],
    filters: [
    ]
}
