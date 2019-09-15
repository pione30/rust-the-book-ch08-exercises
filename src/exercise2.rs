pub fn to_pig_latin(string: &String) -> Option<String> {
    // check if the input string is all ascii-alphabetic
    if !string.chars().all(|c| c.is_ascii_alphabetic()) {
        return None;
    }

    // check if the input string is longer than 2 characters
    if string.len() < 2 {
        return None;
    }

    match string.get(0..1) {
        Some("a") => Some(string.clone() + "-hay"),
        Some("e") => Some(string.clone() + "-hay"),
        Some("i") => Some(string.clone() + "-hay"),
        Some("o") => Some(string.clone() + "-hay"),
        Some("u") => Some(string.clone() + "-hay"),
        Some(other_str) => Some(string.get(1..).unwrap_or("").to_string() + "-" + other_str + "ay"),
        None => unreachable!(),
    }
}
