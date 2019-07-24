use std::io;
mod library;

fn main() {
    println!("Hello, enter a book! I will see if we have it");

    let mut book = String::new();
    io::stdin()
        .read_line(&mut book)
        .expect("Entered invalid input");

    let l = library::lib::Librarian {
        books_available: Vec::new(),
        checked_out: Vec::new(),
        number_of_books: 1,
    };
}
