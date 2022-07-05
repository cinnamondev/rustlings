// option2.rs
// Make me compile! Execute `rustlings hint option2` for hints


fn main() {
    let optional_word = Some(String::from("rustlings"));
    // TODO: Make this an if let statement whose value is "Some" type
    // this looks a bit off initially, what we are doing with iflet is pattern matching without the match.
    // We are saying if optional_word is a Option of Some containing "word", which is declared by the `let`.
    // then execute on that condition.
    if let Some(word)= optional_word {
        println!("The word is: {}", word);
    } else {
        println!("The optional word doesn't contain anything");
    }

    let mut optional_integers_vec: Vec<Option<i8>> = Vec::new();
    for x in 1..10 {
        optional_integers_vec.push(Some(x));
    }

    // TODO: make this a while let statement - remember that vector.pop also adds another layer of Option<T>
    // You can stack `Option<T>`'s into while let and if let
    // ew wtf SomeSome??
    while let Some(Some(integer)) = optional_integers_vec.pop() {
        println!("current value: {}", integer);
    }
}
