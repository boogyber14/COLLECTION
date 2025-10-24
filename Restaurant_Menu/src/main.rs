use std::io;


struct MenuItem {
    name: &'static str,
    price: f32,
}

fn main() {

    let menu = vec![
        MenuItem { name: "🍔 Burger", price: 5.99 },
        MenuItem { name: "🍕 Pizza", price: 8.49 },
        MenuItem { name: "🥗 Salad", price: 4.25 },
        MenuItem { name: "🥤 Soda", price: 1.50 },
    ];

    println!("==============================");
    println!(" Welcome to Rusty Restaurant!");
    println!("==============================");

    let mut total = 0.0;

    loop {
        println!("\nMenu:");
        for (i, item) in menu.iter().enumerate() {
            println!("{}: {} - ${:.2}", i + 1, item.name, item.price);
        }
        println!("0: Finish order");


        print!("Enter the number of your choice: ");
        let _ = io::Write::flush(&mut io::stdout());
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read input");


        let choice: usize = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number!");
                continue;
            }
        };

        if choice == 0 {
            break;
        } else if choice <= menu.len() {
            let item = &menu[choice - 1];
            total += item.price;
            println!("Added {} (${:.2}) to your order.", item.name, item.price);
        } else {
            println!("Invalid choice. Try again!");
        }
    }

    println!("\n==============================");
    println!("Your total is: ${:.2}", total);
    println!("Thank you for dining with us!");
    println!("==============================");
}
