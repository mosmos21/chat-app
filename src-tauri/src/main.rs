// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use chatgpt::prelude::*;
use std::env;

#[tauri::command]
async fn exec_prompt(prompt: String) -> String {
    let client = ChatGPT::new(env::var("API_KEY").unwrap()).unwrap();
    let response = client.send_message(prompt).await.unwrap();

    response.message().content.clone()
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![exec_prompt])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
