use crate::Media::{Audiobook, Book, Movie, Podcast, Placeholder};

#[derive(Debug)]
#[derive(Clone)]
pub enum Media {
    Book { title: String, author: String },
    Movie { title: String, director: String },
    Audiobook { title: String },
    Podcast(u32), // non-named field
    Placeholder // no fields
}

impl Media {
    pub fn description(&self) -> String {
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
