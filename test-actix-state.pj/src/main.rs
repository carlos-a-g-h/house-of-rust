use actix_web::{web, App, HttpServer, Responder};
use std::sync::Mutex;

struct AppStateWithCounter
{
	//Mutex is necessary to mutate safely across threads
	//counter: Mutex<i32>,
	counter: Mutex<Vec<String>>
}

async fn index(data: web::Data<AppStateWithCounter>) -> impl Responder
{
	//get counter's MutexGuard
	let mut counter=data.counter.lock().unwrap();
	// access counter inside MutexGuard
	//response with count
	println!("counter = {:?}",counter);
	"Look at the terminal"
}

async fn add_one(data: web::Data<AppStateWithCounter>) -> impl Responder
{
	let mut counter=data.counter.lock().unwrap();
	*counter.push("another_one".to_string());
	"Added one"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	println!("Actix Web (Rust) demo at 8080\n\nTopic: Application Mutable State")
	// Note: web::Data created _outside_ HttpServer::new closure
	let counter = web::Data::new(AppStateWithCounter {
		counter: Mutex::new(
			Vec::new()
		),
	});

	HttpServer::new(move || {
	// move counter into the closure
		App::new()
			.app_data(counter.clone()) // <- register the created data
			.route("/", web::get().to(index))
			.route("/add", web::get().to(add_one))
	})
	.bind(("127.0.0.1", 8080))?
	.run()
	.await
}
