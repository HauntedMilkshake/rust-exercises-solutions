// move_semantics2.rs
// Execute `rustlings hint move_semantics2` or use the `hint` watch subcommand for a hint.

// Expected output:
// vec0 has length 3 content `[22, 44, 66]`
// vec1 has length 4 content `[22, 44, 66, 88]`

// one way is to just make a copy of vec0 when passing it to fill_vec instead of passing the value itself - still don't know why it is like that
// the other 

//for solution number 2 we have to make the first vector mutable and pass a *mutable* reference to the fill_vec function
//after that we do not need a new variable for vec to take ownership because we are passing a mutable reference and we also 
//drop the need to return a value  
//however since the function now doesn't return a function we need declare the vectors as instances and on a seperate line call the method and pass
// a mutable reference to the vectors which alters themselves rathern than changing ownership
fn main() {
    let mut vec0 = Vec::new();

    // Do not move the following line!
    let mut vec1 = Vec::new();
    fill_vec(&mut vec0);
    fill_vec(&mut vec1);
    //adding vec0.clone() as the parameter to fill_vec fixes the issue
    // Do not change the following line!
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

fn fill_vec(vec: &mut Vec<i32>){
    //let vec : Vec<i32> = vec;

    vec.push(22);
    vec.push(44);
    vec.push(66);
}
