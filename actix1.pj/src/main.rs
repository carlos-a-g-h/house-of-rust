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

#[get("/")]
async fn index() -> impl Responder
{
	format!("{}",INDEX_HTML)

#[get("/queues")]
async fn get_queues() -> impl Responder
{
	"Requested all queues"
}

#[get("/queues/{name}")]
async fn get_one_queue(name: web::Path<String>) -> impl Responder
{
	format!("Requested all items from the store \"{}\"",&name)
};

#[get("/queues/{name}/{index}")]
async fn get_index_from_queue(values: web::Path<(String,u32)>) -> impl Responder
{
	let (name,index)=values.into_inner();
	format!("Requested item at position {} from the store \"{}\"",index,name)
}

#[actix_web::main]
async fn main() -> std::io::Result<()>
{
	const INDEX_HTML: &str="
	<!DOCTYPE html>
	<!--
		WORK IN PROGRESS!
	-->
	<html>
		<head>
			<title>Queues server</title>
		</head>
		<body>

		<h1>Queues server</h1>
		<p>Manage your queues</p>
		<p>Source code <a href=\"https://github.com/carlos-a-g-h/house-of-rust/tree/main/actix1.pj\">here</a></p>

		</body>
	</html>
	";

	println!("Listening at 8080...");
	HttpServer::new(|| App::new()
		.service(index)
		.service(get_queues)
		.service(get_one_queue)
		.service(get_index_from_queue)
		)
		.bind(("127.0.0.1", 8080))?
		.run()
		.await
}
