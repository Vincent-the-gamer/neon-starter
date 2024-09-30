use neon::{prelude::ModuleContext, result::NeonResult};

mod string;

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    let _ = cx.export_function("echo", string::echo);
    Ok(())
}
