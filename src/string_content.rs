pub mod test {
    use std::io;

    pub fn calculate_length(text: String) -> (usize, String) {
        return (text.len(), text);
    }

    pub fn calculate_len(text: &String) -> usize {
        return text.trim().len();
    }

    pub fn refr() {
        let mut text = String::new();
        io::stdin()
            .read_line(&mut text)
            .expect("failed to readline");
        // let (len, text) = calculate_length(text);
        let len = calculate_len(&text);
        println!("{} has {} char", text, len);
    }

    pub fn add_point(text: &mut String) {
        if !text.ends_with(".") {
            text.push('.');
        }
    }
}
