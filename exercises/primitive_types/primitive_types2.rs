// primitive_types2.rs
//
// Fill in the rest of he line thatt has code missing! No hints, there's no
// tricks, just get used to typing these :)

// I AM NOT DONE

fn main() {
    // Characters (`char`)

    // Note the _single_ quotes, these are different from the double quotes
    // you've been seeing around.
    // let my_first_initial = 'C';
    // if my_first_initial.is_alphabetic() {
    //     println!("Alphabetical!");
    // } else if my_first_initial.is_numeric() {
    //     println!("Numerical!");
    // } else {
    //     println!("Neither alphabetic nor numeric!");
    // }

    // Finish this line like the example! What's your favorite character?
    let your_character: char = 'a'; // alpha
    let your_character: char = '1'; // an number
    let your_character: char = '#'; // an symbol
    let your_character: char = '\u{001b}'; // an escape char
    let your_character: char = '字'; // an Chinese character
    let your_character: char = 'ア'; // an Japanese character

    // Try a letter, try a number, try a special character, try a character
    // from a different language than your own, try an emoji!
    if your_character.is_alphabetic() {
        // WTF? is_alphabetic() thought ア is alphabetic???
        println!("Alphabetical! {}", your_character);
    } else if your_character.is_numeric() {
        println!("Numerical!");
    } else {
        println!("Neither alphabetic nor numeric!");
    }
}
