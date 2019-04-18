#[macro_use]
extern crate neon;

use neon::prelude::*;
use std::{thread, time};

fn wait(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let arg = cx.argument::<JsNumber>(0)?;
    if arg.is_a::<JsNumber>() {
        thread::sleep(time::Duration::from_millis(arg.value() as u64));
    }
    return Ok(cx.undefined());
}

register_module!(mut cx, { cx.export_function("wait", wait) });
