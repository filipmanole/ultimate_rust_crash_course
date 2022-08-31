pub fn inspect(s: &String) {
    if s.ends_with("s") {
        println!("plural")
    } else {
        println!("singular")
    }
}

pub fn change(s: &mut String) {
    s.push_str("s");
}

pub fn eat(s: String) -> bool {
    if s.starts_with("b") && s.contains("a") {
        true
    } else {
        false
    }
}
pub fn add(i1: &i32, i2: &i32) -> i32 { i1 + i2 }
