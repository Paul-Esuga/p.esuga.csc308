pub fn tasks() {
  // Task 1
	let mut message = String::from("Hello");
	
	show_message(&message);
	add_note(&mut message);
	
	println!("Final message: {}", message);


	// Task 2
	let mut name = String::from("Ada");
	
	print_name(&name);
	append_title(&mut name);
	
	println!("Final name: {}", name);


	// Task 3
	let mut name = String::from("Firstname ");
	add_surname_to_firstname(&mut name);
	println!("{name}");


	// Task 5
	let s1 = String::from("Hi");
	let s2 = String::from("amazing!");
	
	let result = longest(&s1, &s2);
	println!("The longer string is: {}", result);

}


// Task 1 functions
fn show_message(msg: &String) {
	println!("Current message: {}", msg);
}

fn add_note(msg: &mut String) {
  msg.push_str(", world!");
}


// Task 2 fucntions
fn print_name(n: &String) {
    println!("Name: {}", n);
}

fn append_title(n: &mut String) {
    n.push_str(" Lovelace");
}

// Task 3 fucntions
fn add_surname_to_firstname(name: &mut String) {
    name.push_str("Lastname");
}

// Task 5 functions
fn longest(a: &String, b: &String) -> &String {
    if a.len() > b.len() {
        a
    } else {
        b
    }
}


