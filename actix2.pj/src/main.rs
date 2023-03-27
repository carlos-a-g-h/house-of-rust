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

fn does_it_exist(filepath: &PathBuf) -> Result<(),HttpNegHTML>
{
	if filepath.exists() { Ok(()) } else { Err( HttpNegHTML { txt:"PATH NOT FOUND".to_string(),sc:404 } ) }
}

fn fromreq_get_fse(req: &HttpRequest) -> Result<PathBuf,HttpNegHTML>
{
	let path_raw:String={
		let fromreq_raw=req.match_info().query("filepath");
		format!("./{}",fromreq_raw)
	};
	let fse:PathBuf=match path_raw.parse()
	{
		Ok(v)=>v,
		_=>return Err( HttpNegHTML { txt:"THAT IS NOT A PATH".to_string(),sc:403 } ),
	};
	does_it_exist(&fse)?;
	Ok(fse)
}

#[get("/")]
async fn mainpage() -> HttpResponse
{
	htmlres(200,"Welcome".to_string())
}

#[get("/view/{filepath:.*}")]
async fn fse_viewer(req: HttpRequest) -> Result<HttpResponse,HttpNegHTML>
{
	let fse=fromreq_get_fse(&req)?;
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

#[get("/goto/{filepath:.*}")]
async fn fse_goto(req: HttpRequest) -> Result<HttpResponse,HttpNegHTML>
{
	let fse=fromreq_get_fse(&req)?;
	if !fse.is_dir()
	{
		return Err( HttpNegHTML { txt:"This is not a directory".to_string(),sc:403 } );
	};
	let mut ls_dirs:String=String::new();
	let mut ls_files:String=String::new();
	for entry in fse.read_dir().expect("what")
	{
		if let Ok(entry) = entry
		{
			// println!("{:?} {}",Path::new("/").join(entry.path()),entry.path().is_dir());
			//let p=Path::new("/").join(entry.path());
			let tmpstr:String=format!("\n{}",entry.path().display());
			if entry.path().is_dir() {
				ls_dirs=ls_dirs+&tmpstr;
			} else {
				ls_files=ls_files+&tmpstr;
			};
		}
	};
	Ok( htmlres(200, format!("Contents of:\n{}\n\nDirs:\n{}\n\nFiles:\n{}\n",fse.display(),ls_dirs,ls_files) ) )
}

/*
#[get("/download/{filepath:.*}")]
async fn fse_download(req: HttpRequest) -> Result<fs:NamedFile,HttpNegHTML>
{
	let fse=fromreq_get_fse(&req)?;
	if !fse.is_file()
	{
		return Err( HttpNegHTML { txt:"This is not a file".to_string(),sc:403 } );
	};
	let file=fs::NamedFile::open_async(path).await;
	Ok(file
		.use_last_modified(true)
		.set_content_disposition(ContentDisposition {
			disposition: DispositionType::Attachment,
			parameters: vec![],
		}))
}
*/

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	HttpServer::new(|| App::new()
		.service(mainpage)
		.service(fse_viewer)
		.service(fse_goto)
		)
		.bind(("127.0.0.1", 8080))?
		.run()
		.await
}

/*

#![allow(unused)]
fn main() {
	use std::path::{Path,PathBuf};
	let path = Path::new("./");
	let lsout:String=String::new();
	for entry in path.read_dir().expect("what")
	{
		if let Ok(entry) = entry
		{
			println!("{:?} {}",Path::new("/").join(entry.path()),entry.path().is_dir());
		}
	}
}

*/
