// lifetimes1.rs
//
// The Rust compiler needs to know how to check whether supplied references are
// valid, so that it can let the programmer know if a reference is at risk
// of going out of scope before it is used. Remember, references are borrows
// and do not own their own data. What if their owner goes out of scope?

// I AM NOT DONE



fn longest(x: &str, y: &str) -> String {
    if x.len() > y.len() {
        x.to_string()
    } else {
        y.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_lifetime() {
        let string1 = String::from("abcd");
        let string2 = String::from("xyz");
    
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }
}