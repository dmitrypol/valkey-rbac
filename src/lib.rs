mod commands;
mod filters;
mod utils;

use crate::utils::*;
use std::collections::HashMap;
use std::sync::{LazyLock, RwLock};
use valkey_module::alloc::ValkeyAlloc;
use valkey_module::{Context, Status, ValkeyString, valkey_module};

static MIN_VALID_SERVER_VERSION: &[i64; 3] = &[7, 2, 0];
// TODO should RBAC_ROLES be HashMap<String, String>, HashMap<String, Vec<String>> or HashMap<String, HashSet<String>> (to enforce uniqueness)?
static RBAC_ROLES: LazyLock<RwLock<HashMap<String, String>>> =
    LazyLock::new(|| RwLock::new(HashMap::new()));
static RBAC_USER_ROLE_MAP: LazyLock<RwLock<HashMap<String, String>>> =
    LazyLock::new(|| RwLock::new(HashMap::new()));
static ACL_CATEGORIES: LazyLock<RwLock<Vec<String>>> = LazyLock::new(|| RwLock::new(Vec::new()));
static COMMAND_LIST: LazyLock<RwLock<Vec<String>>> = LazyLock::new(|| RwLock::new(Vec::new()));
static ACL_FLAGS: [&str; 7] = [
    "allcommands",
    "nocommands",
    "allkeys",
    "resetkeys",
    "allchannels",
    "resetchannels",
    "reset",
];

fn preload(ctx: &Context, _args: &[ValkeyString]) -> Status {
    let ver = ctx.get_server_version().expect("can't get_server_version");
    if !valid_server_version(ver) {
        ctx.log_notice(format!("min valid server version {:?}", MIN_VALID_SERVER_VERSION).as_str());
        Status::Err
    } else {
        Status::Ok
    }
}

fn init(ctx: &Context, _args: &[ValkeyString]) -> Status {
    get_acl_categories(ctx);
    get_command_list(ctx);
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
