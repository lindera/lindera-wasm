use std::str::FromStr;

use serde_json::Value;
use wasm_bindgen::prelude::*;

use lindera::mode::Mode;
use lindera::token::Token;
use lindera::tokenizer::{
    Tokenizer as LinderaTokenizer, TokenizerBuilder as LinderaTokenizerBuilder,
};

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[wasm_bindgen(js_name = "getVersion")]
pub fn get_version() -> String {
    VERSION.to_string()
}

// Convert snake_case to camelCase
fn to_camel_case(s: &str) -> String {
    let mut result = String::new();
    let mut capitalize_next = false;

    for c in s.chars() {
        if c == '_' {
            capitalize_next = true;
        } else if capitalize_next {
            result.push(c.to_ascii_uppercase());
            capitalize_next = false;
        } else {
            result.push(c);
        }
    }

    result
}

// Convert Value to JsValue recursively
fn value_to_js(value: Value) -> Result<JsValue, JsValue> {
    match value {
        Value::String(s) => Ok(JsValue::from_str(s.as_str())),
        Value::Number(n) => {
            if let Some(i) = n.as_u64() {
                Ok(JsValue::from_f64(i as f64))
            } else if let Some(i) = n.as_i64() {
                Ok(JsValue::from_f64(i as f64))
            } else if let Some(f) = n.as_f64() {
                Ok(JsValue::from_f64(f))
            } else {
                Ok(JsValue::from_str(&n.to_string()))
            }
        }
        Value::Bool(b) => Ok(JsValue::from_bool(b)),
        Value::Null => Ok(JsValue::NULL),
        Value::Array(arr) => {
            let js_arr = js_sys::Array::new();
            for item in arr {
                js_arr.push(&value_to_js(item)?);
            }
            Ok(js_arr.into())
        }
        Value::Object(map) => {
            let js_obj = js_sys::Object::new();
            for (key, val) in map {
                // Change key to camel case
                let js_key = JsValue::from_str(to_camel_case(&key).as_str());
                let js_val = value_to_js(val)?;
                js_sys::Reflect::set(&js_obj, &js_key, &js_val)
                    .map_err(|e| JsValue::from_str(&format!("Failed to set property: {e:?}")))?;
            }
            Ok(js_obj.into())
        }
    }
}

// Convert Vec<Token> to JsValue (Array of Objects)
fn convert_to_js_objects(tokens: Vec<Token>) -> Result<JsValue, JsValue> {
    let js_array = js_sys::Array::new();
    for mut token in tokens {
        js_array.push(&value_to_js(token.as_value())?);
    }

    Ok(js_array.into())
}

#[wasm_bindgen]
pub struct TokenizerBuilder {
    inner: LinderaTokenizerBuilder,
}

#[wasm_bindgen]
impl TokenizerBuilder {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<Self, JsValue> {
        let inner =
            LinderaTokenizerBuilder::new().map_err(|e| JsValue::from_str(&e.to_string()))?;

        Ok(Self { inner })
    }

    pub fn build(self) -> Result<Tokenizer, JsValue> {
        let inner = self
            .inner
            .build()
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        Ok(Tokenizer { inner })
    }

    #[wasm_bindgen(js_name = "setMode")]
    pub fn set_mode(&mut self, mode: &str) -> Result<(), JsValue> {
        let m = Mode::from_str(mode).map_err(|e| JsValue::from_str(&e.to_string()))?;
        self.inner.set_segmenter_mode(&m);

        Ok(())
    }

    #[wasm_bindgen(js_name = "setDictionary")]
    pub fn set_dictionary(&mut self, uri: &str) -> Result<(), JsValue> {
        self.inner.set_segmenter_dictionary(uri);

        Ok(())
    }

    #[wasm_bindgen(js_name = "setUserDictionary")]
    pub fn set_user_dictionary(&mut self, uri: &str) -> Result<(), JsValue> {
        self.inner.set_segmenter_user_dictionary(uri);

        Ok(())
    }

    #[wasm_bindgen(js_name = "appendCharacterFilter")]
    pub fn append_character_filter(&mut self, name: &str, args: JsValue) -> Result<(), JsValue> {
        let a = serde_wasm_bindgen::from_value::<Value>(args)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        self.inner.append_character_filter(name, &a);

        Ok(())
    }

    #[wasm_bindgen(js_name = "appendTokenFilter")]
    pub fn append_token_filter(&mut self, name: &str, args: JsValue) -> Result<(), JsValue> {
        let a = serde_wasm_bindgen::from_value::<Value>(args)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        self.inner.append_token_filter(name, &a);

        Ok(())
    }
}

#[wasm_bindgen]
pub struct Tokenizer {
    inner: LinderaTokenizer,
}

#[wasm_bindgen]
impl Tokenizer {
    pub fn tokenize(&self, input_text: &str) -> Result<JsValue, JsValue> {
        let tokens = self
            .inner
            .tokenize(input_text)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        let js_value = convert_to_js_objects(tokens)?;

        Ok(js_value)
    }
}

#[cfg(test)]
mod tests {
    #[cfg(target_arch = "wasm32")]
    use wasm_bindgen_test::wasm_bindgen_test;

    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen_test]
    fn test_tokenize() {
        use crate::TokenizerBuilder;
        use serde_json::Value;

        let mut builder = TokenizerBuilder::new().unwrap();
        builder.set_mode("normal").unwrap();
        builder.set_dictionary("embedded://ipadic").unwrap();

        let tokenizer = builder.build().unwrap();

        let t = tokenizer.tokenize("関西国際空港限定トートバッグ").unwrap();
        let tokens: Vec<Value> = serde_wasm_bindgen::from_value(t).unwrap();

        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[0].get("surface").unwrap(), "関西国際空港");
        assert_eq!(tokens[1].get("surface").unwrap(), "限定");
        assert_eq!(tokens[2].get("surface").unwrap(), "トートバッグ");
    }

    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen_test]
    fn test_camel_case() {
        use crate::to_camel_case;

        assert_eq!(to_camel_case("a"), "a");
        assert_eq!(to_camel_case("a_b"), "aB");
        assert_eq!(to_camel_case("a_b_c"), "aBC");
        assert_eq!(to_camel_case("a_b_c_d"), "aBCD");
        assert_eq!(to_camel_case("a_b_c_d_e"), "aBCDE");
    }
}
