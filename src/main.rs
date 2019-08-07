use std::io;
mod library;

fn main() {
    let mut l = get_librarian();

    println!("The librarian has {} books. Try checking one out!", l.number_of_books());
    println!("Type quit or q to leave");

    let mut continue_prompting = true;

    while continue_prompting {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Entered invalid input");

        input = input.trim_end().to_string();

        if input.to_lowercase() == "quit" || input.to_lowercase() == "q" {
            continue_prompting = false;
            continue;
        }

        println!("I am checking to see if {} is available", input);

        let result = l.checkout_book(input);

        match result {
            Ok(r) => r,
            Err(error) => {
                println!("{}", error)
            }
        };

        println!("Your currently checked out books are: ");
        println!("{}", l.get_checkedout_books());
    }

    std::process::exit(0)
}

fn get_librarian() -> library::library_system::Librarian {
    let books = vec![String::from("The Lord Of The Rings"), String::from("A Long Way To An Angry Planet"), String::from("Where the Red Fern Grows")];
    
    library::library_system::Librarian {
        books_available: books,
        checked_out: Vec::new()
    }
}