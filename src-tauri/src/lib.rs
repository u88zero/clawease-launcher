use std::process::Command;
use std::fs;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct OpenClawConfig {
    // We'll use a simplified structure for demonstration, 
    // real app would parse the full openclaw.json
    pub telegram_token: Option<String>,
    pub primary_model: Option<String>,
    pub qq_mail_auth: Option<String>,
}

#[tauri::command]
async fn run_env_repair() -> Result<String, String> {
    #[cfg(target_os = "windows")]
    {
        // Run the install_cn.ps1 script
        let output = Command::new("powershell")
            .args(["-ExecutionPolicy", "Bypass", "-File", "../../projects/clawease/install_cn.ps1"])
            .output()
            .map_err(|e| e.to_string())?;
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    #[cfg(not(target_os = "windows"))]
    {
        // Run the install_cn.sh script
        let output = Command::new("bash")
            .arg("../../projects/clawease/install_cn.sh")
            .output()
            .map_err(|e| e.to_string())?;
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }
}

#[tauri::command]
async fn read_config() -> Result<OpenClawConfig, String> {
    // In real app, locate ~/.openclaw/openclaw.json
    // For now, let's simulate or read a dummy path
    Ok(OpenClawConfig {
        telegram_token: Some("123456:ABC...".into()),
        primary_model: Some("google/gemini-3-pro".into()),
        qq_mail_auth: Some("nrfjezafciavbaic".into()),
    })
}

#[tauri::command]
async fn save_config(config: OpenClawConfig) -> Result<(), String> {
    println!("Saving config: {:?}", config.primary_model);
    // Logic to write back to openclaw.json goes here
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            run_env_repair,
            read_config,
            save_config
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
