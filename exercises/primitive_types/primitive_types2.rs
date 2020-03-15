// primitive_types2.rs
// Fill in the rest of the line that has code missing!
// No hints, there's no tricks, just get used to typing these :)

fn whatChar(c : char) {
    if c.is_alphabetic() {
        println!("Alphabetical!");
    } else if c.is_numeric() {
        println!("Numerical!");
    } else {
        println!("Neither alphabetic nor numeric!");
    }
}

fn main() {
    // Characters (`char`)
    // NEED mut to change val
    // to call val functions needs to be mutatble
    let mut my_first_initial : char = 'C';
    whatChar(my_first_initial);
    my_first_initial = '1';
    whatChar(my_first_initial);
}
