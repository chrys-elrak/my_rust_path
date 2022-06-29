mod string_content;

use crate::string_content::test::{add_point};

fn main() {
    let mut t = "Hello chrys !".to_string();
    add_point(&mut t);
    println!("{}", t);
}
