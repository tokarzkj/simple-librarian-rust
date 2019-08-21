use std::io;
mod library;

fn main() {
    let mut l = get_librarian();

    println!("The librarian has {} books. Try checking one out!", l.number_of_books());
    println!("Type quit or q to leave\r\n");

    let mut continue_prompting = true;

    while continue_prompting {
        let mut command = String::new();
        
        println!("What would you like to do?\r\n");

        io::stdin()
            .read_line(&mut command)
            .expect("Entered invalid input");

        command = command.trim_end().to_string();

        if command.to_lowercase() == "quit" || command.to_lowercase() == "q" {
            continue_prompting = false;
            println!("Okay, have a great day!");
            continue;
        }

        let mut input = String::new();
        if do_prompt_for_book(&command) {
            println!("What book would you like to {}?\r\n", command);
        
            io::stdin()
            .read_line(&mut input)
            .expect("Entered invalid input");

            input = input.trim_end().to_string();
        }

        l = execute_command(l, &command, &input);

        println!("Your currently checked out books are:\r\n ");
        println!("{}", l.get_checkedout_books());
    }

    std::process::exit(0)
}

fn execute_command(mut l : library::library_system::Librarian, 
    command : &str, title : &str) -> library::library_system::Librarian {
    match command {
        "checkout" => {
            let result = l.checkout_book(title.to_string());
            match result {
                Ok(r) => println!("{}\r\n", r),
                Err(err) => println!("{}\r\n", err)
            }
        },
        "return" => {
          let result = l.return_book(title.to_string());
          match result {
              Ok(r) => println!("{}\r\n", r),
              Err(err) => {
                  println!("{}\r\n", err)
              }
          };
        },
        "list" => {
          let result = l.get_list_of_books();
          println!("{}\r\n", result);
        },
        _ => println!("Sorry I don't understand")
    };

    l
}

fn get_librarian() -> library::library_system::Librarian {
    let books = vec![String::from("The Lord Of The Rings"), String::from("A Long Way To An Angry Planet"), String::from("Where the Red Fern Grows")];
    
    library::library_system::Librarian {
        books_available: books,
        checked_out: Vec::new()
    }
}

fn do_prompt_for_book(command : &str) -> bool {
    let do_prompt_for_book = match command {
        "checkout" => true,
        "return" => true,
        "list" => false,
        _ => false
    };

    do_prompt_for_book
}
