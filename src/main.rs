mod hardware;
mod api;
mod config;
mod error;

use config::Env;

use crate::api::Task;

fn main() {
	let env = Env::from_file().unwrap();

	let tasks = api::get_user_tasks(&env);

	println!("get tasks: {:?}", tasks);

	let new_task = Task { user_task_id: 1, task: "Get https working".into() };

	let success = api::set_user_tasks(&env, &[new_task]);

	println!("set tasks: {:?}", success);

	let success = api::delete_user_tasks(&env, &[0]);

	println!("del tasks: {:?}", success);

	let tasks = api::get_user_tasks(&env);

	println!("get tasks: {:?}", tasks);
}

