fn grade(score: i32) -> String {
    let mut result= String::from("");
    if score >= 90 {
        result = String::from("Excellent!");
    } else if score >= 70 {
        result = String::from("Good job!");
    } else if score < 40 {
        result = String::from("Needs Remedial Practice.");
    } else {
        result = String::from("Needs improvement");
    }
    return result;
}

fn main() {
    println!("Hello, world!");
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


    let score = 85;
    let result = grade(score);
    println!("Score: {}, Result: {}", score, result);


}
