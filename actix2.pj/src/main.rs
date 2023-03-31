use std::path::{Path, PathBuf};

use actix_files as fs;
use actix_web::{get, App, error, HttpRequest, HttpServer, HttpResponse};
use actix_web::http::StatusCode;
use actix_web::http::header::{ContentDisposition, DispositionType};

use derive_more::{Display, Error};
use normalize_path::NormalizePath;

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

fn get_client_ip(req: &HttpRequest) -> String
{
	match req.peer_addr()
	{
		Some(val)=>format!("{}",val),
		None=>"Unknown".to_string(),
	}
}

fn get_path_name(fp:PathBuf) -> String
{
	match fp.file_name()
	{
		Some(namae)=>format!("{}",{
			let a=namae.to_os_string();
			match a.into_string()
			{
				Ok(the_string)=>the_string,
				Err(_)=>fp.as_path().display().to_string(),
			}
		}),

	None=>fp.as_path().display().to_string(),
	}
}

fn path_to_url(fp: PathBuf) -> String
{
	let prefix={ if fp.is_dir() {"/goto" } else if fp.is_file() {"/download" } else { "/view" } };
	let np={ let p=Path::new(prefix).join(fp);p.normalize() };
	let a_href=format!("{}",&np.display());
	let a_intext=format!("{}",get_path_name(np));
	format!("\n<p><a href=\"{}\">{}</a></p>",a_href,a_intext)
}

fn assert_exists(filepath: &PathBuf) -> Result<(),HttpNegHTML>
{
	if filepath.exists() { Ok(()) } else { Err( HttpNegHTML { txt:"PATH NOT FOUND".to_string(),sc:404 } ) }
}

fn assert_isfile(filepath: &PathBuf) -> Result<(),HttpNegHTML>
{
	if filepath.is_file() { Ok(()) } else { Err( HttpNegHTML { txt:"NOT A FILE".to_string(),sc:403 } ) }
}

fn assert_isdir(filepath: &PathBuf) -> Result<(),HttpNegHTML>
{
	if filepath.is_dir() { Ok(()) } else { Err( HttpNegHTML { txt:"NOT A DIRECTORY".to_string(),sc:403 } ) }
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
	assert_exists(&fse)?;
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
	Err( HttpNegHTML { txt:"what the hell is that???".to_string(),sc:400 } )
}

#[get("/goto/{filepath:.*}")]
async fn fse_goto(req: HttpRequest) -> Result<HttpResponse,HttpNegHTML>
{
	let fse=fromreq_get_fse(&req)?;
	assert_isdir(&fse)?;
	let mut ls_dirs:String=String::new();
	let mut ls_files:String=String::new();
	for entry in fse.read_dir().expect("what")
	{
		if let Ok(entry) = entry
		{
			let entry_path_copy=entry.path().clone();
			// let tmpstr=path_to_url(entry.path());
			if entry_path_copy.is_dir() {
				// ls_dirs=ls_dirs+&path_to_url(entry.path());
				ls_dirs=format!( "{}{}",ls_dirs.clone(),path_to_url(entry.path()) );
			} else {
				// ls_files=ls_files+&path_to_url(entry.path());
				ls_files=format!( "{}{}",ls_files.clone(),path_to_url(entry.path()) );
			};
		}
	};

	// https://docs.rs/normalize-path/latest/normalize_path/
	// https://doc.rust-lang.org/stable/std/path/
	// https://doc.rust-lang.org/stable/std/path/struct.PathBuf.html
	// https://doc.rust-lang.org/stable/std/path/struct.Path.html
	// https://doc.rust-lang.org/stable/std/string/struct.String.html

	// https://doc.rust-lang.org/std/ffi/struct.OsStr.html
	// https://doc.rust-lang.org/std/ffi/struct.OsString.html

	let html_parent_dir:(String,String)={
		let fallback:(String,String)=( "/".to_string() , "Go to the home page".to_string() );
		match fse.parent()
		{
			Some(fse_parent)=>{
				let fse_parent_norm=fse_parent.normalize();
				let fse_parent_norm_str=format!("./{}",fse_parent_norm.display());
				if fse_parent_norm_str.trim()==""
				{ fallback } else { ( format!("/goto/{}/",fse_parent_norm.display()) , "Go to upper level".to_string() ) }
			},
			None=>fallback,
		}
	};
	Ok( htmlres(200,format!("
<html>
	<body>
		<p><a href=\"{}\">{}</a></p>
		<p>Contents of:</p>
		<p>{}</p>
		<p><br>Directories:</p>
		{}
		<p><br>Files:</p>
		{}
	</body>
</html>",
	html_parent_dir.0,
	html_parent_dir.1,
	{ let f=fse.as_path();f.normalize().display() },
	ls_dirs,ls_files)))
}

#[get("/download/{filepath:.*}")]
async fn fse_download(req: HttpRequest) -> Result<fs::NamedFile,HttpNegHTML>
{
	let fse=fromreq_get_fse(&req)?;
	assert_isfile(&fse)?;
	println!("\n- User {} wants to download:\n  {:?}",get_client_ip(&req),fse.display());
	let file=fs::NamedFile::open_async(fse).await.unwrap();
	Ok(file
		.use_last_modified(true)
		.set_content_disposition(ContentDisposition {
			disposition: DispositionType::Attachment,
			parameters: vec![],
		}))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	println!("Listening at 8080 and printing out some events");
	HttpServer::new(|| App::new()
		.service(mainpage)
		.service(fse_viewer)
		.service(fse_goto)
		.service(fse_download)
		)
		.bind(("127.0.0.1", 8080))?
		.run()
		.await
}
