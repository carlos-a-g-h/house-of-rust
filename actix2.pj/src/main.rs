use std::path::{self, Path, PathBuf};

use actix_files as fs;
use actix_web::http::header::{ContentDisposition, DispositionType};
use actix_web::{get, App, Error, HttpRequest, HttpServer, HttpResponse};

// async fn index(req: HttpRequest) -> Result<fs::NamedFile, Error> {

#[get("/{filename:.*}")]
async fn index(req: HttpRequest) -> HttpResponse {
	/*
	let path: PathBuf = req.match_info().query("filename").parse().unwrap();
	let file = fs::NamedFile::open_async(path).await.unwrap();
	Ok(file
		.use_last_modified(true)
		.set_content_disposition(ContentDisposition {
			disposition: DispositionType::Attachment,
			parameters: vec![],
	}))*/
	match req.match_info().query("filename").parse()
	{
		Ok(fse)=> {
			if !fse.exists() { "The path does not exist" } else
			{
				if fse.is_dir() { "it's a directory" } else { "it's a file" }
			}
		},_=>"Not a path",
	}
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	HttpServer::new(|| App::new()
		.service(index))
		.bind(("127.0.0.1", 8080))?
		.run()
		.await
}
