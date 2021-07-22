use boa::{
    exec::Executable, object::ObjectInitializer, parse, property::Attribute, value::Value, Context,
};
use js_sys::JSON;
use std::convert::From;
use wasm_bindgen::prelude::*;

struct JsValueRef(JsValue);

impl JsValueRef {
    fn inner(self) -> JsValue {
        self.0
    }
}

impl From<Value> for JsValueRef {
    fn from(v: Value) -> Self {
        let js_value = match v {
            Value::Null => JsValue::NULL,
            Value::Undefined => JsValue::UNDEFINED,
            Value::Boolean(b) => match b {
                false => JsValue::FALSE,
                true => JsValue::TRUE,
            },
            Value::Integer(n) => JsValue::from_f64(n as f64),
            Value::Rational(n) => JsValue::from_f64(n),
            _ => JsValue::UNDEFINED,
        };

        Self(js_value)
    }
}

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
        .map(|v| JsValueRef::from(v).inner())
}

fn parse_str(src: &str) -> JsValue {
    match unsafe { JSON::parse(src) } {
        Ok(r) => r,
        Err(_) => JsValue::from(src),
    }
}
