use rocket::http::Status;
use rocket::post;
use rocket::serde::json::{json, Value};
use rocket::State;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::error;

use crate::infrastructure_manager::InfrastructureManager;



#[post("/system/reload")]
async fn reload(infra_mgr_state: &State<Arc<RwLock<InfrastructureManager>>>) -> (Status, String) {
	let infra_mgr = infra_mgr_state.read().await;

	info!("restarting processes...");
	
	match infra_mgr.restart_processes() {
		Ok(_) => {
			info!("Restarted processes");
			(Status::Accepted, String::from(""))
		}
		Err(err) => {
			error!("Error while restarting processes: {err:?}");
			(Status::InternalServerError, String::from("Internal server error"))
		}
	}
}

#[catch(404)]
fn not_found() -> Value {
	json!({
		"status": "error",
		"reason": "Resource was not found."
	})
}

pub fn stage(infra_mgr: std::sync::Arc<tokio::sync::RwLock<InfrastructureManager>>) -> rocket::fairing::AdHoc {
	rocket::fairing::AdHoc::on_ignite("JSON", |rocket| async {
		rocket
			.manage(infra_mgr)
			.register("/", catchers![not_found])
			.mount("/v0/", routes![reload])
	})
}
