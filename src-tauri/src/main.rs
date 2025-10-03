use reqwest::Client;
use serde_json::Value;
use std::fs;
use std::io::Write;
use std::path::PathBuf;

#[tauri::command]
async fn register(username: String, password: String) -> Result<String, String> {
    let client = Client::new();
    let response = client
        .post("http://localhost:3000/register")
        .json(&serde_json::json!({
            "username": username,
            "password": password
        }))
        .send()
        .await
        .map_err(|e| e.to_string())?;
    let result: Value = response.json().await.map_err(|e| e.to_string())?;
    Ok(result.to_string())
}

#[tauri::command]
async fn login(username: String, password: String) -> Result<String, String> {
    let client = Client::new();
    let response = client
        .post("http://localhost:3000/login")
        .json(&serde_json::json!({
            "username": username,
            "password": password
        }))
        .send()
        .await
        .map_err(|e| e.to_string())?;
    let result: Value = response.json().await.map_err(|e| e.to_string())?;
    Ok(result.to_string())
}

fn credentials_file_path(_app_handle: &tauri::AppHandle) -> Result<PathBuf, String> {
    let mut dir = std::env::current_dir().map_err(|e| e.to_string())?;
    dir.push("credentials.json");
    Ok(dir)
}

#[tauri::command]
async fn load_credentials(app_handle: tauri::AppHandle) -> Result<String, String> {
    let path = credentials_file_path(&app_handle)?;
    if !path.exists() {
        return Ok("[]".to_string());
    }
    let data = fs::read_to_string(path).map_err(|e| e.to_string())?;
    Ok(data)
}

#[tauri::command]
async fn save_credentials(app_handle: tauri::AppHandle, json: String) -> Result<(), String> {
    let _: Value = serde_json::from_str(&json).map_err(|e| e.to_string())?;
    let path = credentials_file_path(&app_handle)?;
    if let Some(parent) = path.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
    }
    let mut file = fs::File::create(path).map_err(|e| e.to_string())?;
    file.write_all(json.as_bytes()).map_err(|e| e.to_string())?;
    file.flush().map_err(|e| e.to_string())?;
    Ok(())
}

fn main() {
    println!("[main] Starting Tauri app...");

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            register,
            login,
            load_credentials,
            save_credentials
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    println!("[main] Tauri app shutdown.");
}
