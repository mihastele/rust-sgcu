/* &[String] vector slice of strings */
fn print_elements(elements: &[String]) {
    // for element in elements {
    //     println!("{}", element);
    // }
    elements.iter().for_each(|element| println!("{}", element));
}

fn shorten_strings(elements: &mut Vec<String>) {
    elements.iter_mut().for_each(|element| element.truncate(1));
}

fn main() {
    let mut colors = vec![
        String::from("red"),
        String::from("green"),
        String::from("blue"),
    ];

    // print_elements(&colors);

    shorten_strings(&mut colors);
}
