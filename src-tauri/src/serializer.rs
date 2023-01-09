use serde_json::{self, Value};

#[tauri::command]
pub fn serialize_raw_string(input: &str) -> Value {
    let value: Value = match serde_json::from_str(input) {
        Ok(v) => v,
        Err(_) => Value::Null
    };
    value
}


