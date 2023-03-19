use actix_web::{get, web, App, HttpServer, Responder};

#[get("/")]
async fn root() -> impl Responder
{
	"Welcome to our conglomerate\n\n/store/\n/store/name\n/store/name/index\n\nSource code:\nhttps://github.com/carlos-a-g-h/rusty-yard/blob/main/actix1.pj/src/main.rs"
}

#[get("/stores")]
async fn root_stores() -> impl Responder
{
	"Requested all stores"
}

#[get("/stores/{name}")]
async fn from_store_get_all(name: web::Path<String>) -> Result<String>
{
	Ok(format!("Requested all items from the store \"{}\"",&name))
}

#[get("/stores/{name}/{index}")]
async fn from_store_get_index(values: web::Path<(String,u32)>) -> Result<String>
{
	let (name,index)=values.info_inner();
	Ok(format!("Requested item at position {} from the store \"{}\"",index,name))
}

#[actix_web::main]
async fn main() -> std::io::Result<()>
{
	println!("Listening at 8080...");
	HttpServer::new(|| App::new()
		.service(root)
		.service(root_stores)
		.service(from_store_get_all)
		.service(from_store_get_index)
		)
		.bind(("127.0.0.1", 8080))?
		.run()
		.await
}
