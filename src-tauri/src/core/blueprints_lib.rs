#![allow(dead_code)]

use anyhow::Result;
use base64::{engine::general_purpose, Engine};
use flate2::{read::ZlibDecoder, write::ZlibEncoder, Compression};
use serde_json::{json, Value};
use std::io::{Read, Write};

pub fn dict_to_blueprint(blueprint_dict: &Value) -> Result<String> {
    // 将蓝图数据转换为 JSON 字符串
    let json_data = serde_json::to_string(blueprint_dict)?;
    // 压缩 JSON 数据
    let mut e = ZlibEncoder::new(Vec::new(), Compression::default());
    e.write_all(json_data.as_bytes())?;
    let compressed_data = e.finish()?;
    // 编码为 Base64
    let base64_data = general_purpose::STANDARD.encode(&compressed_data);
    // 添加蓝图前缀
    let blueprint_string = format!("0{}", base64_data);
    Ok(blueprint_string)
}

pub fn blueprint_to_dict(blueprint_string: &str) -> Result<Value> {
    let blueprint_string = if blueprint_string.starts_with("0") {
        &blueprint_string[1..]
    } else {
        blueprint_string
    };
    // Base64 解码
    let compressed_data = general_purpose::STANDARD.decode(blueprint_string)?;
    // 解压缩数据
    let mut d = ZlibDecoder::new(&compressed_data[..]);
    let mut json_data = String::new();
    d.read_to_string(&mut json_data)?;
    // 反序列化为 JSON
    let blueprint_data: Value = serde_json::from_str(&json_data)?;
    Ok(blueprint_data)
}

/// 实体对象
#[derive(Debug)]
pub struct Entity {
    entity_number: Option<i64>,
    name: Option<String>,
    entity_type: Option<String>, // type是rust的关键字
    position: Value,       // {x: 0.0, y: 0.0}
    direction: Option<i64>,
    control_behavior: Value,
}

impl Entity {
    pub fn new(entity: &Value) -> Self {
        Self {
            entity_number: entity["entity_number"].as_i64(),
            name: entity["name"].as_str().and_then(|v| Some(v.to_string())),
            entity_type: entity["type"].as_str().and_then(|v| Some(v.to_string())),
            position: entity["position"].clone(),
            direction: entity["direction"].as_i64(),
            control_behavior: entity["control_behavior"].clone(),
        }
    }

    /// 获得该实体对象的字典格式
    pub fn get_dict(&self) -> Value {
        let mut dict = json!({});

        if let Some(entity_number) = self.entity_number {
            dict["entity_number"] = json!(entity_number);
        }
        if let Some(name) = &self.name {
            dict["name"] = json!(name);
        }
        if let Some(type1) = &self.entity_type {
            dict["type"] = json!(type1);
        }
        if !self.position.is_null() {
            dict["position"] = self.position.clone();
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
}

/// 蓝图对象
#[derive(Debug)]
pub struct BluePrint {
    entities: Vec<Entity>,
    icons: Vec<Value>,
    label: Option<String>,
    version: Option<i64>,
}

impl BluePrint {
    pub fn new(blueprint_dict: &Value) -> Self {
        let blueprint = &blueprint_dict["blueprint"];

        let mut entities = Vec::new();
        if let Some(entity_values) = blueprint["entities"].as_array() {
            for ele in entity_values {
                entities.push(Entity::new(ele));
            }
        }

        let mut icons = Vec::new();
        if let Some(ic) = blueprint["icons"].as_array() {
            icons = ic.clone();
        }

        Self {
            entities,
            icons,
            label: blueprint["label"]
                .as_str()
                .and_then(|v| Some(v.to_string())),
            version: blueprint["version"].as_i64(),
        }
    }

    /// 获得该蓝图对象的字典形式
    pub fn get_dict(&self) -> Value {
        let mut dict = json!({});

        if self.entities.len() > 0 {
            let enetities: Vec<Value> = self.entities.iter().map(|ele| ele.get_dict()).collect();
            dict["entities"] = json!(enetities);
        }
        if self.icons.len() > 0 {
            dict["icons"] = json!(self.icons);
        }
        if let Some(label) = &self.label {
            dict["label"] = json!(label);
        }
        if let Some(version) = self.version {
            dict["version"] = json!(version);
        }

        json!({"blueprint": dict})
    }
}
