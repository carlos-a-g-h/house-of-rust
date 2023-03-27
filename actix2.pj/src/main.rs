use std::path::{self, Path, PathBuf};

use actix_files as fs;
use actix_web::{get, App, Error, HttpRequest, HttpServer, HttpResponse};
use actix_web::http::StatusCode;
use actix_web::http::header::{ContentDisposition, DispositionType};

fn htmlres(sc:u16,text:String) -> HttpResponse
{
	HttpResponse::Ok()
	.status(StatusCode::from_u16(sc).unwrap())
	.insert_header(("Content-Type","text/html"))
	.body( text )
}

fn fromreq_get_fse(req: &HttpRequest) -> Result<PathBuf,HttpResponse>
{
	let path_raw:&str={
		let fromreq_raw=req.match_info().query("filepath");
		format!("./{}",fromreq_raw.as_str())
	};
	match path_raw.parse::<PathBuf>()
	{
		Ok(fse)=>Ok(fse),
		_=>Err( htmlres(403,"THAT IS NOT A PATH".to_string()) ),
	}
}

fn does_it_exist(filepath: &PathBuf) -> Result<(),HttpResponse>
{
	if filepath.exists()
	{ Ok(()) }
	else
	{ Err( htmlres(404,"PATH NOT FOUND".to_string()) ) }
}

#[get("/view/{filepath:.*}")]
async fn fse_view(req: HttpRequest) -> Result<HttpResponse,HttpResponse>
{
	let fse=fromreq_get_fse(&req)?;
	does_it_exist(&fse)?;
	if fse.is_dir()
	{
		return Ok( htmlres(200,"that is a directory".to_string()) );
	};
	if fse.is_file()
	{
		return Ok( htmlres(200,"that is a file".to_string()) );
	};
	Ok ( htmlres(400,"what the hell is that".to_string()) )
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	HttpServer::new(|| App::new()
		.service(fse_view)
		)
		.bind(("127.0.0.1", 8080))?
		.run()
		.await
}

// impl NamedFile
// https://docs.rs/actix-files/latest/src/actix_files/named.rs.html#92
// impl NamedFile (from NamedFile to HttpResponse)
// https://docs.rs/actix-files/latest/src/actix_files/named.rs.html#414

