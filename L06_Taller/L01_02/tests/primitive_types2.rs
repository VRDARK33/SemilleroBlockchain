#[cfg(test)]
mod tests {
    #[test]
    fn test_primitive_types2() {
        // Characters (`char`)

        // Note the _single_ quotes, these are different from the double quotes
        // you've been seeing around.
        let my_first_initial = 'C';
        if my_first_initial.is_alphabetic() {
            println!("Alphabetical!");
        } else if my_first_initial.is_numeric() {
            println!("Numerical!");
        } else {
            println!("Neither alphabetic nor numerioc!");
        }

        let your_character = '4';
        if your_character.is_alphabetic() {
            println!("Alphabetical!");
        } else if your_character.is_numeric() {
            println!("Numerical!");
        } else {
            println!("Neither alphabetic nor numeric!");
        }
    }

}