use serde::Deserialize;

use crate::config::Env;

#[derive(Deserialize)]
pub struct Task {
	user_task_id: i32,
	task: String,
}

pub fn get_user_tasks(env: &Env) -> Option<Vec<Task>> {
	let client = reqwest::blocking::Client::new();
	let response = client.post(format!("{}/api/tasks/get", env.host)).basic_auth(&env.username, Some(&env.password)).send().ok()?;
	let tasks: Vec<Task> = serde_json::from_slice(&response.bytes().ok()?).ok()?;
	Some(tasks)
}