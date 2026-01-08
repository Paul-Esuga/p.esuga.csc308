pub fn tasks() {
    let factorial = |n: u64| -> u64 {
        (1..=n).product()
    };

    let num = 5;
    println!("Factorial of {} is {}", num, factorial(num));

}
