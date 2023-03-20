use std::collections::HashMap;
use actix_web::{get, web, App, HttpServer, Responder};
use serde::Deserialize;

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

// Types of response

#[derive(Serialize)]
struct Qeue {
    name: String,
}

// Handlers

#[get("/")]
async fn get_status() -> impl Responder
{
	"RUNNING_SMOOTH"
}


#[derive(Deserialize)]
struct ResultOf_get_all {
    queues: Vec<String>,
}

#[get("/all")]
async fn get_names(data: web::Data<TheData>) -> impl Responder
{
	let all_queues=&data.quecol;
	Ok(web::Json(
		ResultOf_get_all { queues: all_queues.keys() }
	))
}

#[get("/que/{name}")]
async fn get_que(name: web::Path<String>) -> impl Responder
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
			.service(get_status)
			.service(get_names)
			.service(get_que)
			.service(get_from_queue)
		)
		.bind(("127.0.0.1", 8080))?
		.run()
		.await
}
