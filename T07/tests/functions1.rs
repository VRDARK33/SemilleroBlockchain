// Don't mind this for now :)
// I AM NOT DONE
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn call_function() {
        call_me();
    }

}
fn call_me(){
    print!("hola como estas");
}