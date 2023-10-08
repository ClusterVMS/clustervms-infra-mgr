use duct::cmd;
use std::process::Output;



pub struct InfrastructureManager {
	compose_filepath: String,
}

impl InfrastructureManager {
	pub fn new(compose_filepath: &str) -> Self {
		Self {
			compose_filepath: compose_filepath.to_string(),
		}
	}

	pub fn run_processes(&self) -> std::io::Result<Output> {
		let cmd = cmd!("docker", "compose", "-f", &self.compose_filepath, "up")
			// Pass through the uid and gid to run as
			.env("UID", std::env::var("UID").unwrap_or_default())
			.env("GID", std::env::var("GID").unwrap_or_default());
		cmd.run()
	}

	pub fn restart_processes(&self) -> std::io::Result<Output> {
		let cmd = cmd!("docker", "compose", "-f", &self.compose_filepath, "restart")
			// Pass through the uid and gid to run as
			.env("UID", std::env::var("UID").unwrap_or_default())
			.env("GID", std::env::var("GID").unwrap_or_default());
		cmd.run()
	}
}
