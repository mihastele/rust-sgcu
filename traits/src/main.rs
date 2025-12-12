mod basket;
mod stack;

use basket::Basket;
use stack::Stack;

fn main() {
    let b1 = Basket::new(String::from("apple"));
    let b2 = Basket::new(10);
    let mut b3 = Basket::new(true);

    // b3.put(20);  // Once type defiend, we're stuck
    
    let s1 = Stack::new(
        vec![String::from("apple"), String::from("banana")]
    );
}
