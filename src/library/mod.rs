pub mod library_system {
    use std::error::Error;
    use std::fmt;

    pub struct Librarian {
        pub books_available: Vec<String>,
        pub checked_out: Vec<String>,
    }

    impl Librarian {
        pub fn number_of_books(&self) -> usize {
            self.books_available.len()
        }

        pub fn checkout_book(&mut self, title: String) -> Result<(), LibrarianError> {
            if !self.books_available.contains(&title) {
                return Err::<(), LibrarianError>(LibrarianError::new("Book isn't available"))
            }

            if self.checked_out.contains(&title) {
                return Err::<(), LibrarianError>(LibrarianError::new("Book is already checked out"))
            }

            self.checked_out.push(title);
            Ok(())
        }

        pub fn get_checkedout_books(&self) -> String {
            let mut books = String::from("");

            for book in &self.checked_out {
                books.push_str(book);
                books.push_str("\n");
            };

            books
        }
    }

    #[derive(Debug)]
    pub struct LibrarianError {
        details: String,
    }

    impl LibrarianError {
        fn new(msg: &str) -> LibrarianError {
            LibrarianError {
                details: msg.to_string()
            }
        }
    }

    impl fmt::Display  for LibrarianError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.details)
        }
    }

    impl Error for LibrarianError {
        fn description(&self) -> &str {
            &self.details
        }
    }
}

#[cfg(test)]
pub mod tests {
    fn setup() -> super::library_system::Librarian {
        super::library_system::Librarian {
            books_available: vec![String::from("Lord of the Rings"), String::from("The Three Body Problem")],
            checked_out: Vec::new(),
        }
    }

    #[test]
    fn book_count() {
        let l = setup();

        assert_eq!(l.number_of_books(), 2);
    }

    #[test]
    fn checkout_book() {
        let mut l = setup();
        l.checkout_book(String::from("Lord of the Rings"));

        assert_eq!(l.checked_out.contains(&String::from("Lord of the Rings")), true);
    }

    #[test]
    fn checkout_invalid_book() {
        let mut l = setup();

        l.checkout_book(String::from("The Lord of the Flies"));
        assert_eq!(l.checked_out.contains(&String::from("The Lord of the Flies")), false);
    }
}