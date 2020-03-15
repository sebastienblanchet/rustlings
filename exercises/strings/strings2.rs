// strings2.rs
// Make me compile without changing the function signature!
// Execute `rustlings hint strings2` for hints :)


fn main() {
    let word = String::from("green"); // Try not changing this line :)
    // so  word is a pointer?
    // needed to ref word
    // de ref word will give the str
    if is_a_color_word(&word) {
        println!("That is a color word I know!");
    } else {
        println!("That is not a color word I know.");
    }
}

fn is_a_color_word(attempt: &str) -> bool {
    // take in pointer of string
    attempt == "green" || attempt == "blue" || attempt == "red"
}
