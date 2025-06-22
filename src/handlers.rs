use crate::commands;
use valkey_module::Context;
use valkey_module_macros::shutdown_event_handler;

#[shutdown_event_handler]
fn shutdown_event_handler(ctx: &Context, _event: u64) {
    ctx.log_notice("Shutting down");
    let _ = commands::save::save(ctx);
}
