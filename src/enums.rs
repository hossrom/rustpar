//enums holds a few definite values

enum movement{
	// variants
	
up, 
down,
right,
left
}

fn move_avatar (m:movement) {
	// performs actions depending on statement
	
	match m {
	movement::up=> println!("Avatar moving up"),
	movement::down=> println!("Avatar moving down"),
	movement::left=> println!("Avatar moving left"),
	movement::right=> println!("Avatar moving rright")
	}

}

pub fn run(){
	
	let avatar1 = movement::left;
	let avatar2 = movement::up;
	let avatar3 = movement::right;
	let avatar4 = movement::down;
	
	move_avatar(avatar1);
	move_avatar(avatar2);
	move_avatar(avatar3);
	move_avatar(avatar4);
	
}