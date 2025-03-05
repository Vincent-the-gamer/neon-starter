use neon::{
    prelude::{Context, FunctionContext}, result::JsResult, types::JsString
};

// Echo your text in param
pub fn echo(mut cx: FunctionContext) -> JsResult<JsString> {
    // read the first param of function
    let get_str: String = cx.argument::<JsString>(0)?.value(&mut cx);
    let result: String = format!("Echoed text: {}", &get_str);
    Ok(cx.string(result))
}