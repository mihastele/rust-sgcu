use crate::Media::{Audiobook, Book, Movie, Podcast, Placeholder};

#[derive(Debug)]
enum Media {
    Book { title: String, author: String },
    Movie { title: String, director: String },
    Audiobook { title: String },
    Podcast(u32), // non-named field
    Placeholder // no fields
}

impl Media {
    fn description(&self) -> String {
        // option 1 (Not usual)
        // if let Media::Book { title, author } = self {
        //     format!("Book: {} {}", title, author)
        // } else if let Media::Movie { title, director } = self {
        //     format!("Movie {} {}", title, director)
        // } else if let Media::Audiobook { title } = self {
        //     format!("Audiobook {}", title)
        // } else {
        //     String::from("Media description:")
        // }
        // option 2

        match self {
            Audiobook { title } => format!("Audiobook: {}", title),
            Movie { title, director } => format!("Movie: {} {}", title, director),
            Book { title, author } => format!("Book: {} {}", title, author),
            Podcast(n) => format!("Podcast: {}", n),
            Placeholder => String::from("Media description:"),
            _ => String::from("Media description:")
        }
    }
}

#[derive(Debug)]
struct Catalog {
    items: Vec<Media>,
}

impl Catalog {
    fn new() -> Self {
        Self { items: Vec::new() }
    }

    fn add(&mut self, item: Media) {
        self.items.push(item);
    }

    fn get_by_index_dirty(&self, index: usize) -> &Media {
        &self.items[index]
    }

    fn get_by_index(&self, index: usize) -> MightHaveAValue {
        if self.items.len() > index {
            MightHaveAValue::ThereIsAValue(&self.items[index])
        } else {
            MightHaveAValue::NoValueAvailable
        }
    }
}

enum MightHaveAValue<'a> {
    ThereIsAValue(&'a Media),
    NoValueAvailable,
}

fn print_media(media: Media) {
    println!("{:#?}", media);
}

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
}
