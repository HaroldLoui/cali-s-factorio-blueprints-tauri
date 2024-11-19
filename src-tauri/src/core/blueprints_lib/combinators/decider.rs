use serde_json::{json, Value};

use crate::core::blueprints_lib::Entity;

/// 判断运算器
#[derive(Debug)]
pub struct DeciderCombinator {
    pe: Entity,
    name: String,
    control_behavior: Value,
    conditions: Value,
    outputs: Value,
}

/// 添加条件参数
pub struct AddConditionParams {
    pub comparator: String,
    pub constant: i64,
    pub first_signal_name: String,
    pub first_signal_type: String,
    pub first_use_red_network: bool,
    pub first_use_green_network: bool,
    pub second_signal_name: String,
    pub second_signal_type: String,
    pub second_use_red_network: bool,
    pub second_use_green_network: bool,
}

impl Default for AddConditionParams {
    fn default() -> Self {
        Self {
            comparator: "<".to_string(),
            constant: 0,
            first_signal_name: "".to_string(),
            first_signal_type: "".to_string(),
            first_use_red_network: true,
            first_use_green_network: true,
            second_signal_name: "".to_string(),
            second_signal_type: "".to_string(),
            second_use_red_network: true,
            second_use_green_network: true,
        }
    }
}

/// 添加输出参数
pub struct AddOutPutParams {
    pub signal_name: String,
    pub signal_type: String,
    pub copy_count_from_input: bool,
    pub use_red_network: bool,
    pub use_green_network: bool,
}

impl Default for AddOutPutParams {
    fn default() -> Self {
        Self {
            signal_name: "".to_string(),
            signal_type: "".to_string(),
            copy_count_from_input: true,
            use_red_network: true,
            use_green_network: true,
        }
    }
}

impl DeciderCombinator {
    
    pub fn new(entity: Option<Value>) -> Self {
        let entity = entity.unwrap_or(Value::default());
        let mut combinator = DeciderCombinator {
            pe: Entity::new(Some(entity.clone())),
            name: "constant-combinator".to_string(),
            control_behavior: entity["control_behavior"].clone(),
            conditions: Value::default(),
            outputs: Value::default(),
        };
        if combinator.control_behavior.is_null() {
            combinator.init_control_behavior();
        }
        combinator
    }

    pub fn init_control_behavior(&mut self) {
        self.control_behavior = json!({
            "decider_conditions": {
                "conditions": [],
                "outputs": [],
            }
        });
        self.conditions = json!([]);
        self.outputs = json!([]);
    }

    /// 添加条件
    pub fn add_condition(&mut self, params: AddConditionParams) {
        let AddConditionParams {
            comparator,
            constant,
            first_signal_name,
            first_signal_type,
            first_use_red_network,
            first_use_green_network,
            second_signal_name,
            second_signal_type,
            second_use_red_network,
            second_use_green_network,
        } = params;

        let mut item = json!({
            "first_signal_networks": {
                "red": first_use_red_network,
                "green": first_use_green_network
            },
            "second_signal_networks": {
                "red": second_use_red_network,
                "green": second_use_green_network
            },
        });
        if !comparator.is_empty() {
            item["comparator"] = json!(comparator);
        }
        if constant > 0 {
            item["constant"] = json!(constant);
        }
        if !first_signal_name.is_empty() {
            item["first_signal"] = json!({"name": first_signal_name});
            if !first_signal_type.is_empty() {
                item["first_signal"]["type"] = json!(first_signal_type);
            }
        }
        if !second_signal_name.is_empty() {
            item["second_signal"] = json!({"name": second_signal_name});
            if !second_signal_type.is_empty() {
                item["second_signal"]["type"] = json!(second_signal_type);
            }
        }
        let conditions = self.conditions.as_array_mut().unwrap();
        conditions.push(item);
    }

    /// 添加输出
    pub fn add_output(&mut self, params: AddOutPutParams) {
        let AddOutPutParams { 
            signal_name, 
            signal_type, 
            copy_count_from_input, 
            use_red_network, 
            use_green_network 
        } = params;

        let mut item = json!({
            "copy_count_from_input": copy_count_from_input,
            "networks": {
                "green": use_green_network,
                "red": use_red_network
            }
        });

        if !signal_name.is_empty() {
            item["signal"] = json!({"name": signal_name});
            if !signal_type.is_empty() {
                item["signal"]["type"] = json!(signal_type);
            }
        }
        let outputs = self.outputs.as_array_mut().unwrap();
        outputs.push(item);
    }

    pub fn entity(&self) -> Value {
        let mut entity = self.pe.get_dict();
        entity["name"] = json!(self.name);
        entity["control_behavior"] = json!(self.control_behavior);
        entity["conditions"] = json!(self.conditions);
        entity["outputs"] = json!(self.outputs);
        entity
    }
}
