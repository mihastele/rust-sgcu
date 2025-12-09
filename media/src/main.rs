mod content;

use content::media::Media;
use content::catalog::{Catalog, MightHaveAValue};

fn main() {
    let audiobook = Media::Audiobook { title: String::from("Rust Programming") };
    let good_movie = Media::Movie { title: String::from("How to be an idiot"), director: String::from("Tobak S.") };
    let bad_book = Media::Book { title: String::from("How to be retarded"), author: String::from("Looka T.") };
    let podcast = Media::Podcast(123);
    let placeholder = Media::Placeholder;
    println!("{}", audiobook.description());
    println!("{}", good_movie.description());
    println!("{}", bad_book.description());

    let mut catalog = Catalog::new();
    catalog.add(audiobook);
    catalog.add(good_movie);
    catalog.add(bad_book);
    catalog.add(podcast);
    catalog.add(placeholder);
    // for item in catalog.items {
    //     print_media(item);
    // }
    match catalog.items.get(0) {
        Option::Some(value) => {
            println!("GG {:#?}", value);
        }
        None => {
            println!("No media");
        }
    }

    match catalog.get_by_index(0) {
        MightHaveAValue::ThereIsAValue(value) => {
            println!("GG {:#?}", value);
        }
        MightHaveAValue::NoValueAvailable => {
            println!("No media");
        }
    }

    if let MightHaveAValue::ThereIsAValue(value) = catalog.get_by_index(0) {
        println!("GG {:#?}", value);
    }
    else {
        println!("No media");
    }

    if let MightHaveAValue::ThereIsAValue(value) = catalog.get_by_index(550) {
        println!("GG {:#?}", value);
    }
    else {
        println!("Game over");
    }

    // Ownership tests
    // let mut evenCat = Catalog::new();
    // let mut flip = true;
    // for media in catalog.items{
    //     if flip {
    //         evenCat.add(media.clone());
    //     }
    //     flip = !flip;
    // }
    // for i in 0..catalog.items.len() {
    //     if flip {
    //         if let MightHaveAValue::ThereIsAValue(media) = catalog.get_by_index(i) {
    //             evenCat.add(media.clone());
    //         }
    //     }
    //     flip = !flip;
    // }
    //
    // println!("{:?}", evenCat);
    // println!("{:?}", catalog);


    println!("{:#?}", catalog.items.get(10000).unwrap_or(&Media::Placeholder));
}
