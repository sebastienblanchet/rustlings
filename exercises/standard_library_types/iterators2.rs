// iterators2.rs
// In this module, you'll learn some of unique advantages that iterators can offer
// Step 1. Complete the `capitalize_first` function to pass the first two cases
// Step 2. Apply the `capitalize_first` function to a vector of strings, ensuring that it returns a vector of strings as well
// Step 3. Apply the `capitalize_first` function again to a list, but try and ensure it returns a single string
// As always, there are hints if you execute `rustlings hint iterators2`!

pub fn capitalize_first(input: &str) -> String {

    let mut c = input.chars();

    // get first char
    match c.next() {
        // empty str
        None => String::new(),

        // i.e. upcase the first but
        // to_uppercase is an iterator so need to collect the string
        Some(first) => first.to_uppercase().collect::<String>() + c.as_str(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Step 1.
    // Tests that verify your `capitalize_first` function implementation
    #[test]
    fn test_success() {
        assert_eq!(capitalize_first("hello"), "Hello");
    }

    #[test]
    fn test_empty() {
        assert_eq!(capitalize_first(""), "");
    }

    // Step 2.
    // #[test]
    fn test_iterate_string_vec() {
        let words = vec!["hello", "world"];
        // run iterator to capitalize each word
        let capitalized_words: Vec<String> = words.into_iter().map( |x| capitalize_first(x)).collect();

        assert_eq!(capitalized_words, ["Hello", "World"]);
    }

    #[test]
    fn test_iterate_into_string() {
        let words = vec!["hello", " ", "world"];
        // run iterator to capitalize each word
        // need to run the collect
        let capitalized_words: Vec<String> = words.into_iter().map( |x| capitalize_first(x)).collect();
        let singleStr = capitalized_words.concat();

        assert_eq!(singleStr, "Hello World");
    }
}
