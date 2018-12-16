use std::collections::HashMap;

fn main() {
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
	map.insert(field_name, field_value);

    // the bottom is to test using variables that are out of scope
    // due to map taking ownership on insert

    println!("This shouldn't print {}", field_value);
}


