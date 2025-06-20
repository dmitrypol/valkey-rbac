use crate::{ACL_CATEGORIES, COMMAND_LIST, MIN_VALID_SERVER_VERSION, RBAC_USER_ROLE_MAP};
use valkey_module::{Context, ValkeyValue, Version};

pub(crate) fn valid_server_version(version: Version) -> bool {
    let server_version = &[
        version.major.into(),
        version.minor.into(),
        version.patch.into(),
    ];
    server_version >= MIN_VALID_SERVER_VERSION
}

// loop through RBAC_USER_ROLE_MAP and return a vector of users for the given role
pub(crate) fn get_users_for_role(role: String) -> Vec<String> {
    let mut output = Vec::new();
    let guard = RBAC_USER_ROLE_MAP.read().unwrap();
    for (user, user_role) in guard.iter() {
        if user_role == &role {
            output.push(user.clone());
        }
    }
    output
}

// Get ACL categories from the server and store them in ACL_CATEGORIES
pub(crate) fn get_acl_categories(ctx: &Context) {
    let ctx_call_result = ctx.call("acl", &["cat"]).unwrap();
    let act_categories = ctx_call_to_vector(ctx_call_result);
    for act_cat in act_categories {
        ACL_CATEGORIES.write().unwrap().push(act_cat);
    }
}

// Get the list of commands from the server and store them in COMMAND_LIST
pub(crate) fn get_command_list(ctx: &Context) {
    let ctx_call_result = ctx.call("command", &["list"]).unwrap();
    let command_list = ctx_call_to_vector(ctx_call_result);
    for cmd in command_list {
        COMMAND_LIST.write().unwrap().push(cmd);
    }
}

// Convert the result of a context call to a vector of strings
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
