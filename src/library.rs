pub mod lib {
    pub struct Librarian {
        pub books_available: Vec<String>,
        pub checked_out: Vec<String>,
        pub number_of_books: i32,
    }

    trait Lib {
        fn number_of_books(&self) -> usize;

        fn checkout_book(&mut self, title: String);
    }

    impl Lib for Librarian {
        fn number_of_books(&self) -> usize {
            self.books_available.len()
        }

        fn checkout_book(&mut self, title: String) {
            self.checked_out.push(title);
        }
    }
}
