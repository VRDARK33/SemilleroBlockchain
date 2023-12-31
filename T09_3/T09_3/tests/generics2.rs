// This powerful wrapper provides the ability to store a positive integer value.
// Rewrite it using generics so that it supports wrapping ANY type.


// I AM NOT DONE
/*no tiene asignado el tipo de dato que manejan ni en la estrutura wrapper
 ni en la funcion wrapper en este caso le agregamos U que recibe cualquier
 tipo de dato */
struct Wrapper <U>{
    value: U,
}

impl<U> Wrapper <U> {
    pub fn new(value: U) -> Self {
        Wrapper { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_u32_in_wrapper() {
        assert_eq!(Wrapper::new(42).value, 42);
    }

    #[test]
    fn store_str_in_wrapper() {
        assert_eq!(Wrapper::new("Foo").value, "Foo");
    }
}
