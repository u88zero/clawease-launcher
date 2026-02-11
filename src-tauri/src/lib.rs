use std::process::Command;
use std::fs;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use sysinfo::{System, SystemExt, CpuExt};

#[derive(Serialize, Deserialize)]
pub struct SystemStats {
    pub cpu_usage: f32,
    pub memory_used: u64,
    pub memory_total: u64,
    pub openclaw_status: String,
}

#[derive(Serialize, Deserialize)]
pub struct OpenClawConfig {
    pub telegram_token: Option<String>,
    pub primary_model: Option<String>,
    pub qq_mail_auth: Option<String>,
}

#[tauri::command]
fn get_system_stats() -> SystemStats {
    let mut sys = System::new_all();
    sys.refresh_all();
    
    // Check if openclaw process is running
    let is_running = sys.processes_by_exact_name("openclaw").next().is_some();
    
    SystemStats {
        cpu_usage: sys.global_cpu_info().cpu_usage(),
        memory_used: sys.used_memory() / 1024 / 1024, // MB
        memory_total: sys.total_memory() / 1024 / 1024, // MB
        openclaw_status: if is_running { "Running".into() } else { "Stopped".into() },
    }
}

#[tauri::command]
async fn run_env_repair() -> Result<String, String> {
    // Real logic: We should bundle the script or download it
    // For now, try to run a simple check
    #[cfg(target_os = "windows")]
    {
        let output = Command::new("powershell")
            .args(["-Command", "Get-ExecutionPolicy"])
            .output()
            .map_err(|e| e.to_string())?;
        Ok(format!("Current Policy: {}", String::from_utf8_lossy(&output.stdout).trim()))
    }

    #[cfg(not(target_os = "windows"))]
    {
        Ok("Linux detection active".into())
    }
}

#[tauri::command]
async fn read_config() -> Result<OpenClawConfig, String> {
    // In production, this would look at $HOME/.openclaw/openclaw.json
    Ok(OpenClawConfig {
        telegram_token: Some("HIDDEN".into()),
        primary_model: Some("gemini-3-flash".into()),
        qq_mail_auth: Some("nrfje...".into()),
    })
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_system_stats,
            run_env_repair,
            read_config
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
