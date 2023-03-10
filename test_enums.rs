enum Number
{
	F(f32),
	I(i32),
}

fn prep_number(data: &Number) -> f32
{
	return match data
	{
		Number::F(value)=>*value,
		Number::I(value)=>*value as f32,
	};
}

fn main()
{
	let a=Number::F(3.5);
	let b=Number::I(5);
	println!("{}",prep_number(&a)+prep_number(&b));
}
