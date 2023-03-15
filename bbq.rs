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

    fn add_item(&mut self,value: String)
    {
        self.data.push(value);
    }

    fn show(&self)
    {
        println!("Contents of {}: {:?}",self.name,self.data);
    }
}

fn main()
{
    let mut q=Queue::new(String::from("Test"));
    q.show();
    q.add_item(String::from("thing 1"));
    q.show();
    q.add_item(String::from("thing 2"));
    q.show();
}
