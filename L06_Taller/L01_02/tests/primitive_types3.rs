#[cfg(test)]
mod tests {
    #[test]
    fn test_primitive_types3() {
        let a = "hola estoy haciendo test de rust";

        if a.len() >= 100 {
            println!("Wow, that's a big array!");
        } else {
            println!("Meh, I eat arrays like that for breakfast.");
        }        
    }
}