// macros3.rs
// Make me compile, without taking the macro out of the module!
// Execute `rustlings hint macros3` for hints :)


mod macros {
    #[macro_export]         // cool export macro directive.
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }
}

fn main() { my_macro!(); }