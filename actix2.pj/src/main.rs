use std::path::{self, Path, PathBuf};

use actix_files as fs;
use actix_web::{get, App, error, HttpRequest, HttpServer, HttpResponse};
use actix_web::http::StatusCode;
use actix_web::http::header::{ContentDisposition, DispositionType};
use derive_more::{Display, Error};

#[derive(Debug, Display, Error)]
#[display(fmt = "{}", txt)]
struct HttpNegHTML { txt:String, sc:u16 }

impl error::ResponseError for HttpNegHTML
{
	fn status_code(&self) -> StatusCode { StatusCode::from_u16(self.sc).unwrap() }

	fn error_response(&self) -> HttpResponse
	{
		HttpResponse::Ok()
		.status(self.status_code())
		.insert_header(("Content-Type","text/html"))
		.body( self.txt.clone() )
	}
}

fn htmlres(sc:u16,text:String) -> HttpResponse
{
	HttpResponse::Ok()
	.status(StatusCode::from_u16(sc).unwrap())
	.insert_header(("Content-Type","text/html"))
	.body( text )
}

fn fromreq_get_fse(req: &HttpRequest) -> Result<PathBuf,HttpNegHTML>
{
	let path_raw:String={
		let fromreq_raw=req.match_info().query("filepath");
		format!("./{}",fromreq_raw)
	};
	match path_raw.parse::<PathBuf>()
	{
		Ok(fse)=>Ok(fse),
		_=>Err( HttpNegHTML { txt:"THAT IS NOT A PATH".to_string(),sc:403 } ),
	}
}

fn does_it_exist(filepath: &PathBuf) -> Result<(),HttpNegHTML>
{
	if filepath.exists() { Ok(()) } else { Err( HttpNegHTML { txt:"PATH NOT FOUND".to_string(),sc:404 } ) }
}

#[get("/view/{filepath:.*}")]
async fn fse_view(req: HttpRequest) -> Result<HttpResponse,HttpNegHTML>
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
	Err( HttpNegHTML { txt:"what the hell is that".to_string(),sc:400 } )
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

