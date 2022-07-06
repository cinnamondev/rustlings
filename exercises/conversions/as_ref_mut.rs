// AsRef and AsMut allow for cheap reference-to-reference conversions.
// Read more about them at https://doc.rust-lang.org/std/convert/trait.AsRef.html
// and https://doc.rust-lang.org/std/convert/trait.AsMut.html, respectively.


// Obtain the number of bytes (not characters) in the given argument
// Add the AsRef trait appropriately as a trait bound

            // For type T that implements AsRef for str (base) type. This means &str, String, etc... at a base level all of them implement `str`
fn byte_counter<T: AsRef<str>>(arg: T) -> usize {       // what we are essentially saying here is we can refer to another type as long as the AsRef trait is implemented for that type.
    arg.as_ref().as_bytes().len()                       // as a more succinct example, if we had a struct describing String, we could impl AsRef<str> such that it could take that String value
}                                                       // This simple example can expand to more complex examples converting simular tyeps ie String -> AsRef<Path>.


// Obtain the number of characters (not bytes) in the given argument
// Add the AsRef trait appropriately as a trait bound
fn char_counter<T: AsRef<str>>(arg: T) -> usize {
    arg.as_ref().chars().count()
}

fn main() {
    let s = "Café au lait";
    println!("{}", char_counter(s));
    println!("{}", byte_counter(s));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn different_counts() {
        let s = "Café au lait";
        assert_ne!(char_counter(s), byte_counter(s));
    }

    #[test]
    fn same_counts() {
        let s = "Cafe au lait";
        assert_eq!(char_counter(s), byte_counter(s));
    }

    #[test]
    fn different_counts_using_string() {
        let s = String::from("Café au lait");
        assert_ne!(char_counter(s.clone()), byte_counter(s));
    }

    #[test]
    fn same_counts_using_string() {
        let s = String::from("Cafe au lait");
        assert_eq!(char_counter(s.clone()), byte_counter(s));
    }
}
