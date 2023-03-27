use std::path::{self, Path, PathBuf};

use actix_files as fs;
use actix_web::{get, App, Error, HttpRequest, HttpServer, HttpResponse};
use actix_web::http::StatusCode;
use actix_web::http::header::{ContentDisposition, DispositionType};

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
	// let diag:String=match req.match_info().query("filename").parse::<F>()
	let diag:String=match req.match_info().query("filename").parse::<PathBuf>()
	{
		Ok(fse)=> {
			if !fse.exists()
			{
				println!("\nThe path\n{:?}\ndoes not exist",&fse);
				"non existent".to_string()
			}
			else
			{
				if fse.is_dir()
				{
					println!("\nThe path\n{:?}\nis a directory",&fse);
					"a dir".to_string()
				}
				else
				{
					println!("\nThe path\n{:?}\nis a file",&fse);
					"a file".to_string()
				}
			}
		},
		_=>{
			println!("\nnot a path");
			"error".to_string()
		},
	};
	HttpResponse::Ok()
	.status(StatusCode::from_u16(200).unwrap())
	.insert_header(("Content-Type","text/html"))
	.body( diag )
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	HttpServer::new(|| App::new()
		.service(index))
		.bind(("127.0.0.1", 8080))?
		.run()
		.await
}

// impl NamedFile
// https://docs.rs/actix-files/latest/src/actix_files/named.rs.html#92
// impl NamedFile (from NamedFile to HttpResponse)
// https://docs.rs/actix-files/latest/src/actix_files/named.rs.html#414

