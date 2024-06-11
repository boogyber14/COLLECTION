// Remove unused import
// use std::io;

// Define a struct to represent a book
struct Book {
    title: String,
    author: String,
    publication_year: u32,
    description: String, // New field
}

impl Book {
    // Constructor function to create a new book
    fn new(title: String, author: String, publication_year: u32, description: String) -> Self {
        Book {
            title,
            author,
            publication_year,
            description,
        }
    }
}

// Define a struct to represent the collection of books
struct BookCollection {
    books: Vec<Book>,
}

impl BookCollection {
    // Constructor function to create a new empty collection
    fn new() -> Self {
        BookCollection { books: Vec::new() }
    }

    // Function to add a new book to the collection
    fn add_book(&mut self, book: Book) {
        self.books.push(book);
    }

    // Function to print all books in the collection
    fn print_books(&self) {
        for (index, book) in self.books.iter().enumerate() {
            println!(
                "Book {}: {}, by {} ({}) - {}",
                index + 1,
                book.title,
                book.author,
                book.publication_year,
                book.description
            );
        }
    }

    // Function to update book details
    fn update_book(&mut self, index: usize, new_title: String, new_author: String, new_publication_year: u32, new_description: String) -> Result<(), String> {
        if index < self.books.len() {
            let book = &mut self.books[index];
            book.title = new_title;
            book.author = new_author;
            book.publication_year = new_publication_year;
            book.description = new_description;
            Ok(())
        } else {
            Err("Book index out of bounds".to_string())
        }
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
        "A classic novel depicting the roaring twenties.".to_string(),
    ));
    collection.add_book(Book::new(
        "To Kill a Mockingbird".to_string(),
        "Harper Lee".to_string(),
        1960,
        "A gripping tale of racial injustice in the American South.".to_string(),
    ));
    collection.add_book(Book::new(
        "1984".to_string(),
        "George Orwell".to_string(),
        1949,
        "A dystopian vision of a totalitarian future.".to_string(),
    ));

    // Print all books in the collection
    collection.print_books();

    // Update a book in the collection
    if let Err(err) = collection.update_book(1, "To Kill a Mockingbird".to_string(), "Harper Lee".to_string(), 1960, "A Pulitzer Prize-winning novel.".to_string()) {
        println!("Error: {}", err);
    }

    // Print all books in the collection after update
    println!("After updating:");
    collection.print_books();
}
