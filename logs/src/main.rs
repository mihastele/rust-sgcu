use std::fs;
use std::io::Error;


fn extract_errors(text: &str) -> Vec<String> {
    let split_text = text.split("\n");

    let mut results = vec![];

    for line in split_text {
        if line.starts_with("ERROR") {
            results.push(line.to_string());
        }
    }

    results
}


fn main() {
        // match divide(5.0, 3.0) {
    //     Ok(result) => println!("Result: {}", result),
    //     Err(what_went_wrong) => println!("Error: {}", what_went_wrong),
    // }
    //
    // match divide(5.0, 0.0) {
    //     Ok(result) => println!("Result: {}", result),
    //     Err(what_went_wrong) => println!("Error: {}", what_went_wrong),
    // }

    let mut error_logs = vec![];

    match fs::read_to_string("logs.txt") {
        Ok(contents) => {
            error_logs = extract_errors(contents.as_str());
        }
        Err(why) => println!("Couldn't read file: {}", why)
    }
    println!("Found {} errors\n{:#?}", error_logs.len(), error_logs);
}

// fn validate_email(email: &str) -> Result<(), Error> {
//     if email.contains('@') {
//         Ok(())
//     } else {
//         Err(Error::new(
//             std::io::ErrorKind::InvalidInput,
//             "Email address is invalid",
//         ))
//     }
// }
//
// fn divide(a: f64, b: f64) -> Result<f64, Error> {
//     if b == 0.0 {
//         Err(Error::other("Division by zero"))
//     } else {
//         Ok(a / b)
//     }
// }
