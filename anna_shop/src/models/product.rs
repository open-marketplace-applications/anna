use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, PartialOrd, Copy)]
pub enum Currency {
    EUR,
}
impl Default for Currency {
    fn default() -> Self {
        Currency::EUR
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, Default, PartialEq, PartialOrd, Copy)]
pub struct Price {
    pub amount: f64,
    pub currecy: Currency,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PublishProductModel {
    body: Product,
    #[serde(rename = "type")]
    _type: String,
    version: String,
}
#[derive(Deserialize, Serialize, Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub image: String,
    pub price: Price,
}

use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/src//js/ipfs.js")]
extern "C" {
    #[wasm_bindgen]
    fn write_file() -> JsValue;
}

impl Product {
    pub fn publish(product: Product) {
        let publish_model = PublishProductModel {
            body: product,
            _type: "https//openmarketplace.org/specification/types/0.0.1/product".into(),
            version: "0.0.1".into(),
        };
        log::info!("Publish: {:?}", publish_model);
        unsafe {
            write_file();
        }
    }
}
