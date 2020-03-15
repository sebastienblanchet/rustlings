// modules1.rs
// Make me compile! Execute `rustlings hint modules1` for hints :)

// name space
mod sausage_factory {
    pub fn make_sausage() {
        println!("sausage!");
    }
}

fn main() {
    // fn ==> private
    // deal with this
    // use super::*;
    // no way of colling the function without pub
    sausage_factory::make_sausage();
}
