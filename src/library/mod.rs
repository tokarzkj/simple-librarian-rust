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

        pub fn get_list_of_books(&self) -> String {
            let mut books = String::from("");

            for book in &self.books_available {
                books.push_str(book);
                books.push('\n');
            }

            books
        }

        pub fn checkout_book(&mut self, title: String) -> Result<&str, LibrarianError> {
            if !self.books_available.contains(&title) {
                return Err::<&str, LibrarianError>(LibrarianError::new("Book isn't available"))
            }

            if self.checked_out.contains(&title) {
                return Err::<&str, LibrarianError>(LibrarianError::new("Book is already checked out"))
            }

            self.checked_out.push(title);
            Ok("The book is all yours!")
        }

        pub fn get_checkedout_books(&self) -> String {
            let mut books = String::from("");

            for book in &self.checked_out {
                books.push_str(book);
                books.push_str("\n");
            };

            books
        }

        pub fn return_book(&mut self, title: String) -> Result<&str, LibrarianError> {
            if !self.checked_out.contains(&title) {
                return Err::<&str, LibrarianError>(LibrarianError::new("That title wasn't checked out"));
            }

            let book_index = match self.checked_out.binary_search(&title) {
                Ok(index) => index,
                Err(e) => e
            };

            self.checked_out.remove(book_index);
            Ok("Thank you for returning your book!")
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
        l.checkout_book(String::from("Lord of the Rings")).unwrap();

        assert_eq!(l.checked_out.contains(&String::from("Lord of the Rings")), true);
    }

    #[test]
    fn list_books() {
        let l = setup();
        let books = l.get_list_of_books();
        assert_eq!(books, "Lord of the Rings\nThe Three Body Problem\n");
    }

    #[test]
    #[should_panic]
    fn checkout_invalid_book() {
        let mut l = setup();
        l.checkout_book(String::from("The Lord of the Flies")).unwrap();
    }

    #[test]
    fn return_book() {
        let mut l = setup();

        l.checkout_book(String::from("Lord of the Rings")).unwrap();
        let response = l.return_book(String::from("Lord of the Rings")).unwrap();
        
        assert_eq!(response, "Thank you for returning your book!");
    }

    #[test]
    #[should_panic]
    fn return_book_not_checked_out() {
        let mut l = setup();
        l.return_book(String::from("The Lord Of The Rings")).unwrap();
    }
}
