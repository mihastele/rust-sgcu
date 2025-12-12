
fn pythagoras(a: f64, b: f64) -> f64 {
    (a.powi(2) + b.powi(2)).sqrt()
}

fn main() {

    let a: f32 = 3.0;
    let b: f64 = 4.0;
    let c: i32 = 5;

    a + b + c; // No arithmetic allowed between different types

    println!("{}", pythagoras(3.0, 4.0));
    println!("{}", pythagoras(a, b));
}
