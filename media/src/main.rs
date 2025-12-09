use crate::Media::Audiobook;

#[derive(Debug)]
enum Media {
    Book { title: String, author: String },
    Movie { title: String, director: String },
    Audiobook { title: String },
}

impl Media {
    fn description(&self) -> String {
        // option 1 (Not usual)
        if let Media::Book { title, author } = self {
            format!("Book: {} {}", title, author)
        } else if let Media::Movie { title, director } = self {
            format!("Movie {} {}", title, director)
        } else if let Media::Audiobook { title } = self {
            format!("Audiobook {}", title)
        } else {
            String::from("Media description:")
        }
        // option 2
    }
}

fn print_media(media: Media) {
    println!("{:#?}", media);
}

fn main() {
    let audiobook = Media::Audiobook { title: String::from("Rust Programming") };
    let good_movie = Media::Movie { title: String::from("How to be an idiot"), director: String::from("Tobak Stuzzin") };
    let bad_book = Media::Book { title: String::from("How to be retarded"), author: String::from("Luka Tashler") };
    print_media(audiobook);
    print_media(good_movie);
    print_media(bad_book);
}
