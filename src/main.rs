mod hardware;
mod api;
mod config;
mod error;

use std::{sync::{Arc, mpsc::{self, Sender}, RwLock}, thread, time::Duration};

use config::Env;
use hardware::HardwareSystem;

use crate::api::Task;

enum ButtonEvent {
	ReloadTasks,
	ScrollRight,
	ScrollLeft,
	TaskDone
}

const NO_TASKS_MSG: &'static str = "No tasks";

fn main() {
	// let delay = 1;

	// println!("INITIALISING GPIO");

	// let mut gpio = Gpio::new().unwrap();

	// thread::sleep(Duration::from_secs(delay));

	// println!("OPENING PIN 23");

	// gpio.open_pin(0, 23, true).unwrap();

	// thread::sleep(Duration::from_secs(delay));

	// println!("WRITING 1 TO PIN 23");

	// gpio.write_pin(0, true).unwrap();

	// thread::sleep(Duration::from_secs(delay));

	// println!("OPENING PIN 25");

	// gpio.open_pin(1, 25, true).unwrap();

	// thread::sleep(Duration::from_secs(delay));

	// println!("WRITING 1 TO PIN 25");

	// gpio.write_pin(1, true).unwrap();

	// thread::sleep(Duration::from_secs(delay));

	// println!("CLOSING PINS 23 AND 25");

	// gpio.close_pin(0).unwrap();
	// gpio.close_pin(1).unwrap();

	// println!("OPENING PIN 17");

	// gpio.open_pin(2, 17, false).unwrap();

	// println!("WAITING FOR INPUT FROM PIN 17");

	// while !gpio.read_pin(2).unwrap() {
	// 	thread::sleep(Duration::from_secs(1))
	// }

	// println!("CLOSING PIN 2");

	// gpio.close_pin(2).unwrap();

	// return;

	let env = Env::from_file().expect("[CRITICAL]: No env config detected, cannot run");

	let hardware_system = Arc::new(RwLock::new(HardwareSystem::new()));
	{
		hardware_system.write().unwrap().init(&env);
	}

	let (btn_evt_sender, btn_evt_reciever) = mpsc::channel::<ButtonEvent>();

	let hardware_system_clone = Arc::clone(&hardware_system);
	let btn_poll_thread_handle = thread::spawn(move || {
		btn_evt_sender.send(ButtonEvent::ReloadTasks).unwrap();
		poll_btns(hardware_system_clone, btn_evt_sender);
	});

	let mut held_tasks: Vec<Task> = Vec::new();
	let mut curr_task_idx: usize = 0;

	loop {
		// Block until event is sent
		let evt = btn_evt_reciever.recv().unwrap();

		match evt {
			ButtonEvent::ReloadTasks => {
				// Reload env (might have changed)
				let env = Env::from_file();

				if let Some(env) = env {
					let tasks = api::get_user_tasks(&env);
					match tasks {
						Ok(mut tasks) => {
							held_tasks.clear();
							held_tasks.append(&mut tasks);

							curr_task_idx = 0;
							if held_tasks.len() >= 1 {
								hardware_system.write().unwrap().display_text(&held_tasks[curr_task_idx].task);
							} else {
								hardware_system.write().unwrap().display_text(NO_TASKS_MSG);
							}
							hardware_system.write().unwrap().connection_led_toggle(true);
						}
						Err(e) => {
							eprintln!("[ERROR]: {:?}", e);
							hardware_system.write().unwrap().connection_led_toggle(false);
						}
					}
				} else {
					eprintln!("[ERROR]: Nonexistent or invalid env");
				}
			},
			ButtonEvent::ScrollRight => {
				curr_task_idx += 1;
				if curr_task_idx >= held_tasks.len() {
					curr_task_idx = 0;
				}

				if held_tasks.len() >= 1 {
					hardware_system.write().unwrap().display_text(&held_tasks[curr_task_idx].task);
				} else {
					hardware_system.write().unwrap().display_text(NO_TASKS_MSG);
				}
			},
			ButtonEvent::ScrollLeft => {
				if curr_task_idx == 0 {
					curr_task_idx = held_tasks.len();
					if curr_task_idx != 0 {
						curr_task_idx -= 1;
					}
				} else {
					curr_task_idx -= 1;
				}

				if held_tasks.len() >= 1 {
					hardware_system.write().unwrap().display_text(&held_tasks[curr_task_idx].task);
				} else {
					hardware_system.write().unwrap().display_text(NO_TASKS_MSG);
				}
			},
			ButtonEvent::TaskDone => {
				// Reload env (might have changed)
				let env = Env::from_file();

				if let Some(env) = env {
					let res = api::delete_user_tasks(&env, &[held_tasks[curr_task_idx].user_task_id]);
					if let Err(e) = res {
						eprintln!("[ERROR]: {:?}", e);
					}
					held_tasks.remove(curr_task_idx);
					if curr_task_idx >= held_tasks.len() && curr_task_idx != 0 {
						curr_task_idx -= 1;
					}

					if held_tasks.len() >= 1 {
						hardware_system.write().unwrap().display_text(&held_tasks[curr_task_idx].task);
					} else {
						hardware_system.write().unwrap().display_text(NO_TASKS_MSG);
					}
				} else {
					eprintln!("[ERROR]: Nonexistent or invalid env");
				}
			},
		}
	}

	btn_poll_thread_handle.join();
}

fn poll_btns(hardware_system: Arc<RwLock<HardwareSystem>>, btn_evt_sender: Sender<ButtonEvent>) {
	let (
		mut reload_debounce,
		mut scrollr_debounce,
		mut scrolll_debounce,
		mut delete_debounce
	) = (
		false,
		false,
		false,
		false
	);

	loop {
		thread::sleep(Duration::from_millis(100));

		let reload = {
			hardware_system.write().unwrap().get_reload_btn_state()
		};
		let scrollr = {
			hardware_system.write().unwrap().get_scrollr_btn_state()
		};
		let scrolll = {
			hardware_system.write().unwrap().get_scrolll_btn_state()
		};
		let delete = {
			hardware_system.write().unwrap().get_delete_btn_state()
		};

		// println!("Done polling buttons...");

		if reload && !reload_debounce {
			println!("Sending button press: Reload");
			btn_evt_sender.send(ButtonEvent::ReloadTasks).unwrap();
		}
		reload_debounce = reload;

		if scrollr && !scrollr_debounce {
			println!("Sending button press: Scroll Right");
			btn_evt_sender.send(ButtonEvent::ScrollRight).unwrap();
		}
		scrollr_debounce = scrollr;

		if scrolll && !scrolll_debounce {
			println!("Sending button press: Scroll Left");
			btn_evt_sender.send(ButtonEvent::ScrollLeft).unwrap();
		}
		scrolll_debounce = scrolll;

		if delete && !delete_debounce {
			println!("Sending button press: Done");
			btn_evt_sender.send(ButtonEvent::TaskDone).unwrap();
		}
		delete_debounce = delete;
	}
}