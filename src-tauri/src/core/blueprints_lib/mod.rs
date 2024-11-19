#![allow(dead_code)]

use anyhow::Result;
use base64::{engine::general_purpose, Engine};
use flate2::{read::ZlibDecoder, write::ZlibEncoder, Compression};
use serde_json::Value;
use std::{fmt::Display, io::{Read, Write}};

pub mod blueprint;
pub mod combinators;
pub mod entity;
pub mod icon;

pub use entity::Entity;
pub use icon::Icon;

#[derive(Debug)]
pub struct KeyError;

impl std::error::Error for KeyError {}

impl Display for KeyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Mapping key not found.")
    }
}

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