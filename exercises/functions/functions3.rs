// functions3.rs
// Execute `rustlings hint functions3` or use the `hint` watch subcommand for a hint.

//call me is declared with a parameter however you call it in main without one
fn main() {
    call_me(32);
}

fn call_me(num: u32) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}
