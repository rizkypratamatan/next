use crate::models::Config;
use tauri::{AppHandle, Manager};

#[tauri::command]
pub fn login(app_handle: AppHandle, password: &str) -> String {
	if password == Config::default().password {
		if let Some(window) = app_handle.get_webview_window("main") {
			let _ = window.eval("window.location.href = 'https://web.whatsapp.com/';");
		}

		return "Password matched. You will redirect to the page.".to_string();
	}

	"Incorrect password. Please try again.".to_string()
}
