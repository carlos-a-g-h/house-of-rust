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

	fn pop(&mut self)
	{
		if self.data.len()>0
		{
		    self.data.pop();
		};
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
	q.push(String::from("thing 1"));
	q.print();
	q.push(String::from("thing 2"));
	q.print();
	q.push(String::from("thing 3"));
	q.print();
	println!("popping out the last element");
	q.pop();
	q.print();
}
