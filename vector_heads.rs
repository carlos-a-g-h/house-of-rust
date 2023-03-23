fn have_same_head(elem_a:&Vec<String>,elem_b:&Vec<String>) ->bool
{
	let head_a=elem_a.first().unwrap();
	let head_b=elem_b.first().unwrap();
	if head_a==head_b { true } else { false }
}

fn main()
{
	let e1:Vec<String>=vec![String::from("head1"),String::from("bb")];
	let e2:Vec<String>=vec![String::from("head2"),String::from("body"),String::from("tail")];
	println!("same head? {}",have_same_head(&e1,&e2));
}
