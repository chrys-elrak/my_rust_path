use std::io;

fn calculate_length(text: String) -> (usize, String) {
    return (text.len(), text);
}

fn calculate_len(text: &String) -> usize {
    return text.trim().len();
}

fn refr() {
    let mut text = String::new();
    io::stdin()
        .read_line(&mut text)
        .expect("failed to readline");
    // let (len, text) = calculate_length(text);
    let len = calculate_len(&text);
    println!("{} has {} char", text, len);
}

fn add_point(text: &mut String) {
    if !text.ends_with(".") {
        text.push('.');
    }
}

fn main() {
    let mut t = "Hello chrys !".to_string();
    add_point(&mut t);
    println!("{}", t);
}
