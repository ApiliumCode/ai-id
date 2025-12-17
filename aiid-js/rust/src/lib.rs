use wasm_bindgen::prelude::*;

// Use `lol_alloc` as the global allocator on wasm32
#[cfg(target_arch = "wasm32")]
use lol_alloc::{AssumeSingleThreaded, FreeListAllocator};

#[cfg(target_arch = "wasm32")]
#[global_allocator]
static ALLOC: AssumeSingleThreaded<FreeListAllocator> =
    unsafe { AssumeSingleThreaded::new(FreeListAllocator::new()) };

pub type JsResult<T> = Result<T, JsValue>;

macro_rules! jserr {
    ($code:expr) => {
        match $code {
            Ok(v) => Ok(v),
            Err(e) => Err(JsValue::from_str(&format!("{:?}", e))),
        }
    };
}

#[wasm_bindgen]
pub struct Encoding(aingle_id::AiidEncoding);

#[wasm_bindgen]
impl Encoding {
    #[wasm_bindgen(constructor)]
    pub fn new(encoding_name: &str) -> JsResult<Encoding> {
        Ok(Encoding(jserr!(aingle_id::AiidEncoding::with_kind(encoding_name))?))
    }

    pub fn encode(&self, data: &[u8]) -> JsResult<String> {
        jserr!(self.0.encode(data))
    }

    pub fn decode(&self, data: &str) -> JsResult<Vec<u8>> {
        jserr!(self.0.decode(data))
    }

    pub fn is_corrupt(&self, data: &str) -> JsResult<bool> {
        jserr!(self.0.is_corrupt(data))
    }
}
