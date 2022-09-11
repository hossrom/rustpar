//reference pointer- point to a resource in memory

pub fn run()
{
	// primitive array
	
	let arr1 = [1,2,3];
	let arr2 = arr1;
	
	// with non primitive, if you assign another variable or piece of data then the first value will no longer hold that value.
	//you will need to use a reference(&) to point to the resource.
	
	
	//vector
	let vec1 = vec![1,8, 3];
	let vec2 = &vec1;
	
	println!("values {:?}" ,(&vec1, &vec2));
	
}