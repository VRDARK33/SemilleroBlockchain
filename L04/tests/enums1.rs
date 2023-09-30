// I AM NOT DONE

#[derive(Debug)]
enum Message {
    Quit,
    Echo(i32),
    Move(i32),
    ChangeColor(String),


}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn call_enum() {
        println!("{:?}", Message::Quit);
        println!("{:?}", Message::Echo(3));
        println!("{:?}", Message::Move(23));
        println!("{:?}", Message::ChangeColor("green".to_string()));    
    }

}

