use std::io;

fn fah_to_cels(a: f64)-> f64 {
  let x: f64 = 5.0 / 9.0;
  return (a - 32.0) * (x);
}

fn cels_to_fah(a: f64)-> f64 {
  return ((9.0 * a) / 5.0) + 32.0;
}

fn bills_discount(a: i32) {
  let mut discount: f64 = 1.0;
  if a > 10000 {
      discount = 0.85
  } else if a > 5000 {
      discount = 0.9
  }

  let dis = if discount < 0.9 {15} else if discount < 1.0 {10} else {0};
  println!("Original bill: N{} \nDiscount applied: {}% \nFinal bill: N{}\n\n", a, dis, ((a as f64) * discount))
}

fn calc_elec_usage(usg: f32) -> f32 {
    let rates = if usg > 200.0 {
        30.0
    } else if usg > 100.0 {
        25.0
    } else {
        20.0
    };

    usg * rates
}

pub fn tasks() {
  // Celsius to Fahrenheit Converter

  let fah = 68.0;
  let cels = fah_to_cels(fah);
  print!("Temperature: {} *F \nConverted: {} *C\n", fah, cels);

  let cels1 = 100.0;
  let fah1 = cels_to_fah(cels1);
  print!("Temperature: {} *C \nConverted: {} *F\n\n", cels1, fah1);


  // Customer discount calculator

  bills_discount(15500);


  // Electricity usage bill

    println!("Enter your energy consumption in kWh:");

    let mut inp = String::new();
    io::stdin()
        .read_line(&mut inp)
        .expect("Failed to read input");

    let usg: f32 = inp.trim().parse().expect("Please enter a valid number");

    let bill = calc_elec_usage(usg);

    println!("Your total electricity bill is ₦{:.2}", bill);


}