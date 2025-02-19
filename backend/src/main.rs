use std::sync::Arc;
use std::env;
use poem::listener::TcpListener;
use poem::middleware::Cors;
use poem::{EndpointExt, Route, Server};
use poem_openapi::param::Path;
use poem_openapi::payload::{Json, PlainText};
use poem_openapi::{OpenApi, OpenApiService};
use rcon::{Connection, Error};
use tokio::{net::TcpStream, sync::Mutex};

/// Handle for RCON
type RconHandle = Arc<Mutex<Connection<TcpStream>>>;

/// Creates RCON with server
async fn create_rcon_connection() -> Result<RconHandle, String> {
	let address = env::var("RCON_HOST").expect("Unknown RCON host!");
	// let address = "localhost:25575"; // TODO: Use env file
	let password = env::var("RCON_PASSWORD").expect("Unknown RCON password");
	
	let connection = Connection::builder()
		.connect(address, &password)
		.await
		.expect("Connection failed");

	let handle: RconHandle = Arc::new(Mutex::new(connection));

	return Ok(handle);
}

struct RconApi {
	connection: RconHandle
}

impl RconApi {
	/// Performs the given minecraft command
	async fn perform_command(&self, command: String) -> Result<String, Error> {
		return Ok(self.connection.lock().await.cmd(&command).await?);
	}
}

#[OpenApi]
impl RconApi {
	/// Gives the index page
	#[oai(path = "/", method = "get")]
	async fn index(&self) -> PlainText<String> {
		let a = self.connection.clone().lock().await.cmd("list").await.expect("Failed to get list");
		return PlainText(a);
	}

	#[oai(path="/players", method="get")]
	async fn list_players(&self) -> Json<i32> {
		return Json(32);
	}

	/// Gives OP rights to the given username
	#[oai(path="/op/:player", method="post")]
	async fn give_op_rights(&self, player: Path<String>) -> Json<bool> {
		let cmd = format!("op {}", player.0);
		return Json(self.perform_command(cmd).await.is_ok());
	}
	/// Remove OP rights from the given username
	#[oai(path="/op/:player", method="delete")]
	async fn remove_op_rights(&self, player: Path<String>) -> Json<bool> {
		let cmd = format!("deop {}", player.0);
		return Json(self.perform_command(cmd).await.is_ok());
	}
}



#[tokio::main]
async fn main() {
	println!("Starting your msc backend!");
	
	let handle = create_rcon_connection().await.expect("Hello");

	let rcon_api = RconApi { connection: handle };

	let api_service = OpenApiService::new(rcon_api, "Hello World", "1.0").server("http://0.0.0.0:3000");
	let ui = api_service.swagger_ui();
	let app = Route::new()
		.nest("/", api_service)
		.nest("/docs", ui)
		.with(Cors::new());

	Server::new(TcpListener::bind("0.0.0.0:3000"))
		.run(app)
		.await
		.expect("Server error!");
}
