use std::path::Path;
use std::str::FromStr;

use serde_json::Value;
use wasm_bindgen::prelude::*;

use lindera::dictionary::DictionaryKind;
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

fn token_to_json(token: &mut Token) -> Value {
    serde_json::json!({
        "text": token.text,
        "details": token.details().clone(),
        "byte_start": token.byte_start,
        "byte_end": token.byte_end,
        "word_id": token.word_id,
    })
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

    #[wasm_bindgen(js_name = "setDictionaryKind")]
    pub fn set_dictionary_kind(&mut self, kind: &str) -> Result<(), JsValue> {
        let k = DictionaryKind::from_str(kind).map_err(|e| JsValue::from_str(&e.to_string()))?;
        self.inner.set_segmenter_dictionary_kind(&k);

        Ok(())
    }

    #[wasm_bindgen(js_name = "setDictionaryPath")]
    pub fn set_dictionary_path(&mut self, path: &str) -> Result<(), JsValue> {
        self.inner.set_segmenter_dictionary_path(Path::new(path));

        Ok(())
    }

    #[wasm_bindgen(js_name = "setUserDictionaryPath")]
    pub fn set_user_dictionary_path(&mut self, path: &str) -> Result<(), JsValue> {
        self.inner
            .set_segmenter_user_dictionary_path(Path::new(path));

        Ok(())
    }

    #[wasm_bindgen(js_name = "setUserDictionaryKind")]
    pub fn set_user_dictionary_kind(&mut self, kind: &str) -> Result<(), JsValue> {
        let k = DictionaryKind::from_str(kind).map_err(|e| JsValue::from_str(&e.to_string()))?;
        self.inner.set_segmenter_user_dictionary_kind(&k);

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
        let mut tokens = self
            .inner
            .tokenize(input_text)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        let js_value = serde_wasm_bindgen::to_value(
            &tokens
                .iter_mut()
                .map(|token| token_to_json(token))
                .collect::<Vec<_>>(),
        )
        .map_err(|e| JsValue::from_str(&e.to_string()))?;

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
        builder.set_dictionary_kind("ipadic").unwrap();

        let tokenizer = builder.build().unwrap();

        let t = tokenizer.tokenize("関西国際空港限定トートバッグ").unwrap();
        let tokens: Vec<Value> = serde_wasm_bindgen::from_value(t).unwrap();

        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[0].get("text").unwrap(), "関西国際空港");
        assert_eq!(tokens[1].get("text").unwrap(), "限定");
        assert_eq!(tokens[2].get("text").unwrap(), "トートバッグ");
    }
}
