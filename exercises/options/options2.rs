// options2.rs
// Execute `rustlings hint options2` or use the `hint` watch subcommand for a hint.


#[cfg(test)]
mod tests {
    #[test]
    fn simple_option() {
        let target = "rustlings";
        let optional_target = Some(target);

        // TODO: Make this an if let statement whose value is "Some" type
        if let Some(word) = optional_target {
            assert_eq!(word, target);
        } else { 
            println!("No match");
        }
    }

    #[test]
    fn layered_option() {
        let range = 10;
        let mut optional_integers: Vec<Option<i8>> = vec![None];

        for i in 1..(range + 1) {
            optional_integers.push(Some(i));
        }

        let mut cursor = range;

        // TODO: make this a while let statement - remember that vector.pop also adds another layer of Option<T>
        // You can stack `Option<T>`s into while let and if let
        //In Rust, the .flatten() method is used to simplify nested Option values. It operates on an Option<Option<T>> and returns an Option<T>. 
        //If the outer Option is Some, it "flattens" it by removing one level of nesting and returning the inner Option. 
        //If the outer Option is None, it returns None.
        while let Some(integer) = optional_integers.pop().flatten() {
                assert_eq!(integer, cursor);
                cursor -= 1;
        }

        assert_eq!(cursor, 0);
    }
}
