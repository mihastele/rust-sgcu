/* &[String] vector slice of strings */
fn print_elements(elements: &[String]) {
    // for element in elements {
    //     println!("{}", element);
    // }
    elements.iter().for_each(|element| println!("{}", element));
}

fn shorten_strings(elements: &mut [String]) {
    elements.iter_mut().for_each(|element| element.truncate(1));
}

fn to_uppercase(elements: &[String]) -> Vec<String> {
    elements.iter().map(|element| element.to_uppercase()).collect::<Vec<String>>()
}

fn move_elements(vec_a: Vec<String>, vec_b: &mut Vec<String>) {
    vec_a.into_iter().for_each(|element| vec_b.push(element));
}

fn explode(elements: &[String]) -> Vec<Vec<String>> {
    elements.
        iter()
        .map(|element| element.chars()
            .map(|c| c.to_string()).collect())
        .collect::<Vec<Vec<String>>>()
}

fn find_color_or(elements: &[String], search: &str, fallback: &str) -> String {
    elements.iter().
        find(|el| el.contains(search)).
        map_or(String::from(fallback), |el| el.to_string())
}

fn main() {
    let mut colors = vec![
        String::from("red"),
        String::from("green"),
        String::from("blue"),
    ];

    // print_elements(&colors);

    // shorten_strings(&mut colors);
    // shorten_strings(&mut colors[1..3]);
    // println!("{:#?}", colors);

    let uppercased = to_uppercase(&colors);
    println!("{:#?}", uppercased);

    // let mut destination = vec![];
    // move_elements(colors, &mut destination);
    // println!("{:#?}", destination);

    // println!("{:#?}", explode(&colors));

    let found = find_color_or(&colors, "re", "orange");
    println!("{}", found);
    let found = find_color_or(&colors, "sus", "orange");
    println!("{}", found);
}
