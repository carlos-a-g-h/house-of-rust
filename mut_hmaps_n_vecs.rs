use std::collections::HashMap;

fn main()
{
	let mut mymap:HashMap<String,Vec<String>>=HashMap::new();
	mymap.insert(String::from("Old_Key"),vec![String::from("Current thing")]);
	println!("Before: {:?}",mymap);
	{
		let kn=String::from("Some Key");
		// let kn=String::from("Old_Key");
		let val1=String::from("some new thing");
		let val2=String::from("some other new thing");
		match mymap.get_mut(&kn)
		{
			Some(found)=>{
				found.push(val1);
				found.push(val2);
			},
			None=>{
				println!("Creating non-existing key: {}",kn);
				let mut newvec:Vec<String>=Vec::new();
				newvec.push(val1);
				newvec.push(val2);
				mymap.insert(kn,newvec);
			},
		};
	};
	println!("After: {:?}",mymap);
}
