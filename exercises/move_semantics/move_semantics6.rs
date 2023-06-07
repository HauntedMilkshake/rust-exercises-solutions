// move_semantics6.rs
// Execute `rustlings hint move_semantics6` or use the `hint` watch subcommand for a hint.
// You can't change anything except adding or removing references.



//basically we swap how the functions work. Since we don't want get_char to get ownership we 
// make it take a mutable reference of the data and string uppercase takes ownership and we change it so it doesn't take a reference
fn main() {
    let mut data = "Rust is great!".to_string();

    get_char(&mut data);

    string_uppercase(data);
}

// Should not take ownership
fn get_char(data: &mut  String) -> char {
    data.chars().last().unwrap()
}

// Should take ownership
fn string_uppercase(mut data: String) {
    data = data.to_uppercase();

    println!("{}", data);
}
