// functions5.rs
// Make me compile! Execute `rustlings hint functions5` for hints :)

fn main() {
    let x : i32 = 3;
    let answer : i32 = square(x);
    println!("The answer is {}", answer);
}

// RETURN DOES NOT TAKE WITH ; very common error
// IN is a key word
fn square(num: i32) -> i32 {
    return num * num
}
