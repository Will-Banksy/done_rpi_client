use serde::Deserialize;

#[derive(Deserialize)]
pub struct Env {
	pub host: String,
	pub username: String,
	pub password: String
}

impl Env {
	pub fn from_file() -> Option<Env> {
		let env_str = std::fs::read_to_string(".env").ok()?;
		toml::from_str(&env_str).ok()
	}
}