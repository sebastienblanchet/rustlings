// functions2.rs
// Make me compile! Execute `rustlings hint functions2` for hints :)

fn main() {
    let mut y : i32 = 10;
    call_me(y);
}

// must declare type to function
fn call_me(num : i32) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}
