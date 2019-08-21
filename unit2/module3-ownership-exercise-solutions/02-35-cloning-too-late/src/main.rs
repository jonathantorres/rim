fn main() {
    let s = String::from("book");

    // Add code here that calls the pluralize function
    let pl = pluralize(s);

    println!(
        "I have one {}, you have two {}",
        s.clone(),
        pl,
    );
}

// Add appropriate parameters, return values, and implementation to this function
fn pluralize(singular: String) -> String {
    singular + "s"
}
