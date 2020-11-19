use neon::prelude::*;

fn hello(mut cx: FunctionContext) -> JsResult<JsString> {
    Ok(cx.string("Hello, World!"))
}

#[neon::init]
fn init(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("hello", hello)?;

    Ok(())
}
