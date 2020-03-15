// tests1.rs
// Tests are important to ensure that your code does what you think it should do.
// Tests can be run on this file with the following command:
// rustlings run tests1

// This test has a problem with it -- make the test compile! Make the test
// pass! Make the test fail! Execute `rustlings hint tests1` for hints :)

// this is to configure unit tests
// would run this `cargo tests`
// 
// Section 11.3 The Book 
// https://doc.rust-lang.org/book/ch11-03-test-organization.html?highlight=tests#integration-tests
// 

pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    // allows to test private functions
    use super::*;

    #[test]
    fn you_can_assert() {
        assert!(4 == internal_adder(2, 2));
    }
}
