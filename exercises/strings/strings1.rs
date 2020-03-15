// strings1.rs
// Make me compile without changing the function signature!
// Execute `rustlings hint strings1` for hints ;)

fn main() {
    let answer = current_favorite_color();
    println!("My current favorite color is {}", answer);
}

// always use String::from
// function need return no ;
fn current_favorite_color() -> String {
    // return String::from("blue")
    return "blue".to_string()
}
