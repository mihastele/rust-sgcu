mod basket;
mod stack;
mod container;

use basket::Basket;
use stack::Stack;
use container::Container;

fn add_string<T: Container<String>>(c: &mut T, item: String) {
    c.put(item);
}

fn main() {
    let mut b1 = Basket::new(String::from("apple"));
    let b2 = Basket::new(10);
    let mut b3 = Basket::new(true);

    // b3.put(20);  // Once type defiend, we're stuck

    let mut s1 = Stack::new(
        vec![String::from("apple"), String::from("banana")]
    );
    let s2 = Stack::new(vec![1, 2, 3]);
    let mut s3 = Stack::new(vec![true, false]);

    add_string(&mut b1, String::from("orange"));
    add_string(&mut s1, String::from("grape"));

}
