// macros1.rs
// Make me compile! Execute `rustlings hint macros1` for hints :)

// macros take a ! 
// but what exactly is a macro?
// macros is writting code that writes OTHER code

macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

fn main() {
    my_macro!();
}
