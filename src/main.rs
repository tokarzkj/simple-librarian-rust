use std::io;
mod library;

fn main() {
    println!("Hello, enter a book! I will see if we have it");

    let mut book = String::new();
    io::stdin()
        .read_line(&mut book)
        .expect("Entered invalid input");

    let mut l = library::library_system::Librarian {
        books_available: Vec::new(),
        checked_out: Vec::new(),
    };

    l.checkout_book(String::from("Test"));
}