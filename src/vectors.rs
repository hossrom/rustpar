//vectors - fixed list where elemens are the same data types

use std::mem;

pub fn run ()
{
	let mut numbers: Vec<i32> = vec![1,2,3,4];
	
	println!("{:?}", numbers);
	
	//get single value 
	println!("Single value : {}", numbers[0]); 
	
	//reassign value 
	numbers[2]=20;
	
	//add to vector
	numbers.push(5);
	numbers.push(87);
	
	//vector len
	println!("vector length: {}", numbers.len());
	
	//bytes occupied
	println!("vector occupies{} bytes", mem::size_of_val(&numbers));
	
	//Get slice
	let slice: &[i32] = &numbers[0..3 ];
	println!("slice: {:?}",slice);
	
		println!("{:?}", numbers);
	
	//loop through vector values
	for x in numbers.iter()
	{
		println!("Number: {}",x);
		
		
	}
	
	//loop and mutate values
	
for x in numbers.iter_mut()
{
	*x *=2;
}
println!("Numbers vec: {:?}", numbers); //if we are going to print out the whole array we need to put {:?}

}