use std::io;

fn input_i32(prompt: &str) -> i32 {
    let v = &mut String::new();
    println!("{}", prompt);
    io::stdin()
        .read_line(v)
        .expect("Failed to read use&r input");
    let a: i32 = v.trim().parse().unwrap_or(0);
    return a;
}

pub fn second_degree() {
    println!("(a)x^2 + (b)x + (c)");
    let a = input_i32("Please give me a: ");
    let b = input_i32("Please give me b: ");
    let c = input_i32("Please give me c: ");
    println!("{}x^2 + {}x + {}", a, b, c);

    let delta = b.pow(2) - 4 * a * c;
    if delta == 0 {
        let x0 = -b / 2 * a;
        print!("Only one solution for it ! \n x0 = {}", x0);
    } else if delta > 0 {
        print!("There is two solutions !");
        let x1 = (-b as f32) - (delta as f32).sqrt() / 2.0 * (a as f32);
        let x2 = (-b as f32) + (delta as f32).sqrt() / 2.0 * (a as f32);
        print!("x1 = {}, x2 = {}", x1, x2);
    } else if delta < 0 {
        print!("No solution for now !");
    }
}