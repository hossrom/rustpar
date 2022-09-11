// used to create custom data type


	
	//traditional struct
//	struct Color{
//		red:u8,
//		green: u8,
//		blue:u8,
//	}
	//tuple struct
	
//	struct Color (u8, u8,u8);
////	
//	pub fn run()
//		{
//		let mut c = Color{
//			
//		
//		red:255,
//		green:0,
//		blue:0
//						};
//						
//		 c.red = 200;
//	println!("color:{} {} {}", c.red,c.green,c.blue);
//	
//		}
//pub fn run() {
//		let mut c =Color(255, 0, 0);
//		c.0= 190;
//		
//		println!("color: {} {} {}", c.0, c.1, c.2);
////
//	}

struct Person {
	first_name:String,
	last_name: String
			  }
	
	impl Person{
		
	//construct a person
	fn new (first: &str, last: &str) -> Person {
		Person{
			first_name: first.to_string(), // we don't do (;) here only (,)
			last_name: last.to_string()
			}
											}
				fn full_name(&self)-> String {
					format!( " {} {}", self.first_name, self.last_name)
				}
				
				// set last name
				fn set_last_name(&mut self, last: &str)
				{
					self.last_name = last.to_string();
				}
				//Name to tuple
				fn to_tuple(self)-> (String, String){
					(self.first_name, self.last_name) // no semicolon because we are returning it
				}
				}
	
	pub fn run()
	{ 
		let mut p = Person::new("mary","henry");
		p.set_last_name("williams");
		println! ("Person {} ", p.full_name()); // for calling methods we need to make sure we are having paranthesis
		println!("Person Tuple {:?}", p.to_tuple()); //for tuples we have to use the debug {:?}
		
	}
	
				