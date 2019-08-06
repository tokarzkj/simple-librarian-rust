use std::io;
mod library;

fn main() {
    let mut l = get_librarian();

    println!("The librarian has {} books. Try checking one out!", l.number_of_books());
    println!("Type quit or q to leave");

    let mut continue_prompting = true;

    while continue_prompting {
        let mut book = String::new();
        io::stdin()
            .read_line(&mut book)
            .expect("Entered invalid input");

        if book == "quit" || book == "q" {
            continue_prompting = true;
            continue;
        }

        println!("I am checking to see if {} is available", book);

        let result = l.checkout_book(book);

        match result {
            Ok(r) => r,
            Err(error) => {
                println!("There was an error, {}", error)
            }
        };

        println!("Your currently checked out books are: ");
        println!("{}", l.get_checkedout_books());
    }
}

fn get_librarian() -> library::library_system::Librarian {
    let books = vec![String::from("The Lord Of The Rings"), String::from("A Long Way To An Angry Planet"), String::from("Where the Red Fern Grows")];
    
    library::library_system::Librarian {
        books_available: books,
        checked_out: Vec::new()
    }
}