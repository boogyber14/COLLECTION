use std::num::ParseIntError;


fn parse_int(s: &str) -> Result<i32, ParseIntError> {
    s.parse::<i32>()
}


fn find_by_id(id: usize) -> Option<String> {
    // Simulated database lookup
    let database = vec![
        (1, "Alice".to_string()),
        (2, "Bob".to_string()),
        (3, "Charlie".to_string()),
    ];

    database.iter()
        .find_map(|&(i, ref name)| if i == id { Some(name.clone()) } else { None })
}

fn main() {

    let num_str = "10";
    match parse_int(num_str) {
        Ok(num) => println!("Parsed number: {}", num),
        Err(e) => eprintln!("Error parsing: {}", e),
    }


    let id_to_find = 2;
    if let Some(result) = find_by_id(id_to_find) {
        println!("Found result: {}", result);
    } else {
        println!("No result found for id: {}", id_to_find);
    }
}
