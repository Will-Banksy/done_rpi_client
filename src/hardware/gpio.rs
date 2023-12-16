use std::{fs::{File, OpenOptions}, collections::HashMap, os::fd::AsRawFd};

use crate::error::Error;

const GPIODRV_IOCTL_NOTHING: u32 = 50;
const GPIODRV_IOCTL_WRITEPIN: u32 = 51;
const GPIODRV_IOCTL_READPIN: u32 = 52;
const GPIODRV_IOCTL_OPENPIN: u32 = 53;
const GPIODRV_IOCTL_CLOSEPIN: u32 = 54;

#[repr(C)]
struct CGpioPin {
	value: u8,
	pin_num: u32,
	output: u8
}

pub fn bool_to_u8(value: bool) -> u8 {
	if value { 1 } else { 0 }
}

pub fn u8_to_bool(value: u8) -> bool {
	value == 1
}

pub struct Gpio {
	pins: HashMap<u32, CGpioPin>,
	gpio_drv: File
}

impl Gpio {
	pub fn new() -> Result<Self, Error> {
		let gpio_drv = OpenOptions::new().read(true).write(true).open("/dev/gpio_drv").map_err(|e| Error::from(e))?;
		Ok(Gpio {
			pins: HashMap::new(),
			gpio_drv
		})
	}

	pub fn open_pin(&mut self, id: u32, gpio_pin_num: u32, output: bool) -> Result<(), Error> {
		let fd = self.gpio_drv.as_raw_fd();
		let pin_output = bool_to_u8(output);
		let c_gpio_pin = CGpioPin {
			value: 0,
			pin_num: gpio_pin_num,
			output: pin_output
		};

		unsafe {
			if libc::ioctl(fd, GPIODRV_IOCTL_OPENPIN as libc::c_ulong, &c_gpio_pin as *const CGpioPin) == -1 {
				let errno = *libc::__errno_location();
				return Err(Error::Errno(errno));
			}
		}

		self.pins.insert(id, c_gpio_pin);

		Ok(())
	}

	pub fn close_pin(&mut self, id: u32) -> Result<(), Error> {
		let fd = self.gpio_drv.as_raw_fd();
		let c_gpio_pin = self.pins.remove(&id).ok_or(Error::InvalidArgument("Pin not open".to_string()))?;

		unsafe {
			if libc::ioctl(fd, GPIODRV_IOCTL_CLOSEPIN as libc::c_ulong, &c_gpio_pin as *const CGpioPin) == -1 {
				let errno = *libc::__errno_location();
				return Err(Error::Errno(errno));
			}
		}

		Ok(())
	}

	pub fn write_pin(&mut self, id: u32, value: bool) -> Result<(), Error> {
		let fd = self.gpio_drv.as_raw_fd();
		let c_gpio_pin = self.pins.get_mut(&id).ok_or(Error::InvalidArgument("Pin not open".to_string()))?;
		c_gpio_pin.value = bool_to_u8(value);

		unsafe {
			if libc::ioctl(fd, GPIODRV_IOCTL_WRITEPIN as libc::c_ulong, c_gpio_pin as *const CGpioPin) == -1 {
				let errno = *libc::__errno_location();
				return Err(Error::Errno(errno));
			}
		}

		Ok(())
	}

	pub fn read_pin(&mut self, id: u32) -> Result<bool, Error> {
		let fd = self.gpio_drv.as_raw_fd();
		let c_gpio_pin = self.pins.get_mut(&id).ok_or(Error::InvalidArgument("Pin not open".to_string()))?;

		unsafe {
			if libc::ioctl(fd, GPIODRV_IOCTL_READPIN as libc::c_ulong, c_gpio_pin as *mut CGpioPin) == -1 {
				let errno = *libc::__errno_location();
				return Err(Error::Errno(errno));
			}
		}

		// println!("Read pin value: {}", c_gpio_pin.value);

		Ok(u8_to_bool(c_gpio_pin.value))
	}
}
