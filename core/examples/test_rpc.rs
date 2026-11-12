//! Test-only adapter: the desktop uses Tauri commands, never this process.
use ardin_core::{Event, Record, Store};
use serde_json::{json, Value};
use std::io::{self, Read};
fn run() -> Result<Value, String> {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .map_err(|e| e.to_string())?;
    let request: Value = serde_json::from_str(&input).map_err(|e| e.to_string())?;
    let path = std::env::var("ARDIN_TEST_DB").map_err(|e| e.to_string())?;
    let mut db = Store::open(std::path::Path::new(&path))?;
    let a = &request["args"];
    let string = |k: &str| a[k].as_str().unwrap_or("");
    match request["cmd"].as_str().unwrap_or("") {
        "read_state" => db.state(),
        "save_stock" => {
            db.stock(string("code"), string("name"), string("sector"))?;
            Ok(Value::Null)
        }
        "save_event" => {
            let e: Event = serde_json::from_value(a["event"].clone()).map_err(|e| e.to_string())?;
            db.save_event(e)?;
            Ok(Value::Null)
        }
        "save_record" => {
            let r: Record =
                serde_json::from_value(a["record"].clone()).map_err(|e| e.to_string())?;
            db.save_record(r)?;
            Ok(Value::Null)
        }
        "delete_event" => {
            db.delete_event(a["id"].as_i64().ok_or("ID missing")?)?;
            Ok(Value::Null)
        }
        "receive_dividend" => {
            db.receive_dividend(a["id"].as_i64().ok_or("ID missing")?)?;
            Ok(Value::Null)
        }
        _ => Err("Unsupported test command".into()),
    }
}
fn main() {
    println!(
        "{}",
        match run() {
            Ok(v) => json!({"ok":v}),
            Err(e) => json!({"error":e}),
        }
    );
}
