// variables1.rs
// Make me compile! Execute the command `rustlings hint variables1` if you want a hint :)

fn main() {
    // equiv to const
    let x = 5;
    println!("x has the value {}", x);

    // not const
    let mut y = 3;
    println!("y has the value {}", y);
    y = y + 1;
    println!("y changed {}", y);
}
