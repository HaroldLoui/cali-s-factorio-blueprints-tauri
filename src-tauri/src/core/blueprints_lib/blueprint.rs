use serde_json::{json, Value};

use super::{Icon, KeyError};

/// 蓝图对象
#[derive(Debug)]
pub struct BluePrint {
    entities: Vec<Value>,
    icons: Vec<Icon>,
    label: Option<String>,
    description: Option<String>,
    version: Option<i64>,
    // 线缆数据
    wires: Value,
}

impl BluePrint {
    pub fn new(blueprint_dict: Option<Value>) -> Self {
        let blueprint_dict = blueprint_dict.unwrap_or(Value::default());
        let blueprint = &blueprint_dict["blueprint"];

        let mut entities = Vec::new();
        if let Some(entity_values) = blueprint["entities"].as_array() {
            for ele in entity_values {
                entities.push(ele.clone());
            }
        }

        let mut icons = Vec::new();
        if let Some(icon_values) = blueprint["icons"].as_array() {
            for ic in icon_values {
                icons.push(Icon::new(ic));
            }
        }

        Self {
            entities,
            icons,
            label: blueprint["label"].as_str().and_then(|v| Some(v.to_string())),
            description: blueprint["description"].as_str().and_then(|v| Some(v.to_string())),
            version: blueprint["version"].as_i64(),
            wires: blueprint["wires"].clone()
        }
    }

    /// 获得该蓝图对象的字典形式
    pub fn get_dict(&self) -> Value {
        let mut dict = json!({"item": "blueprint"});

        if self.entities.len() > 0 {
            // let enetities: Vec<Value> = self.entities.iter().map(|ele| ele.get_dict()).collect();
            dict["entities"] = json!(self.entities.clone());
        }
        if self.icons.len() > 0 {
            let icons: Vec<Value> = self.icons.iter().map(|ele| ele.get_dict()).collect();
            dict["icons"] = json!(icons);
        }
        if let Some(label) = &self.label {
            dict["label"] = json!(label);
        }
        if let Some(description) = &self.description {
            dict["description"] = json!(description);
        }
        if let Some(version) = self.version {
            dict["version"] = json!(version);
        }
        dict["wires"] = self.wires.clone();

        json!({"blueprint": dict})
    }

    /// 获取实体总数
    pub fn get_entities_number(&self) -> usize {
        self.entities.len()
    }

    pub fn add_entity(&mut self, entity: &mut Value, entity_number: i64) {
        if entity_number > 0 {
            entity["entity_number"] = json!(self.entities.len() + 1);
        }
        self.entities.push(entity.clone());
    }

    pub fn connect_entity(
        &mut self, 
        first_entity: &Value,
        second_entity: &Value,
        connect_code: String,
        wire_type: String
    ) -> Result<(), KeyError> {
        if !first_entity.check_entity_number() || !second_entity.check_entity_number() {
            return Err(KeyError);
        }

        if self.wires.is_null() {
            self.wires = json!([]);
        }

        let num1 = first_entity["entity_number"].as_i64().unwrap();
        let num2 = second_entity["entity_number"].as_i64().unwrap();
        let wires = self.wires.as_array_mut().unwrap();
        match connect_code.as_str() {
            "ii" => {
                if wire_type.contains('r') {
                    wires.push(json!([num1, 1, num2, 1]));
                }
                if wire_type.contains('g') {
                    wires.push(json!([num1, 2, num2, 2]));
                }
            },
            "io" => {
                if wire_type.contains('r') {
                    wires.push(json!([num1, 1, num2, 3]));
                }
                if wire_type.contains('g') {
                    wires.push(json!([num1, 2, num2, 4]));
                }
            },
            "oi" => {
                if wire_type.contains('r') {
                    wires.push(json!([num1, 3, num2, 1]));
                }
                if wire_type.contains('g') {
                    wires.push(json!([num1, 4, num2, 2]));
                }
            },
            "oo" => {
                if wire_type.contains('r') {
                    wires.push(json!([num1, 3, num2, 3]));
                }
                if wire_type.contains('g') {
                    wires.push(json!([num1, 4, num2, 4]));
                }
            },
            _ => return Err(KeyError),
        }

        Ok(())
    }
}

trait CheckEntityNumber {
    fn check_entity_number(&self) -> bool;
}

impl CheckEntityNumber for Value {
    fn check_entity_number(&self) -> bool {
        let entity_number = &self["entity_number"];
        if entity_number.is_null() {
            return false;
        }
        entity_number.as_i64().is_some_and(|x| x > 0)
    }
}