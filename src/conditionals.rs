//conditionals


pub fn run()
{
	let age:u8 =18;
	let check_id:bool = false;	
	let knows_person = true;
	
	//if-else
	
	if age>=21 &&check_id || knows_person
	
	{
		println!("Bartender: what would you like to drink");
	}
	else if age<21 && age >=18 &&check_id|| knows_person{
	println!("Bartender: hmm let's see");
	}
	else
	{
	println!("securityy");
	}
	
	//short hand if
	let is_of_age = if age >=21 {true} else {false};
	println!("is of age:{}", is_of_age);
}