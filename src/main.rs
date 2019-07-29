use std::io;
mod library;

fn main() {
    let mut l = library::library_system::Librarian {
        books_available: Vec::new(),
        checked_out: Vec::new(),
    };

    println!("The librarian has {} books. Try checking one out!", l.number_of_books());

    let mut book = String::new();
    io::stdin()
        .read_line(&mut book)
        .expect("Entered invalid input");

    

    l.checkout_book(book);
}