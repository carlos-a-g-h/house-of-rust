macro_rules! capitalize
{
	($a: expr)=>
	{
		let mut v: Vec <char> =$a.chars().collect();
		v[0]=v[0].to_uppercase().nth(0).unwrap();
		$a=v.into_iter().collect();
	}
}

fn main()
{
	let mut x=String::from("test");
	println!("before the macro: {}",&x);
	capitalize!(x);
	println!("after the macro: {}",x);
}
