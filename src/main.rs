use async_std::task;
use clap::{Arg, Command};
use clustervms::config;
use core::time::Duration;
use infrastructure_manager::InfrastructureManager;
use rocket::{Request, Response};
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;

mod infrastructure_manager;
mod rest_api;

#[macro_use] extern crate rocket;

const DEFAULT_COMPOSE_FILE: &str = "/etc/clustervms/clustervms-compose.yaml";



// Since the UI is served by another server, we may need to setup CORS to allow the UI to make requests to this server, in case they don't share a proxy server.
pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
	fn info(&self) -> Info {
		Info {
			name: "Add CORS headers to responses",
			kind: Kind::Response
		}
	}

	async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
		response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
		response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, PATCH, OPTIONS"));
		response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
		response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
	}
}


#[rocket::main]
async fn main() -> anyhow::Result<()> {
	let matches = Command::new("clustervms-infra-mgr")
		.version("0.0.1")
		.author("Alicrow")
		.about("ClusterVMS infrastructure manager.")
		.arg(
			Arg::new("config")
				.short('c')
				.long("config")
				.help("TOML file with ClusterVMS config")
		)
		.arg(
			Arg::new("compose")
				.long("compose")
				.help("Docker Compose file to use")
		)
		.arg(
			Arg::new("verbose")
				.long("verbose")
				.short('v')
				.help("Prints debug log information"),
		)
		.get_matches();

	let log_level = if matches.contains_id("verbose") {
		tracing::Level::DEBUG
	} else {
		tracing::Level::INFO
	};
	
	tracing_subscriber::fmt()
		.with_max_level(log_level)
		.init();

	let compose_filepath = match matches.get_one::<String>("compose") {
		Some(filename) => filename,
		None => DEFAULT_COMPOSE_FILE,
	};

	let mut config_manager = config::ConfigManager::new();

	let config_filename_matches = matches.get_many::<String>("config");
	let res = match config_filename_matches {
		Some(filenames) => {
			config_manager.read_config(filenames.map(|v| v.as_str()).collect())
		},
		None => {
			// Use default file path
			config_manager.read_default_config_files()
		}
	};
	match res {
		Ok(()) => {}
		Err(err) => {
			error!("Failed to read config file(s): error was: {}", err)
		}
	}

	let mut infra_mgr = std::sync::Arc::new(tokio::sync::RwLock::new(InfrastructureManager::new(compose_filepath)));
	let mut infra_mgr_clone = infra_mgr.clone();

	let mut run_requested  = true;
	tokio::spawn(async move {
		loop {
			if run_requested {
				infra_mgr_clone.read().await.run_processes();
			}

			// Sleep for a bit after failure
			// If the issue persists, we don't want to waste all our time constantly trying to start up again.
			task::sleep(Duration::from_secs(1)).await;
		}
	});

	// Listen for command to restart components
	let _rocket = rocket::build()
		.attach(rest_api::stage(infra_mgr))
		.attach(CORS)
		.launch()
		.await?;

	anyhow::Ok(())
}
