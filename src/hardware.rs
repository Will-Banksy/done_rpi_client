pub mod gpio;

use crate::config::Env;

use self::gpio::Gpio;

const GPIO_ID_LED: u32 = 0;
const GPIO_ID_BTN_RELOAD: u32 = 1;
const GPIO_ID_BTN_SCROLLR: u32 = 2;
const GPIO_ID_BTN_SCROLLL: u32 = 3;
const GPIO_ID_BTN_DELETE: u32 = 4;

pub struct HardwareSystem {
	gpio: Gpio
}

impl HardwareSystem {
	pub fn new() -> Self {
		HardwareSystem {
			gpio: Gpio::new().unwrap()
		}
	}

	pub fn init(&mut self, env: &Env) {
		self.gpio.open_pin(GPIO_ID_LED, env.gpio.led_pin, true).unwrap();
		self.gpio.open_pin(GPIO_ID_BTN_RELOAD, env.gpio.reload_btn_pin, false).unwrap();
		self.gpio.open_pin(GPIO_ID_BTN_SCROLLR, env.gpio.scrollr_btn_pin, false).unwrap();
		self.gpio.open_pin(GPIO_ID_BTN_SCROLLL, env.gpio.scrolll_btn_pin, false).unwrap();
		self.gpio.open_pin(GPIO_ID_BTN_DELETE, env.gpio.done_btn_pin, false).unwrap();

		self.gpio.write_pin(GPIO_ID_LED, false).unwrap();
	}

	pub fn deinit(&mut self) {
		self.gpio.close_pin(GPIO_ID_LED).unwrap();
		self.gpio.close_pin(GPIO_ID_BTN_RELOAD).unwrap();
		self.gpio.close_pin(GPIO_ID_BTN_SCROLLR).unwrap();
		self.gpio.close_pin(GPIO_ID_BTN_SCROLLL).unwrap();
		self.gpio.close_pin(GPIO_ID_BTN_DELETE).unwrap();
	}

	pub fn display_text(&self, task: &str) {
		println!("Displaying on OLED: {}", task);
	}

	pub fn clear_oled(&self) {
		println!("Clearing OLED");
	}

	pub fn connection_led_toggle(&mut self, on: bool) {
		self.gpio.write_pin(GPIO_ID_LED, on).unwrap();
	}

	pub fn get_reload_btn_state(&mut self) -> bool {
		self.gpio.read_pin(GPIO_ID_BTN_RELOAD).unwrap()
	}

	pub fn get_scrollr_btn_state(&mut self) -> bool {
		self.gpio.read_pin(GPIO_ID_BTN_SCROLLR).unwrap()
	}

	pub fn get_scrolll_btn_state(&mut self) -> bool {
		self.gpio.read_pin(GPIO_ID_BTN_SCROLLL).unwrap()
	}

	pub fn get_delete_btn_state(&mut self) -> bool {
		self.gpio.read_pin(GPIO_ID_BTN_DELETE).unwrap()
	}
}
