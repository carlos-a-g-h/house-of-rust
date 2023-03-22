use std::collections::HashMap;
use std::sync::Mutex;
use actix_web::{get, post, web, App, HttpServer, HttpResponse};
use actix_web::http::StatusCode;
use serde::{Serialize, Deserialize};
use serde_json::json;

// Queue struct

struct Queue
{
	data: Vec<Vec<String>>,
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
		let size=self.get_size();
		if size==0 { true } else { false }
	}

	fn index_exists(&self,index:usize) -> bool
	{
		let size=self.get_size();
		if index>size || size==0 || size==index { false } else { true }
	}

	fn add(&mut self,value: Vec<String>)
	{
		self.data.push(value);
	}

	fn get(&self,index: usize) -> Vec<String>
	{
		if self.index_exists(index) { self.data[index] } else { Vec::new() }
	}

	fn kick(&mut self,index: usize) -> bool
	{
		if self.index_exists(index)
		{
			self.data.remove(index);
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
		let size:u16=self.get_size() as u16;
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
			let que=&self.quecol;
			for key in que.keys()
			{
				if key==&tgt_name
				{
					found=true;
					break;
				};
			};
			found
		}
	}
}

// Application Data in a Mutex

struct TheAppState
{
	counter: Mutex<TheData>
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
async fn get_status() -> HttpResponse
{
	HttpResponse::Ok()
	.status(StatusCode::from_u16(200).unwrap())
	.json( json!({}) )
}

#[get("/all")]
async fn get_names(app_data: web::Data<TheAppState>) -> HttpResponse
{
	let mut names: Vec<String>=Vec::new();
	let status_code:u16={
		if app_data.is_empty()
		{
			404
		}
		else
		{
			let counter=app_data.counter.lock().unwrap();
			for key in counter.quecol.keys()
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
async fn get_queue(name: web::Path<String>,app_data: web::Data<TheAppState>) -> HttpResponse
{
	let mut result: Vec<Vec<String>>=Vec::new();
	let status_code:u16={
		if app_data.is_empty()
		{
			404
		}
		else
		{
			let counter=app_data.counter.lock().unwrap();
			let que=&counter.quecol;
			let tgt_name=name.into_inner();
			match que.get(&tgt_name)
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
			}
		}
	};

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
async fn get_index(from_path: web::Path<(String,usize)>,app_data: web::Data<TheAppState>) -> HttpResponse
{
	let mut element:Vec<String>=Vec::new();
	let (name,index)=from_path.into_inner();
	let counter=app_data.counter.lock().unwrap();
	let status_code:u16=match counter.quecol.get(&name)
	{
		Some(queue_found) => {
			if queue_found.index_exists(index)
			{
				for e in &queue_found.get(index)
				{
					element.push(e.to_string());
				};
				200
			}
			else
			{
				404
			}
		},
		None=>404,
	};

	HttpResponse::Ok()
	.status(StatusCode::from_u16(status_code).unwrap())
	.json( if status_code==200 { json!({ "element":element }) } else { json!({}) } )
}

#[post("/que/{name}/add")]
async fn post_queue(name: web::Path<String>,from_post: web::Json<POST_BringElem>,&mut app_data: web::Data<TheAppState>) -> HttpResponse
{
	let mut status_code:u16=200;
	let mut counter=app_data.counter.lock().unwrap();
	let mut wutt={ if counter.is_empty() {false} else {status_code=403;true} };

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
		let tgt_name=name.into_inner();
		wutt=match counter.quecol.get(&tgt_name)
		{
			Some(fq) => {
				// let elem=&from_post.elem;
				// fq.add(elem.to_vec());
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
	println!("Running server at port 8080");
	let persistent=web::Data::new(TheAppState{
		counter: Mutex::new( TheData{quecol: HashMap::new()} )
	})
	HttpServer::new(move ||
		App::new()
			.app_data(persistent.clone())
			.service(get_status)
			.service(get_names)
			.service(get_queue)
			.service(get_index)
			.service(post_queue)
		)
		.bind(("127.0.0.1", 8080))?
		.run()
		.await
}
