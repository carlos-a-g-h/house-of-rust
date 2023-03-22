use actix_web::{web, App, HttpServer};
use std::sync::Mutex;

struct AppStateWithCounter
{
	//Mutex is necessary to mutate safely across threads
	//counter: Mutex<i32>,
	counter: Mutex<Vec<&str>>
}

async fn index(data: web::Data<AppStateWithCounter>) -> String
{
		//get counter's MutexGuard
		let mut counter=data.counter.lock().unwrap();
		// access counter inside MutexGuard
		//response with count
		format!("Counter number: {counter}")
}

async fn add(data: web::Data<AppStateWithCounter>) -> String
{
	let mut counter=data.counter.lock().unwrap();
	*counter.push("yes");
	format!("Added one")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
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
	})
	.bind(("127.0.0.1", 8080))?
	.run()
	.await
}
