#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use ardin_core::{Event, Record, Store};
use std::sync::Mutex;
use tauri::{Manager, State};
struct App(Mutex<Store>);
#[tauri::command]
fn read_state(state: State<App>) -> Result<serde_json::Value, String> {
    let db = state.0.lock().map_err(|e| e.to_string())?;
    db.state()
}
#[tauri::command]
fn save_event(state: State<App>, event: Event) -> Result<(), String> {
    state.0.lock().map_err(|e| e.to_string())?.save_event(event)
}
#[tauri::command]
fn delete_event(state: State<App>, id: i64) -> Result<(), String> {
    state.0.lock().map_err(|e| e.to_string())?.delete_event(id)
}
#[tauri::command]
fn save_stock(state: State<App>, code: String, name: String, sector: String) -> Result<(), String> {
    state
        .0
        .lock()
        .map_err(|e| e.to_string())?
        .stock(&code, &name, &sector)
}
#[tauri::command]
fn save_record(state: State<App>, record: Record) -> Result<(), String> {
    state
        .0
        .lock()
        .map_err(|e| e.to_string())?
        .save_record(record)
}
#[tauri::command]
fn backup(state: State<App>, path: String) -> Result<(), String> {
    state
        .0
        .lock()
        .map_err(|e| e.to_string())?
        .backup(std::path::Path::new(&path))
}
#[tauri::command]
fn restore(state: State<App>, path: String, safety: String) -> Result<(), String> {
    state
        .0
        .lock()
        .map_err(|e| e.to_string())?
        .restore(std::path::Path::new(&path), std::path::Path::new(&safety))
}
#[tauri::command]
fn receive_dividend(state: State<App>, id: i64) -> Result<(), String> {
    state
        .0
        .lock()
        .map_err(|e| e.to_string())?
        .receive_dividend(id)
}
fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let dir = app.path().app_data_dir()?;
            std::fs::create_dir_all(&dir)?;
            let store = Store::open(&dir.join("ardin-saham.db")).map_err(std::io::Error::other)?;
            app.manage(App(Mutex::new(store)));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            read_state,
            save_event,
            delete_event,
            save_stock,
            save_record,
            backup,
            restore,
            receive_dividend
        ])
        .run(tauri::generate_context!())
        .expect("Gagal menjalankan ARDIN SAHAM");
}
