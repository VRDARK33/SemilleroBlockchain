// This shopping list program isn't compiling!
// Use your knowledge of generics to fix it.

// I AM NOT DONE
#[cfg(test)]
mod tests {
    use super::*;

    //el error es que el vector no tiene el tipo de dato que recibe definido
    //en este caso debe ser de tipo str porque esta recibiendo una cadena "milk"
    #[test]
    fn test_generic() {
        let mut shopping_list: Vec<&str> = Vec::new();
        shopping_list.push("milk");
    }

}