use std::io;

// Define a struct to represent a book
struct Book {
    title: String,
    author: String,
    publication_year: u32,
}

impl Book {
    // Constructor function to create a new book
    fn new(title: String, author: String, publication_year: u32) -> Self {
        Book {
            title,
            author,
            publication_year,
        }
    }
}

// Define a struct to represent the collection of books
struct BookCollection {
    books: Vec<Box<Book>>,
}

impl BookCollection {
    // Constructor function to create a new empty collection
    fn new() -> Self {
        BookCollection { books: Vec::new() }
    }

    // Function to add a new book to the collection
    fn add_book(&mut self, book: Book) {
        self.books.push(Box::new(book));
    }

    // Function to print all books in the collection
    fn print_books(&self) {
        for (index, book) in self.books.iter().enumerate() {
            println!(
                "Book {}: {}, by {} ({})",
                index + 1,
                book.title,
                book.author,
                book.publication_year
            );
        }
    }

    // Function to remove a book from the collection
    fn remove_book(&mut self, index: usize) -> Result<(), String> {
        if index < self.books.len() {
            self.books.remove(index);
            Ok(())
        } else {
            Err("Book index out of bounds".to_string())
        }
    }

    // Function to search for a book by title
    fn search_by_title(&self, title: &str) -> Option<&Box<Book>> {
        self.books.iter().find(|&book| book.title == title)
    }
}

// Function to interact with the book collection
fn main() {
    let mut collection = BookCollection::new();

    // Add some books to the collection
    collection.add_book(Book::new(
        "The Great Gatsby".to_string(),
        "F. Scott Fitzgerald".to_string(),
        1925,
    ));
    collection.add_book(Book::new(
        "To Kill a Mockingbird".to_string(),
        "Harper Lee".to_string(),
        1960,
    ));
    collection.add_book(Book::new(
        "1984".to_string(),
        "George Orwell".to_string(),
        1949,
    ));

    // Print all books in the collection
    collection.print_books();

    // Search for a book by title
    let search_title = "To Kill a Mockingbird";
    if let Some(book) = collection.search_by_title(search_title) {
        println!(
            "Found book '{}': {}, by {} ({})",
            search_title, book.title, book.author, book.publication_year
        );
    } else {
        println!("Book '{}' not found", search_title);
    }

    // Remove a book from the collection
    if let Err(err) = collection.remove_book(1) {
        println!("Error: {}", err);
    }

    // Print all books in the collection after removal
    println!("After removing:");
    collection.print_books();
}
