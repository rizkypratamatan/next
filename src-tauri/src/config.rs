use crate::models::Config;

impl Default for Config {
	fn default() -> Self {
		Self {
			base_url: "http://tauri.localhost".to_string(),
			password: "26101103".to_string(),
		}
	}
}
