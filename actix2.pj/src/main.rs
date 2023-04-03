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

fn html_adeq(body: String) -> String
{
	format!("
<html lang=\"en\">
	<meta charset=\"UTF-8\">
	<meta name=\"viewport\" content=\"width=device-width,initial-scale=1\">
	<head>
		<title>
			rExplorer
		</title>
	</head>
	<body>
		{}
	</body>
</html>
"
	,body)
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

fn fpath_to_url(fp: PathBuf) -> String
{
	let path_type=String::from( { if fp.is_dir() { "/goto" } else if fp.is_file() { "/download" } else { "/view" } } );
	let path_as_url={ let p=Path::new(path_type).join(fp);p.normalize() };
	format!("{}",path_as_url.display())
}

fn fpath_to_html(fp: PathBuf) -> String
{
	/*
	let the_type:(&str,String)={ if fp.is_dir() { ("/goto",String::from("📁")) } else if fp.is_file() { ("/download",String::from("📄")) } else { ("/view",String::from("❓")) } };
	let (the_prefix,the_icon)=the_type;
	let np={ let p=Path::new(the_prefix).join(fp);p.normalize() };
	let a_href=format!("{}",&np.display());
	let a_intext=format!("{}",get_path_name(np));
	*/
	let the_icon={ if fp.is_dir() { String::from("📁") } else if fp.is_file() { String::from("📄") } else { String::from("❓") } };
	let a_href=fpath_to_url(fp.clone());
	let a_intext=format!("{}",get_path_name(fp));
	format!("\n<p><strong><code>{} <a href=\"{}\">{}</a></strong></code></p>",the_icon,a_href,a_intext)
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

	let mut html_body=String::new();

	//Link to parent directory or homepage
	{
		let link_to_back_or_upper:(String,String,String)={
			let fallback:(String,String,String)=( String::from("🏠") , String::from("/") , String::from("Go to the home page") );
			let fse_norm={ let p=fse.as_path();p.normalize() };
			let fse_norm_str=format!("{}",fse_norm.display());
			if fse_norm_str.trim()=="" { fallback } else
			{
				match fse_norm.parent()
				{
					None=>fallback,
					Some(the_parent)=>
					{
						let parent_str=format!("{}",the_parent.display());
						let ulevel=String::from("Go to upper level");
						let uicon=String::from("⬆️");
						if parent_str.trim()=="" { ( uicon,String::from("/goto/"),ulevel ) } else
						{ (
							uicon,
							{ let the_string=format!("/goto/{}/",parent_str);if &the_string=="/goto//" { String::from("/goto/") } else { the_string } },
							ulevel
						) }
					},
				}
			}
		};
		let (emoji,a_href,a_innertext)=link_to_back_or_upper;
		html_body=format!("
			<h3>{} <a href=\"{}\">{}</a></h3>",emoji,a_href,a_innertext);
	};

	// Files and directories
	{
		let mut count_f=0;
		let mut count_d=0;
		let mut the_dirs:String=String::new();
		let mut the_files:String=String::new();
		for entry in fse.read_dir().expect("what")
		{
			if let Ok(entry) = entry
			{
				let entry_path_copy=entry.path().clone();
				if entry_path_copy.is_dir() {
					the_dirs=format!( "{}{}" ,the_dirs,fpath_to_html(entry.path()) );
					count_d=count_d+1;
				} else {
					the_files=format!( "{}{}" ,the_files,fpath_to_html(entry.path()) );
					count_f=count_f+1;
				};
			}
		};
		html_body=format!("{}\n\t\t{}",html_body, String::from( { if count_d>0 || count_f>0 { "<h2>Contents</h2>" } else { "<p><br>Empty</p>" } } ) );
		if count_d>0
		{
			html_body=format!("{}\n\t\t<h3>Directories</h3>\n{}",html_body,the_dirs);
		};
		if count_f>0
		{
			html_body=format!("{}\n\t\t<h3>Files</h3>\n{}",html_body,the_files);
		};
	};

	// Actions
	/*
	if count_f>1
	{
		
	};*/

	Ok( htmlres(200, html_adeq(html_body) ) )
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
