fn test(a: i32) -> i32
{
	if a > 0
	{
		a
	}
	else
	{
		-a
	}
}

fn test2(a :i32) -> i32
{
	if a > 0 
	{
		a
	}
	else
	{
		-1
	}
}

fn main() {
	let a = test(-1);
	println!("{}", a);
}
