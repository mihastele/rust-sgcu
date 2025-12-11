/* &[String] vector slice of strings */
fn print_elements(elements: &[String]) {
    // for element in elements {
    //     println!("{}", element);
    // }
    elements.iter().for_each(|element| println!("{}", element));
}


fn main() {
    let colors = vec![
        String::from("red"),
        String::from("green"),
        String::from("blue"),
    ];

    print_elements(&colors);
}
