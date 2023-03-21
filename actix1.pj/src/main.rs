use std::collections::HashMap;
use actix_web::{get, web, App, HttpServer, Responder, HttpResponse};
use serde::Serialize;

// Queue struct

struct Queue
{
	data: Vec<String>,
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
	fn get(&self,index: usize) -> String
	{
		if self.data.len()==0
		{
			"".to_string()
		}
		else if self.data.len()>index || self.data.len()==index
		{
			"".to_string()
		}
		else
		{
			let v=&self.data[index];
			v.to_string()
		}
	}
	fn add(&mut self,value: String)
	{
		self.data.push(value);
	}
	fn bail(&mut self) -> bool
	{
		if self.data.len()==0
		{
			false
		}
		else
		{
			self.data.pop();
			true
		}
	}
	fn kick(&mut self,index: usize) -> bool
	{
		if self.data.len()==0
		{
			false
		}
		else
		{
			let val: String=self.get(index);
			if val.len()==0
			{
				false
			}
			else
			{
				true
			}
		}
	}
	fn prog(&mut self) -> bool
	{
		self.kick(0)
	}
}

// Main Data struct

struct TheData
{
	quecol: HashMap<String,Queue>,
}

// JSON Responses

#[derive(Serialize)]
struct ResultOf_any
{
	msg:String,
}

#[derive(Serialize)]
struct ResultOf_get_names {
	queues: Vec<String>,
}

// Handlers

#[get("/")]
async fn get_state() -> impl Responder
{
	"RUNNING_SMOOTH"
}

#[get("/allnames")]
async fn get_names(data: web::Data<TheData>) -> HttpResponse
{
	let all_queues=&data.quecol;
	let the_names: Vec<String>=Vec::new();
	let mut st:u32=0;
	if the_names.len()>0
	{
		for key in all_queues.keys()
		{
			the_names.push(key.to_string());
		};
		println!("→ Sending back:\n  Queue names: {:?}",the_names);
		//Ok(web::Json( ResultOf_get_names { queues: the_names } ))
		st=200;
	}
	else
	{
		//Ok(web::Json( ResultOf_any { msg: "ZERO_QUEUES".to_string() } ))
		//let response=HttpResponse::new(400);
		st=400;
	};
	HttpsResponse::Ok().status(st).json( ResultOf_get_names { queues: the_names } )
}

#[get("/que/{name}")]
async fn get_queue(name: web::Path<String>) -> impl Responder
{
	format!("Requested all items from the queue \"{}\"",&name)
}

#[get("/que/{name}/{index}")]
async fn get_index(values: web::Path<(String,u32)>) -> impl Responder
{
	let (name,index)=values.into_inner();
	format!("Requested item at position {} from the queue \"{}\"",index,name)
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
		)
		.bind(("127.0.0.1", 8080))?
		.run()
		.await
}
