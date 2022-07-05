// iterators2.rs
// In this exercise, you'll learn some of the unique advantages that iterators
// can offer. Follow the steps to complete the exercise.
// As always, there are hints if you execute `rustlings hint iterators2`!



// iterators my beloved.


// Step 1.
// Complete the `capitalize_first` function.
// "hello" -> "Hello"
pub fn capitalize_first(input: &str) -> String {
    let mut c = input.chars();
    match c.next() { // 1st value. .next consumes first character of iterator BTW.
        None => String::new(),      // to uppercase gives a STRING.
        Some(first) => first.to_uppercase().to_string() + c.as_str(),
    }
}

// Step 2.
// Apply the `capitalize_first` function to a slice of string slices.
// Return a vector of strings.
// ["hello", "world"] -> ["Hello", "World"]
pub fn capitalize_words_vector(words: &[&str]) -> Vec<String> {
    // odd quirk about rust vectors (assumedly because they are simpler than vector structure)
    // they do not have methods like IntoIterator which is used in vectors to do stuff like:
    // for i in vector
    // so you have to convert to iterable first.
    words.iter()// Make into a iterator...
        .map(|word| capitalize_first(word))// Map each value according to function...
        .collect() // Put it back together!
}

// Step 3.
// Apply the `capitalize_first` function again to a slice of string slices.
// Return a single string.
// ["hello", " ", "world"] -> "Hello World"
pub fn capitalize_words_string(words: &[&str]) -> String {
    words.iter().map(|word| capitalize_first(word)).collect()
    //words.iter.map(|word| capitalize_first(word)).collect::<String>()
    // another cool thing!!
    // FromIterator trait can let us convert things/collect things back.
    // This means that we can use collect to collect into a vector, into an array, into whatever!
    // We can do this via the syntax commented, but since rust is so nice to us, it can infer that:
    // - we are returning
    // - return type is String
    // - so maybbeeeeee the collect type should also be String! wow! so nice of you rust
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(capitalize_first("hello"), "Hello");
    }

    #[test]
    fn test_empty() {
        assert_eq!(capitalize_first(""), "");
    }

    #[test]
    fn test_iterate_string_vec() {
        let words = vec!["hello", "world"];
        assert_eq!(capitalize_words_vector(&words), ["Hello", "World"]);
    }

    #[test]
    fn test_iterate_into_string() {
        let words = vec!["hello", " ", "world"];
        assert_eq!(capitalize_words_string(&words), "Hello World");
    }
}
