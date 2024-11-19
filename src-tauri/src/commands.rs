use serde::{Deserialize, Serialize};
use tauri::State;

use crate::{
    core::{
        generate_mini_dynamic_image_blueprint, generate_mini_static_image_blueprint, generate_screen_blueprint, get_gif_duration
    }, 
    AppData
};

#[derive(Serialize, Deserialize)]
pub struct ScreenForm {
    width: u32,
    height: u32,
    #[serde(rename = "redLine")]
    red_line: bool,
    #[serde(rename = "greenLine")]
    green_line: bool,
    #[serde(rename = "keepOpen")]
    keep_open: bool,
}

/// 生成显示屏蓝图
#[tauri::command]
pub fn generate_screen_bp(form: ScreenForm, state: State<'_, AppData>) -> String {
    let mut wire_type_list = Vec::new();
    if form.red_line {
        wire_type_list.push(1);
    }
    if form.green_line {
        wire_type_list.push(2);
    }

    let result = generate_screen_blueprint(
        form.width,
        form.height,
        Some(wire_type_list),
        form.keep_open,
        state.inner(),
    );
    match result {
        Ok(res) => res,
        Err(e) => e.to_string(),
    }
}

#[derive(Serialize, Deserialize)]
pub struct ImageForm {
    width: u32,
    height: u32,
    #[serde(rename = "originalPath")]
    original_path: String,
}

/// 生成静态小图片蓝图
#[tauri::command]
pub fn generate_mini_static_image_bp(form: ImageForm, state: State<'_, AppData>) -> String {
    let result = generate_mini_static_image_blueprint(
        &form.original_path, 
        form.width, 
        form.height, 
        state.inner()
    );
    match result {
        Ok(res) => res,
        Err(e) => e.to_string(),
    }
}

/// 获取gif动图帧间间隔
#[tauri::command]
pub fn get_gif_tick(path: String) -> u32 {
    match get_gif_duration(path) {
        Ok(tick) => tick,
        Err(e) => {
            println!("{}", e.to_string());
            0
        },
    }
}

/// 生成动态态小图片蓝图
#[tauri::command]
pub fn generate_mini_dynamic_image_bp(form: ImageForm, state: State<'_, AppData>) -> String {
    let result = generate_mini_dynamic_image_blueprint(
        &form.original_path, 
        form.width, 
        form.height, 
        9,
        state.inner()
    );
    match result {
        Ok(res) => res,
        Err(e) => e.to_string(),
    }
}
