use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

use crate::{config::Env, error::Error};

#[derive(Deserialize)]
pub struct GetTasksResponse {
	success: bool,
	tasks: Vec<Task>
}

#[derive(Deserialize)]
pub struct SuccessResponse {
	success: bool
}

pub type DeleteTasksResponse = SuccessResponse;
pub type SetTasksResponse = SuccessResponse;

#[derive(Debug, Deserialize, Serialize)]
pub struct Task {
	pub user_task_id: i32,
	pub task: String,
}

/// Sends a (blocking) API request to the server address as specified by the env to retrieve the tasks for the specified user
pub fn get_user_tasks(env: &Env) -> Result<Vec<Task>, Error> {
	let client = reqwest::blocking::Client::new();
	let response = client.post(format!("{}/api/tasks/get", env.host)).basic_auth(&env.username, Some(&env.password)).send().map_err(|e| Error::from(e))?;
	if response.status() != StatusCode::OK {
		return Err(Error::UnsuccessfulApiRequestError)
	}
	let response_deserialized: GetTasksResponse = serde_json::from_slice(&response.bytes().map_err(|e| Error::from(e))?).map_err(|e| Error::from(e))?;
	if response_deserialized.success {
		Ok(response_deserialized.tasks)
	} else {
		Err(Error::UnsuccessfulApiRequestError)
	}
}

/// Sends a (blocking) API request to the server address as specified by the env to delete a specifiec task
pub fn delete_user_tasks(env: &Env, user_task_ids: &[i32]) -> Result<(), Error> {
	let client = reqwest::blocking::Client::new();
	let body = serde_json::to_string(user_task_ids).map_err(|e| Error::from(e))?;
	println!("delete_user_tasks body: {}", body);
	let response = client.post(format!("{}/api/tasks/delete", env.host)).body(body).header(reqwest::header::CONTENT_TYPE, "application/json").basic_auth(&env.username, Some(&env.password)).send().map_err(|e| Error::from(e))?;
	if response.status() != StatusCode::OK {
		return Err(Error::UnsuccessfulApiRequestError)
	}
	let response_deserialized: DeleteTasksResponse = serde_json::from_slice(&response.bytes().map_err(|e| Error::from(e))?).map_err(|e| Error::from(e))?;
	if response_deserialized.success {
		Ok(())
	} else {
		Err(Error::UnsuccessfulApiRequestError)
	}
}

/// Sends a (blocking) API request to the server address as specified by the env to add/set tasks
pub fn set_user_tasks(env: &Env, tasks: &[Task]) -> Result<(), Error> {
	let client = reqwest::blocking::Client::new();
	let body = serde_json::to_string(tasks).map_err(|e| Error::from(e))?;
	println!("set_user_tasks body: {}", body);
	let response = client.post(format!("{}/api/tasks/set", env.host)).body(body).header(reqwest::header::CONTENT_TYPE, "application/json").basic_auth(&env.username, Some(&env.password)).send().map_err(|e| Error::from(e))?;
	if response.status() != StatusCode::OK {
		return Err(Error::UnsuccessfulApiRequestError)
	}
	let response_deserialized: SetTasksResponse = serde_json::from_slice(&response.bytes().map_err(|e| Error::from(e))?).map_err(|e| Error::from(e))?;
	if response_deserialized.success {
		Ok(())
	} else {
		Err(Error::UnsuccessfulApiRequestError)
	}
}
