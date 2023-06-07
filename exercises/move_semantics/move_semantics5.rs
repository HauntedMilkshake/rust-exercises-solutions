// move_semantics5.rs
// Make me compile only by reordering the lines in `main()`, but without
// adding, changing or removing any of them.
// Execute `rustlings hint move_semantics5` or use the `hint` watch subcommand for a hint.

// all that was needed was to move the line below where the variable is used then it gets dropped and the ownership? gets transferred
//its quite basic you cannto have 2 mutable references to the same variable 

fn main() {
    let mut x = 100;
    let y = &mut x;
    *y += 100;

    let z = &mut x;
    *z += 1000;
    assert_eq!(x, 1200);
}
