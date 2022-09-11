//Arrays - fixed list where elemens are the same data types

use std::mem;

pub fn run ()
{
	let mut numbers: [i32;4] = [1,2,3,4];
	
	println!("{:?}", numbers);
	
	//get single value 
	println!("Single value : {}", numbers[0]); 
	
	//reassign value 
	numbers[2]=20;
	
	//array len
	println!("Array length: {}", numbers.len());
	
	//bytes occupied
	println!("array occupies{} bytes", mem::size_of_val(&numbers));
	
	//Get slice
	let slice: &[i32] = &numbers[0..3 ];
	println!("slice: {:?}",slice);
}