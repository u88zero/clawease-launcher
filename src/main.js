const { invoke } = window.__TAURI__.core;

window.addEventListener("DOMContentLoaded", () => {
  const consoleBody = document.querySelector("#pro-console");
  const launchBtn = document.querySelector("#launch-btn");
  const repairBtn = document.querySelector("#repair-btn");
  const saveBtn = document.querySelector("#save-config-btn");
  const navItems = document.querySelectorAll('.nav-item[data-view]');
  const views = document.querySelectorAll('.content-view');

  // Input elements
  const tokenInput = document.querySelector("#config-token");
  const modelSelect = document.querySelector("#config-model");
  const emailKeyInput = document.querySelector("#config-email-key");

  function log(msg, type='INFO') {
    const time = new Date().toLocaleTimeString();
    const line = document.createElement('div');
    line.className = 'log-line';
    line.innerHTML = `<span class="log-ts">[${time}]</span> <span class="log-tag tag-${type.toLowerCase()}">[${type}]</span> ${msg}`;
    consoleBody.appendChild(line);
    consoleBody.scrollTop = consoleBody.scrollHeight;
  }

  // View switching
  navItems.forEach(item => {
    item.onclick = () => {
      const targetView = item.getAttribute('data-view');
      navItems.forEach(i => i.classList.remove('active'));
      views.forEach(v => v.classList.remove('active'));
      
      item.classList.add('active');
      const target = document.getElementById(`view-${targetView}`);
      if (target) target.classList.add('active');
    };
  });

  // Load Initial Config
  async function loadConfig() {
    try {
      const config = await invoke("read_config");
      if (config.telegram_token) tokenInput.value = config.telegram_token;
      if (config.primary_model) modelSelect.value = config.primary_model;
      if (config.qq_mail_auth) emailKeyInput.value = config.qq_mail_auth;
      log("本地配置文件加载成功。", "SUCCESS");
    } catch (e) {
      log("加载配置失败: " + e, "ERROR");
    }
  }

  // Repair Environment
  repairBtn.onclick = async () => {
    log("正在执行暴力越狱环境修补...", "INFO");
    try {
      const result = await invoke("run_env_repair");
      log(result, "SUCCESS");
    } catch (e) {
      log("修复过程出错: " + e, "ERROR");
    }
  };

  // Save Config
  saveBtn.onclick = async () => {
    const config = {
      telegram_token: tokenInput.value,
      primary_model: modelSelect.value,
      qq_mail_auth: emailKeyInput.value
    };
    try {
      await invoke("save_config", { config });
      log("配置已成功保存并同步到 openclaw.json。", "SUCCESS");
    } catch (e) {
      log("保存失败: " + e, "ERROR");
    }
  };

  // Launch Tony V (Simulated for UI)
  launchBtn.onclick = () => {
    if (launchBtn.innerText.includes("启动")) {
      log("正在启动 TONY V 核心进程...", "INFO");
      setTimeout(() => log("OpenClaw WebSocket 网关就绪。", "SUCCESS"), 1000);
      setTimeout(() => log("Telegram 机器人已连接。", "SUCCESS"), 2000);
      launchBtn.innerText = "🛑 停止运行";
      launchBtn.style.background = "var(--error)";
    } else {
      log("核心进程已安全终止。", "WARN");
      launchBtn.innerText = "🚀 启动 TONY V 核心";
      launchBtn.style.background = "var(--primary)";
    }
  };

  loadConfig();
});
