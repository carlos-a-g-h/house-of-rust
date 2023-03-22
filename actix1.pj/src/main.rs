use std::collections::HashMap;
use actix_web::{get, post, delete, web, App, HttpServer, Responder, HttpResponse};
use actix_web::http::StatusCode;
use serde::{Serialize, Deserialize};
use serde_json::json;

// Queue struct

struct Queue
{
	data: Vec<<Vec<String>>,
}

impl Queue
{
	fn new() -> Queue
	{
		Queue
		{
			data: Vec::new(),
		}
	}

	fn get_size(&self) -> usize
	{
		self.data.len()
	}

	fn is_empty(&self) -> bool
	{
		let size:u16=self.get_size as u16;
		if size==0 { true } else { false }
	}

	fn index_exists(&self,index:usize) -> bool
	{
		let size: u16=self.data.get_size() as u16;
		if index>size || size==0 || size==index { false } else { true }
	}

	fn add(&mut self,value: Vec<String>)
	{
		self.data.push(value);
	}

	fn get(&self,index: usize) -> Vec<String>
	{
		if self.index_exists(index) { &self.data[index] } else { Vec::new() }
	}

	fn kick(&mut self,index: usize) -> bool
	{
		if self.index_exists(index)
		{
			&self.data.remove(index);
			true
		}
		else
		{
			false
		}
	}
}

// Main Data struct

struct TheData
{
	quecol: HashMap<String,Queue>,
}

impl TheData
{
	fn get_size(&self) -> usize
	{
		self.quecol.len()
	}

	fn is_empty(&self) -> bool
	{
		let size:u16=self.get_size as u16;
		if size==0 { true } else { false }
	}

	fn if_key(&self,tgt_name: &str) -> bool
	{
		if self.is_empty()
		{
			false
		}
		else
		{
			let found: bool=false;
			for key in &self.quecol.keys()
			{
				if key==&tgt_name
				{
					found=true;
					break;
				};
			}
			found
		}
	}
}

// JSON requests

#[derive(Deserialize)]
struct POST_BringElem
{
	name:String,
	elem:Vec<String>,
}

#[derive(Deserialize)]
struct POST_BringIndex
{
	name:String,
	index:usize,
}

// Handlers

#[get("/")]
async fn get_state() -> HttpResponse
{
	HttpResponse::Ok()
	.status(StatusCode::from_u16(200).unwrap())
	.json( json!({}) )
}

#[get("/all")]
async fn get_names(app_data: web::Data<TheData>) -> HttpResponse
{
	let mut names: Vec<String>=Vec::new();

	let status_code:u16={
		if app_data.is_empty()
		{
			404
		}
		else
		{
			for key in &app_data.quecol.keys()
			{
				names.push(key.to_string());
			};
			200
		}
	};

	HttpResponse::Ok()
	.status(StatusCode::from_u16(status_code).unwrap())
	.json(
		if status_code==200
		{
			json!({ "result":names })
		}
		else
		{
			json!({})
		}
	)
}

#[get("/que/{name}")]
async fn get_queue(name: web::Path<String>,app_data: web::Data<TheData>) -> HttpResponse
{
	let mut result: Vec<Vec<String>>=Vec::new();
	let status_code:u16={
		if app_data.is_empty()
		{
			404
		}
		else
		{
			let sc:u16=match app_data.quecol.get(&name)=>
			{
				Some(queue_found)=>
				{
					for elem in &queue_found.data
					{
						result.push(elem);
					};
					200
				},
				None=>404,
			};
			sc
		}
	}

	HttpResponse::Ok()
	.status(StatusCode::from_u16(status_code).unwrap())
	.json(
		if status_code==200
		{
			json!({ "result":result })
		}
		else
		{
			json!({})
		}
	)
}

#[get("/que/{name}/{index}")]
async fn get_index(from_path: web::Path<(String,usize)>,app_data: web::Data<TheData>) -> HttpResponse
{
	let element:Vec<String>=Vec::new();
	let status_code:u16={
		let (name,index)=from_path.into_inner();
		match app_data.quecol.get(&name) => {
			Some(queue_found) => {
				if queue_found.index_exists(index)
				{
					for e in &queue_found.get(index)
					{
						element.push(e);
					};
					200
				}
				else
				{
					404
				}
			},
			None=>404,
		}
	}

	HttpResponse::Ok()
	.status(StatusCode::from_u16(status_code).unwrap())
	.json( if status_code==200 { json!({ "element":element }) } else { json!({}) } )
}

#[post("/que/{name}/add")]
async fn post_queue(name: web::Path<String>,from_post: web::Json<POST_BringElem>,app_data: web::Data<TheData>) -> HttpResponse
{
	let status_code:u16=200;
	let wutt={ if app_data.is_empty() {false} else {status_code=403;true} };

	if wutt==false
	{
		if from_post.elem.len()==0
		{
			wutt=true;
			status_code=403;
		};
	};

	if wutt==false
	{
		wutt=match app_data.quecol.get(&from_post.name)=> {
			Some(fq) => {
				fq.add(from_post.elem);
				false
			},
			None => {
				status_code=404;
				true
			},
		};
	};

	HttpResponse::Ok()
	.status(StatusCode::from_u16(status_code).unwrap())
	.json(json!({}))
}

// Application setup

#[actix_web::main]
async fn main() -> std::io::Result<()>
{
	println!("Listening at 8080...");
	HttpServer::new(||
		App::new().app_data(
			web::Data::new(
				TheData
				{
					quecol: HashMap::new(),
				})
			)
			.service(get_state)
			.service(get_names)
			.service(get_queue)
			.service(get_index)
			.service(post_queue)
		)
		.bind(("127.0.0.1", 8080))?
		.run()
		.await
}
