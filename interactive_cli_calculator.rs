use std::io;

fn from_stdin(title: &str) -> Option<f32>
{
	println!("{}",title);

	let mut data_raw: String=String::new();
	let mut data:f32=0.0;

	let mut stat:bool=false;
	let mut quit:bool=false;

	loop
	{
		io::stdin()
			.read_line(&mut data_raw)
			.expect("wutt");

		stat=match data_raw.trim().parse()
		{
			Ok(num)=>
			{
				quit=true;
				data=num;
				true
			},
			Err(_)=>
			{
				let tmp=data_raw.trim();
				println!("NaN: {}",tmp);
				if tmp=="q"
				{
					quit=true;
				}
				else
				{
					println!("Try again. If you wanna get out, just type 'q'");
				};
				false
			},
		};
		data_raw.clear();
		if quit
		{
			break stat;
		};
	};

	return match stat
	{
		true => Some(data),
		false => None,
	};
}

fn op_sim(oper: &str) -> bool
{
	let texts:(&str, &str, &str, &str)=match oper
	{
		"div"=>("Division","Denominator","Numerator","Denom/Numer"),
		"diff"=>("Difference","Minuend","Sustraend","Min/Sust"),
		"sum2"=>("Sum btwn 2 numbers","Data1","Data2","Data1+Data2"),
		"mul2"=>("Mult btwn 2 numbers","Factor1","Factor2","Factor1 * Factor2"),
		_=>("","","",""),
	};

	println!("Math operation: {}",texts.0);

	let a:Option<f32>=from_stdin(texts.1);
	if a==None
	{
		return false;
	};

	let b:Option<f32>=from_stdin(texts.2);
	if b==None
	{
		return false;
	};

	let result:f32=match oper
	{
		"div"=>a.unwrap_or(0.0)/b.unwrap_or(0.0),
		"diff"=>a.unwrap_or(0.0)-b.unwrap_or(0.0),
		"sum2"=>a.unwrap_or(0.0)+b.unwrap_or(0.0),
		"mul2"=>a.unwrap_or(0.0)*b.unwrap_or(0.0),
		_=>0.0,
	};

	println!("{} = {}",texts.3,result);
	true
}

fn main() -> ()
{

	println!("\nInteractive CLI calculator made in Rust");

	let mut text_in:String=String::new();
	let mut quit:bool=false;
	loop
	{

		println!("\nType a command\nValid commands are: sum2, mul2, div, diff, q");

		io::stdin()
			.read_line(&mut text_in)
			.expect("wutt");

		let cmd=text_in.trim().to_string();

		println!("\nYou typed: {}",cmd);

		if cmd=="q"
		{
			quit=true;
		}
		else
		{
			match op_sim(&cmd)
			{
				true =>{println!("...")},
				false =>{println!("Cancelled")},
			};
		};

		text_in.clear();

		if quit
		{
			break;
		};
	};

	println!("Bye!");
}
