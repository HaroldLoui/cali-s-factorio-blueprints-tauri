mod commands;
mod core;

use std::{fs::File, io::BufReader};

use commands::*;
use serde_json::Value;
use tauri::{path::BaseDirectory, App, Manager};

pub struct AppData {
    pub quality_list: [&'static str; 5],
    pub signal_dict: Value,
    pub signal_dict_len: usize,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(app_state)
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            generate_screen_bp,
            generate_mini_static_image_bp,
            generate_mini_dynamic_image_bp,
            get_gif_tick,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn app_state(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    let signal_json_path = app
        .path()
        .resolve("resources/signal_dict.json", BaseDirectory::Resource)
        .unwrap();
    let file = File::open(&signal_json_path).unwrap();
    let reader = BufReader::new(file);
    let dict: Value = serde_json::from_reader(reader).unwrap();
    let mut signal_dict_len = 0;
    if let Value::Object(m) = &dict {
        signal_dict_len = m.len();
    };
    app.manage(AppData {
        quality_list: ["normal", "uncommon", "rare", "epic", "legendary"],
        signal_dict: dict,
        signal_dict_len,
    });
    Ok(())
}
