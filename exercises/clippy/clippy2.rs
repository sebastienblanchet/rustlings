// clippy2.rs
// Make me compile! Execute `rustlings hint clippy2` for hints :)

fn main() {
    let mut res = 42;
    let option = Some(12);

    /*
     for loop over option
     this is 💩
    for x in option {
        res += x;
    }
    */

    // as per clippy lint
    if let Some(x) = option {
        res += x;
    }

    println!("{}", res);
}
