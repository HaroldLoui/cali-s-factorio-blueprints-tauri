use serde_json::{json, Value};

/// 图标对象
#[derive(Debug)]
pub struct Icon {
    index: i64,
    signal: Value,
}

impl Icon {

    pub fn new(icon: &Value) -> Self {
        Self { 
            index: icon["index"].as_i64().unwrap_or(0), 
            signal: json!({
                "name": icon["signal"]["name"],
                "type": icon["signal"]["type"],
            }),
        }
    }

    pub fn get_dict(&self) -> Value {
        let mut dict = json!({
            "index": self.index,
            "signal": {},
        });
        if !self.signal["name"].is_null() {
            dict["signal"]["name"] = self.signal["name"].clone();
        }
        if !self.signal["type"].is_null() {
            dict["signal"]["type"] = self.signal["type"].clone();
        }

        dict
    }
}