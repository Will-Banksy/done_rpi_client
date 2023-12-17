pub mod gpio;

use embedded_graphics::{text::{Text, Baseline}, geometry::Point, mono_font::{MonoTextStyleBuilder, ascii::FONT_6X10}, pixelcolor::BinaryColor, Drawable};
use rppal::i2c::I2c;
use ssd1306::{size::DisplaySize128x64, rotation, Ssd1306, prelude::I2CInterface, mode::{BufferedGraphicsMode, DisplayConfig}};

use crate::config::Env;

use self::gpio::Gpio;

const GPIO_ID_LED: u32 = 0;
const GPIO_ID_BTN_RELOAD: u32 = 1;
const GPIO_ID_BTN_SCROLLR: u32 = 2;
const GPIO_ID_BTN_SCROLLL: u32 = 3;
const GPIO_ID_BTN_DELETE: u32 = 4;

pub struct HardwareSystem {
	gpio: Gpio,
	display: Ssd1306<I2CInterface<I2c>, DisplaySize128x64, BufferedGraphicsMode<DisplaySize128x64>>
}

impl HardwareSystem {
	pub fn new() -> Self {
		let i2c = I2c::new().expect("Could not access the I2C bus - is I2C enabled?");

		let ssd1306_i2c = ssd1306::I2CDisplayInterface::new(i2c);
		let display = ssd1306::Ssd1306::new(ssd1306_i2c, DisplaySize128x64, rotation::DisplayRotation::Rotate0).into_buffered_graphics_mode();

		HardwareSystem {
			gpio: Gpio::new().unwrap(),
			display
		}
	}

	pub fn init(&mut self, env: &Env) {
		self.gpio.open_pin(GPIO_ID_LED, env.gpio.led_pin, true).unwrap();
		self.gpio.open_pin(GPIO_ID_BTN_RELOAD, env.gpio.reload_btn_pin, false).unwrap();
		self.gpio.open_pin(GPIO_ID_BTN_SCROLLR, env.gpio.scrollr_btn_pin, false).unwrap();
		self.gpio.open_pin(GPIO_ID_BTN_SCROLLL, env.gpio.scrolll_btn_pin, false).unwrap();
		self.gpio.open_pin(GPIO_ID_BTN_DELETE, env.gpio.done_btn_pin, false).unwrap();

		self.gpio.write_pin(GPIO_ID_LED, false).unwrap();

		self.display.init().unwrap();
	}

	pub fn deinit(&mut self) {
		self.gpio.close_pin(GPIO_ID_LED).unwrap();
		self.gpio.close_pin(GPIO_ID_BTN_RELOAD).unwrap();
		self.gpio.close_pin(GPIO_ID_BTN_SCROLLR).unwrap();
		self.gpio.close_pin(GPIO_ID_BTN_SCROLLL).unwrap();
		self.gpio.close_pin(GPIO_ID_BTN_DELETE).unwrap();

		self.display.clear_buffer();
		self.display.flush().unwrap();
	}

	pub fn display_text(&mut self, task: &str) {
		println!("Displaying on OLED: {}", task);

		self.display.clear_buffer();

		let text_style = MonoTextStyleBuilder::new()
			.font(&FONT_6X10)
			.text_color(BinaryColor::On)
			.build();

		let characters_per_row = (128.0 / text_style.font.character_size.width as f32).floor() as usize;
		// This next line is extremely inefficient but we move
		let split_text: String = format!(
			"TASK:\n{}",
			task.bytes().collect::<Vec<u8>>().chunks(characters_per_row).map(|chunk| String::from_utf8(chunk.to_vec()).unwrap()).collect::<Vec<String>>().join("\n")
		);

		Text::with_baseline(&split_text, Point::zero(), text_style, Baseline::Top)
			.draw(&mut self.display)
			.unwrap();

		self.display.flush().unwrap();
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

impl Drop for HardwareSystem {
    fn drop(&mut self) {
        self.deinit();
    }
}