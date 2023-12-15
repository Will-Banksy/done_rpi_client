pub mod gpio;

use crate::config::Env;

pub struct HardwareSystem {
}

// maybe just put the hardwaresystem in a rwlock
unsafe impl Sync for HardwareSystem {}
unsafe impl Send for HardwareSystem {}

impl HardwareSystem {
	pub fn init(&self, env: &Env) {
		todo!()
	}

	pub fn deinit(&self) {
		todo!()
	}

	pub fn display_text(&self, task: &str) {
		todo!()
	}

	pub fn clear_oled(&self) {
		todo!()
	}

	pub fn connection_led_toggle(&self, on: bool) {
		todo!()
	}

	pub fn get_reload_btn_state(&self) -> bool {
		todo!()
	}

	pub fn get_scrollr_btn_state(&self) -> bool {
		todo!()
	}

	pub fn get_scrolll_btn_state(&self) -> bool {
		todo!()
	}

	pub fn get_delete_btn_state(&self) -> bool {
		todo!()
	}
}
