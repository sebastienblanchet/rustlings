// tests2.rs
// This test has a problem with it -- make the test compile! Make the test
// pass! Make the test fail! Execute `rustlings hint tests2` for hints :)


pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {

    // could also just ask for exact name space function
    // similar to cpp

    use internal_adder;

    #[test]
    fn you_can_assert_eq() {
        assert_eq!(4, internal_adder(2, 2));
    }
}
