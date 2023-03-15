struct Queue
{
		name: String,
		data: Vec<String>,
}

impl Queue
{
	fn new(name: String) -> Queue
	{
		Queue
		{
			name: name,
			data: Vec::new(),
		}
	}

	fn push(&mut self,value: String)
	{
		self.data.push(value);
	}

	fn print(&self)
	{
		println!("Queue: {}\n\tContents: {:?}\n",self.name,self.data);
	}
}

fn main()
{
	let mut q=Queue::new(String::from("Test"));
	q.print();
	q.push(String::from("uwu"));
	q.print();
	q.push(String::from("lol"));
	q.print();
}
