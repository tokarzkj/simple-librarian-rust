pub mod library_system {
    pub struct Librarian {
        pub books_available: Vec<String>,
        pub checked_out: Vec<String>,
        pub number_of_books: i32,
    }

    impl Librarian {
        fn number_of_books(&self) -> usize {
            self.books_available.len()
        }

        pub fn checkout_book(&mut self, title: String) {
            if !self.books_available.contains(&title) 
                || self.checked_out.contains(&title) {
                return
            }

            self.checked_out.push(title);
        }
    }
}

#[cfg(test)]
pub mod tests {

    #[test]
    fn checkout_book() {
        let mut l = super::library_system::Librarian {
            books_available: vec![String::from("Lord of the Rings"), String::from("The Three Body Problem")],
            checked_out: Vec::new(),
            number_of_books: 1,
        };

        l.checkout_book(String::from("Lord of the Rings"));

        assert_eq!(l.checked_out.contains(&String::from("Lord of the Rings")), true);
    }
}