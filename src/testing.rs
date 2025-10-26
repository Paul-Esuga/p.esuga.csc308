fn grade(score: i32) -> String {
    if score >= 90 {
        String::from("Excellent!")
    } else if score >= 70 {
        String::from("Good job!")
    } else if score < 40 {
        String::from("Needs Remedial Practice.")
    } else {
        String::from("Needs improvement")
    }
}

fn add() -> i32 {
    2 + 2
}
fn add_num(a: i32, b: i32) -> i32 {
    a + b
}

// figure out what a funcion is and what a macro is in rust

// fn add_string(a: String, b: String) -> String {
//     a + &b
// }

pub fn test() {
    let x = 5;
    let mut y = 10;
    println!("x: {}, y: {}", x, y);
    y = 15;
    println!("Updated y : {}", y);

    // let age: i32 = 25;
    // let price: f64 = 19.99;
    // let is_member: bool = true;
    // let letter: char = 'R';
    // let name: &str = "Rustacean";

    // let person: (i32, f64, char) = (30, 5.9, 'M');
    let _scores: [i32; 4] = [10, 20, 30, 40];

    let mut name = String::from("");
    println!("Hello, {}", name);
    name.push_str("acean");
    println!("Updated: {}", name);

    let age: [i32; 8] = [4; 8];
    println!("Age array: {:?}", age);

    // let x = String::from("hi");
    // let y = x.clone();
    let x = add();
    let y = add_num(2, 5);
    println!("x: {}, y: {}", x, y);

    for i in 1..=5 {
        print!("Count: {}\n", i)
    }


    let score = 35;
    let result = grade(score);
    println!("Score: {}, Result: {}", score, result);


}
