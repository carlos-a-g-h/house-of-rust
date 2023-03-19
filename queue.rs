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

fn main()
{
	let mut q=Queue::new();
	q.add("item".to_string());
	q.add("item1".to_string());
	q.add("item2".to_string());
	q.prog();
	q.bail();
	q.add("item55".to_string());
	q.add("item45".to_string());
	q.kick(2);
}
