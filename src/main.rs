// Define a struct to represent an item
struct Item {
    name: String,
    description: String,
}

impl Item {
    // Constructor function to create a new item
    fn new(name: String, description: String) -> Self {
        Item { name, description }
    }
}

// Define a struct to represent the collection of items
struct ItemCollection {
    items: Vec<Box<Item>>,
}

impl ItemCollection {
    // Constructor function to create a new empty collection
    fn new() -> Self {
        ItemCollection { items: Vec::new() }
    }

    // Function to add a new item to the collection
    fn add_item(&mut self, item: Item) {
        self.items.push(Box::new(item));
    }

    // Function to print all items in the collection
    fn print_items(&self) {
        for (index, item) in self.items.iter().enumerate() {
            println!("Item {}: {} - {}", index + 1, item.name, item.description);
        }
    }
}

// Define a function to interact with the item collection
fn main() {
    let mut collection = ItemCollection::new();

    // Add some items to the collection
    collection.add_item(Item::new("Korbad Os".to_string(), "By Josh".to_string()));
    collection.add_item(Item::new("Supporting".to_string(), "Rickrick".to_string()));
    collection.add_item(Item::new("Status".to_string(), "Still Working ".to_string()));

    // Print all items in the collection
    collection.print_items();
}
