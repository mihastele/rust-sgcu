mod basket;

use basket::Basket;

fn main() {
    let b1 = Basket::new(String::from("apple"));
    let b2 = Basket::new(10);
    let mut b3 = Basket::new(true);

    // b3.put(20);  // Once type defiend, we're stuck
}
