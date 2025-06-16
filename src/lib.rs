mod commands;
mod filters;

use std::collections::HashMap;
use std::sync::{LazyLock, RwLock};
use valkey_module::alloc::ValkeyAlloc;
use valkey_module::{Context, Status, ValkeyString, ValkeyValue, Version, valkey_module};

static MIN_VALID_SERVER_VERSION: &[i64; 3] = &[7, 2, 0];
static RBAC_ROLES: LazyLock<RwLock<HashMap<String, String>>> =
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

fn init(ctx: &Context, _args: &[ValkeyString]) -> Status {
    get_acl_categories(ctx);
    get_command_list(ctx);
    Status::Ok
}

fn get_acl_categories(ctx: &Context) {
    let ctx_call_result = ctx.call("acl", &["cat"]).unwrap();
    let act_categories = ctx_call_to_vector(ctx_call_result);
    for act_cat in act_categories {
        ACL_CATEGORIES.write().unwrap().push(act_cat);
    }
}

fn get_command_list(ctx: &Context) {
    let ctx_call_result = ctx.call("command", &["list"]).unwrap();
    let command_list = ctx_call_to_vector(ctx_call_result);
    for cmd in command_list {
        COMMAND_LIST.write().unwrap().push(cmd);
    }
}

fn ctx_call_to_vector(ctx_call_result: ValkeyValue) -> Vec<String> {
    let mut output = Vec::new();
    match ctx_call_result {
        ValkeyValue::Array(vv_array) => {
            for vk_val in vv_array {
                match vk_val {
                    ValkeyValue::SimpleString(tmp) => {
                        output.push(tmp);
                    }
                    _ => {}
                }
            }
        }
        _ => {}
    }
    output
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
