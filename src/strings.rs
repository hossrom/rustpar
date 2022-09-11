//primitive str= imutable fixed length string somewhere in memory
//String = growable

pub fn run()

{
	let mut hello = String::from("Hello ");
	
	//Get length
	println!("length: {}", hello.len());
	
	//push char
	 hello.push('w');
	
	//push string
	hello.push_str("orld");
	
	//capacity in bytes
	println!("Capacity: {}",hello.capacity());
	
	//check if empty
	println!("is empty {}", hello.is_empty());
	
	//contains
	println!("Contains 'World' {}", hello.contains("world"));
	
	
	//replace
	println!("replace: {}",hello.replace("world","there"));
	
	//loop through strinng by whitespace
	for word in hello.split_whitespace()
	{
		println!("{}", word);
	}
	
	//create a string with capacity
	let mut s = String::with_capacity(10);
	s.push ('a');
	s.push ('b');
	s.push ('b');
	
	//Assertion testing
	assert_eq!(3, s.len());
		assert_eq!(10, s.capacity());

	println!("{}", s);
	
	
}