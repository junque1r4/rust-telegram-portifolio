use std::collections::HashMap;
use serde_json::{from_str, Result, Value};
use serde::Deserialize;


#[derive(Debug, Deserialize, Clone)]
pub struct JsonObject {
    name: String,
    description: String,
}

pub(crate) fn test_button_types() -> Result<Vec<JsonObject>> {
    let json_string = r#"[{
  "name": "rust",
  "description": "rust is cool"
}, {
  "name": "python",
  "description": "python is cool"
}, {
  "name": "c",
  "description": "c is cool"
}]"#;

    let data: Vec<JsonObject> = match from_str(json_string) {
        Ok(data) => data,
        Err(err) => {
            println!("Error deserializing JSON: {}", err);
            return Err(err);
        }
    };

    Ok(data)
}