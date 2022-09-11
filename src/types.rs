/*
primitive types--

Integegers:u8,i8,u16,i16,u32,i32,u64,i64,u128,i128 (number of bits in memory)

Floats:f32, f64

Boolean(bool)
characters(char)
Tuples
Arrays
*/

pub fn run ()
{
	//default is i32
	let x = 1;
	
	//default is f64
	let y=2.5;
	
	//add explicit type
	
let z:i64=454634528959284;

//find max size
println! ("max i32: {}", std::i32::MAX);
println! ("max i64: {}", std::i64::MAX);

//boolean
let is_active: bool = true;

let is_greater: bool = 10<5;

let a1 = 'a';

let face = '\u{1F600}';

println!("{:?}",(x,y,z, is_active, is_greater, a1,face));

}