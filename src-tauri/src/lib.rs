use dotenv::from_filename;
use std::env;
use tauri::menu::{MenuBuilder, MenuItemBuilder};
use tauri::tray::{MouseButton, TrayIconBuilder, TrayIconEvent};
use tauri::{AppHandle, Manager, Url, WindowEvent, Wry};

#[tauri::command]
fn login(app_handle: AppHandle, password: &str) -> String {
	if password == env::var("VITE_APPLICATION_PASSWORD").unwrap_or_default() {
		if let Some(window) = app_handle.get_webview_window("main") {
			let _ = window.eval("window.location.href = 'https://web.whatsapp.com/';");
		}

		return "Password matched. You will redirect to the page.".to_string();
	}

	"Incorrect password. Please try again.".to_string()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
	if cfg!(debug_assertions) {
		from_filename(".env.development").ok();
	} else {
		from_filename(".env.production").ok();
	}

	tauri::Builder::<Wry>::default()
		.setup(|app| {
			let app_handle: &AppHandle = app.handle();

			// Build tray menus
			let tray_menu = MenuBuilder::new(&app_handle.clone())
				.item(
					&MenuItemBuilder::new("Open")
						.id("open")
						.build(&app_handle.clone())?,
				)
				.item(
					&MenuItemBuilder::new("Quit")
						.id("quit")
						.build(&app_handle.clone())?,
				)
				.build()?;

			// Build tray icon
			TrayIconBuilder::new()
				.icon(app_handle.default_window_icon().unwrap().clone())
				.menu(&tray_menu)
				.on_menu_event(|app, event| match event.id().as_ref() {
					"open" => {
						if let Some(window) = app.get_webview_window("main") {
							window.show().unwrap();
							window.set_focus().unwrap();
						}
					}
					"quit" => app.exit(0),
					_ => {}
				})
				.on_tray_icon_event(|tray, event| {
					if let TrayIconEvent::Click { button, .. } = event {
						if button == MouseButton::Left {
							if let Some(window) = tray.app_handle().get_webview_window("main") {
								window.show().unwrap();
								window.set_focus().unwrap();
							}
						}
					}
				})
				.build(app)?;

			Ok(())
		})
		.on_window_event(|window, event| {
			if let WindowEvent::CloseRequested { api, .. } = event {
				if let Some(webview) = window.get_webview_window(window.label()) {
					let _ = webview.navigate(
						Url::parse(&env::var("VITE_APPLICATION_URL").unwrap_or_default()).unwrap(),
					);
				}

				// Hide to tray
				window.hide().unwrap();
				api.prevent_close();
			}
		})
		.invoke_handler(tauri::generate_handler![login])
		.run(tauri::generate_context!())
		.expect("Error while running Next application");
}
