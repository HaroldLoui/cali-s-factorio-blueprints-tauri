use serde_json::json;
#[allow(dead_code)]

use serde_json::Value;

use crate::core::blueprints_lib::Entity;

/// 算术运算器
pub struct ArithmeticCombinator {
    pe: Entity,
    name: String,
    control_behavior: Value,
}

impl ArithmeticCombinator {
    
    pub fn new(entity: Option<Value>) -> Self {
        let entity = entity.unwrap_or(Value::default());
        let mut combinator = ArithmeticCombinator {
            pe: Entity::new(Some(entity.clone())),
            name: "arithmetic-combinator".to_string(),
            control_behavior: entity["control_behavior"].clone(),
        };
        if combinator.control_behavior.is_null() {
            combinator.init_control_behavior();
        }
        combinator
    }

    /// 初始化行为
    pub fn init_control_behavior(&mut self) {
        self.control_behavior = json!({"arithmetic_conditions": {}});
    }

    /// 设置第一信号
    pub fn set_first_signal(&mut self, name: String, type1: String) {
        self.control_behavior["arithmetic_conditions"]["first_signal"] = json!({
            "name": name, "type": type1
        });
    }

    /// 设置第二信号
    pub fn set_second_signal(&mut self, name: String, type1: String) {
        self.control_behavior["arithmetic_conditions"]["second_signal"] = json!({
            "name": name, "type": type1
        });
    }

    /// 设置输出信号
    pub fn set_output_signal(&mut self, name: String, type1: String) {
        self.control_behavior["arithmetic_conditions"]["output_signal"] = json!({
            "name": name, "type": type1
        });
    }

    /// 设置运算符，operation默认‘*’
    pub fn set_operation(&mut self, operation: String) {
        self.control_behavior["arithmetic_conditions"]["operation"] = json!(operation);
    }

    /// 设置第一常量
    pub fn set_first_constant(&mut self, constant: i64) {
        self.control_behavior["arithmetic_conditions"]["first_constant"] = json!(constant);
    }

    /// 设置第二常量
    pub fn set_second_constant(&mut self, constant: i64) {
        self.control_behavior["arithmetic_conditions"]["second_constant"] = json!(constant);
    }

    pub fn entity(&self) -> Value {
        let mut entity = self.pe.get_dict();
        entity["name"] = json!(self.name);
        entity["control_behavior"] = json!(self.control_behavior);
        entity
    }
}