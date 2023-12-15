use serde::Deserialize;

#[derive(Deserialize)]
pub struct Env {
	pub host: String,
	pub username: String,
	pub password: String,
	pub gpio: GpioEnv
}

#[derive(Deserialize)]
pub struct GpioEnv {
	pub led_pin: u32,
	pub reload_btn_pin: u32,
	pub scrollr_btn_pin: u32,
	pub scrolll_btn_pin: u32,
	pub done_btn_pin: u32,
}

impl Env {
	pub fn from_file() -> Option<Env> {
		let env_str = std::fs::read_to_string(".env").ok()?;
		toml::from_str(&env_str).ok()
	}
}