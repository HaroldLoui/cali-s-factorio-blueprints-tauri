use serde_json::{json, Value};

use crate::AppData;
use crate::core::blueprints_lib::Entity;

/// 常量运算器
#[derive(Debug)]
pub struct ConstantCombinator {
    pe: Entity, // parent_entity，rust没有继承这一概念
    pub name: String,
    pub control_behavior: Value, // 控制行为
    pub filter_count: i64,       // 总过滤器数量
}

impl ConstantCombinator {
    pub fn new(entity: Option<Value>) -> Self {
        let entity = entity.unwrap_or(Value::default());
        let mut combinator = ConstantCombinator {
            pe: Entity::new(Some(entity.clone())),
            name: "constant-combinator".to_string(),
            control_behavior: entity["control_behavior"].clone(),
            filter_count: 0,
        };
        if combinator.control_behavior.is_null() {
            combinator.init_control_behavior();
        }
        combinator
    }

    /// 初始化行为
    pub fn init_control_behavior(&mut self) {
        self.control_behavior = json!({
            "sections": {
                "sections": []
            }
        });
    }

    /// 手动设置过滤器
    pub fn set_filter(
        &mut self,
        section_index: i64,
        filter_index: i64,
        name: String,
        type1: String,
        count: i64,
        quality: String,
    ) {
        let mut has_section = false;
        let sections = self.control_behavior["sections"]["sections"].as_array_mut().unwrap();
        for sec in sections.into_iter() {
            if sec["index"].as_i64() == Some(section_index) {
                has_section = true;
                break;
            }
        }
        if !has_section {
            sections.push(json!({
                "filters": [],
                "index": section_index
            }));
        }

        for sec in sections {
            if sec["index"].as_i64() == Some(section_index) {
                let filters = sec["filters"].as_array_mut().unwrap();
                filters.push(json!({
                    "comparator": "=",
                    "count": count,
                    "index": filter_index,
                    "name": name,
                    "type": type1,
                    "quality": quality,
                }));
            }
        }
    }

    /// 自动根据序号添加过滤器，主要用于显示屏存储大量数据
    pub fn add_filter_auto(&mut self, count: Option<u32>, data: &AppData) {
        let count = count.unwrap_or(1);

        let this_filter_global_index = self.filter_count;
        let this_filter_local_index = this_filter_global_index % 1000 + 1;
        let this_filter_section_index = this_filter_local_index / 1000 + 1;

        let sections = self.control_behavior["sections"]["sections"].as_array_mut().unwrap();
        if (sections.len() as i64) < this_filter_section_index {
            sections.push(json!({"filters": [], "index": this_filter_section_index}));
        }

        let signal_dict = &data.signal_dict;
        let quality_list = &data.quality_list;

        let mut signal = json!({
            "comparator": "=",
            "count": count,
            "index": this_filter_local_index,
            "name": signal_dict[(this_filter_global_index / 5).to_string()]["name"],
            "quality": quality_list[(this_filter_global_index % 5) as usize],
        });

        if !signal_dict[(this_filter_global_index / 5).to_string()]["type"].is_null() {
            signal["type"] = signal_dict[(this_filter_global_index / 5).to_string()]["type"].clone();
        }
        let filters = sections[(this_filter_section_index - 1) as usize]["filters"].as_array_mut().unwrap();
        filters.push(signal);

        self.filter_count += 1;
    }

    pub fn entity(&self) -> Value {
        let mut entity = self.pe.get_dict();
        entity["name"] = json!(self.name);
        entity["control_behavior"] = json!(self.control_behavior);
        entity["filter_count"] = json!(self.filter_count);
        entity
    }
}
