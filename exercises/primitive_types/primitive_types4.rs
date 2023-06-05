// primitive_types4.rs
// Get a slice out of Array a where the ??? is so that the test passes.
// Execute `rustlings hint primitive_types4` or use the `hint` watch subcommand for a hint.


#[test]
fn slice_out_of_array() {
    let a = [1, 2, 3, 4, 5];
    //making a slice of array a
    //the syntax is as follows we show the location of the array we are making a slice of
    // then we declare from where we start the slice inclusive and where it ends not inclusive
    let nice_slice = &a[1..4]; 

    assert_eq!([2, 3, 4], nice_slice)
}
