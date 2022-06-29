use std::io;

fn input_i32(prompt: &str) -> f32 {
    let mut text = String::new();
    loop {
        println!("{}", prompt);
        io::stdin()
        .read_line(&mut text)
        .expect("Failed to read user input");
        let text: f32  = match text.trim().parse() {
            Ok(num) => num,
            Err(x) => {
                print!("{}", x);
                continue
            }
        };
        return text;
    }
}

pub fn main() {
    println!("(a)x^2 + (b)x + (c)");
    let a = input_i32("Please give me a: ");
    let b = input_i32("Please give me b: ");
    let c = input_i32("Please give me c: ");
    println!("{}x^2 + {}x + {}", a, b, c);

    let delta = b.powf(2_f32) - 4_f32 * a * c;
    if delta == 0_f32 {
        let x0 = -b / 2_f32 * a;
        print!("Only one solution for it ! \n x0 = {}", x0);
    } else if delta > 0_f32 {
        print!("There is two solutions !");
        let x1 = -b - delta.sqrt() / 2_f32 * a;
        let x2 = -b + delta.sqrt() / 2_f32 * a;
        print!("x1 = {}, x2 = {}", x1, x2);
    } else if delta < 0_f32 {
        print!("No solution for now !");
    }
}