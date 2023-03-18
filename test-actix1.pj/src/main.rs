use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

struct AppState {
    app_name: String,
}

#[get("/")]
async fn hello() -> impl Responder {
	HttpResponse::Ok().body("Hello world!")
}

#[get("/app_name")]
async fn get_app_name(data: web::Data<AppState>) -> String {
	let the_name=&data.app_name;
	format!("Hello {the_name}!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
	HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
	HttpResponse::Ok().body("Hey there!")
}

async fn inside_a_scope() -> impl Responder {
	HttpResponse::Ok().body("This is inside some scope")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	HttpServer::new(|| {
		App::new()
			// Decorator-like routing
			.service(hello)
			.service(echo)
			//manual routing
			.route("/hey", web::get().to(manual_hello))
			//Scopes routing
			.service(
				web::scope("/app")
					.route("/index.html",web::get().to(inside_a_scope))
			)
			//Application state example
			.app_data(web::Data::new(AppState{
				app_name: String::from("SOME NICE APP"),
			}))
			//route that shows the application state
			.service(get_app_name)
	})
	.bind(("127.0.0.1", 8080))?
	.run()
	.await
}
