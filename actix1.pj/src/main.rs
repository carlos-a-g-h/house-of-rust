use std::collections::HashMap;
use actix_web::{get, web, App, HttpServer, Responder};

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

struct QColl {
	app_name: String,
}

#[get("/")]
async fn root() -> impl Responder
{
	"Source code:\nhttps://github.com/carlos-a-g-h/rusty-yard/blob/main/actix1.pj/src/main.rs"
}

#[get("/queues")]
async fn root_stores() -> impl Responder
{
	"Requested all queues"
}

#[get("/queues/{name}")]
async fn from_store_get_all(name: web::Path<String>) -> impl Responder
{
	format!("Requested all items from the store \"{}\"",&name)
}

#[get("/queues/{name}/{index}")]
async fn from_store_get_index(values: web::Path<(String,u32)>) -> impl Responder
{
	let (name,index)=values.into_inner();
	format!("Requested item at position {} from the store \"{}\"",index,name)
}

#[actix_web::main]
async fn main() -> std::io::Result<()>
{
	println!("Listening at 8080...");
	HttpServer::new(|| App::new()
		.service(root)
		.service(root_stores)
		.service(from_store_get_all)
		.service(from_store_get_index)
		)
		.bind(("127.0.0.1", 8080))?
		.run()
		.await
}
