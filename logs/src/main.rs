use std::fs;
use std::io::Error;


fn extract_errors(text: &str) -> Vec<&str> {
    let split_text = text.split("\n");

    let mut results = vec![];

    for line in split_text {
        if line.starts_with("ERROR") {
            results.push(line);
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

    match fs::read_to_string("logs.txt") {
        Ok(contents) => {
            let error_logs = extract_errors(contents.as_str());
            println!("{:#?}", error_logs);

            match fs::write("errors.txt", error_logs.join("\n")) {
                Ok(..) => println!("Errors written to file successfully."),
                Err(why) => println!("Couldn't write to file: {}", why),
            }
        }
        Err(why) => println!("Couldn't read file: {}", why)
    }
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
