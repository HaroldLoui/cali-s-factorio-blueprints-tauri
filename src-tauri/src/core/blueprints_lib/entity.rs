use serde_json::{json, Value};

/// 实体对象
#[derive(Debug, Clone)]
pub struct Entity {
    pub entity_number: Option<i64>,
    pub name: Option<String>,
    pub entity_type: Option<String>, // type是rust的关键字
    // position: Value,       // {x: 0.0, y: 0.0}
    pub position_x: f64,
    pub position_y: f64,
    pub direction: Option<i64>,
    pub control_behavior: Value,
}

impl Entity {
    pub fn new(entity: Option<Value>) -> Self {
        let entity = entity.unwrap_or(Value::default());
        Self {
            // 实体序号
            entity_number: entity["entity_number"].as_i64(),
            // 名称
            name: entity["name"].as_str().and_then(|v| Some(v.to_string())),
            // 类型
            entity_type: entity["type"].as_str().and_then(|v| Some(v.to_string())),
            // 位置x
            position_x: entity["position"]["x"].as_f64().unwrap_or(0.0),
            // 位置y
            position_y: entity["position"]["y"].as_f64().unwrap_or(0.0),
            // 朝向
            direction: entity["direction"].as_i64(),
            // 非共有
            control_behavior: Value::Null,
        }
    }

    /// 获得该实体对象的字典格式
    pub fn get_dict(&self) -> Value {
        let mut dict = json!({
            "position": {
                "x": self.position_x,
                "y": self.position_y
            }
        });

        if let Some(entity_number) = self.entity_number {
            dict["entity_number"] = json!(entity_number);
        }
        if let Some(name) = &self.name {
            dict["name"] = json!(name);
        }
        if let Some(type1) = &self.entity_type {
            dict["type"] = json!(type1);
        }
        if let Some(direction) = self.direction {
            dict["direction"] = json!(direction);
        }
        if !self.control_behavior.is_null() {
            dict["control_behavior"] = self.control_behavior.clone();
        }

        dict
    }

    /// 用新的name或type来置换当前实体
    pub fn replace(&mut self, name: Option<String>, type1: Option<String>) {
        if name.is_some() {
            self.name = name;
        }
        if type1.is_some() {
            self.entity_type = type1;
        }
    }

    /// 旋转实体
    pub fn rotate(&mut self, direction: i64) {
        self.direction = Some(direction);
    }

    pub fn check_entity_number(&self) -> bool {
        self.entity_number.is_some_and(|x| x > 0)
    }
}
