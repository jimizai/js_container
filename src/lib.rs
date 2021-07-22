use boa::{
    exec::Executable, object::ObjectInitializer, parse, property::Attribute, value::Value, Context,
};
use js_sys::JSON;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn evaluate(src: &str) -> Result<JsValue, JsValue> {
    // Setup executor
    let mut context = Context::new();
    let window_object = ObjectInitializer::new(&mut context)
        .property("x", 0, Attribute::all())
        .property("y", 1, Attribute::all())
        .build();

    context.register_global_property("window", window_object, Attribute::all());

    let expr = match parse(src, false) {
        Ok(res) => res,
        Err(e) => {
            return Err(format!(
                "Uncaught {}",
                context
                    .throw_syntax_error(e.to_string())
                    .expect_err("interpreter.throw_syntax_error() did not return an error")
                    .display()
            )
            .into());
        }
    };
    expr.run(&mut context)
        .map_err(|e| JsValue::from(format!("Uncaught {}", e.display())))
        .map(|v| parse_str(&v.display().to_string()))
}

fn parse_str(src: &str) -> JsValue {
    match unsafe { JSON::parse(src) } {
        Ok(r) => r,
        Err(_) => JsValue::from(src),
    }
}
