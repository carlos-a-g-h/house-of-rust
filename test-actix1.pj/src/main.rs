use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
	HttpResponse::Ok().body("Hello world!")
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
			.service(hello)
			.service(echo)
			.route("/hey", web::get().to(manual_hello))
			.service(
				web::scope("/app")
					.route("/index.html",web::get().to(inside_a_scope))
			)
	})
	.bind(("127.0.0.1", 8080))?
	.run()
	.await
}
