use num_traits::ToPrimitive;


// First version


// Second - any kind of number

fn pythagoras(a: f64, b: f64) -> f64 {
    let a_f64 = a.to_f64().unwrap(); // included in num_traits
    let b_f64 = b.to_f64().unwrap(); // included in num_traits
    (a_f64.powi(2) + b_f64.powi(2)).sqrt()
}

fn main() {

    let a: f32 = 3.0;
    let b: f64 = 4.0;
    let c: i32 = 5;

    // a + b + c; // No arithmetic allowed between different types
    let a_f64: f64 = a as f64;
    let a_f64 = a.to_f64().unwrap(); // included in num_traits

    println!("{}", pythagoras(3.0, 4.0));
    println!("{}", pythagoras(a_f64, b));
}
