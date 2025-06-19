use std::io;

fn main() {
    let n1 = read_number("Type a number: ");
    let n2 = read_number("Type another number: ");
    let sum = n1 + n2;

    println!("{} + {} = {}", n1, n2, sum);
}

fn read_number(prompt: &str) -> i32 {
    loop {
        let mut input = String::new();
        println!("{}", prompt);

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input.");

        match input.trim().parse::<i32>() {
            Ok(num) => return num,
            Err(_) => {
                println!("Please enter a valiid integer.");
            }
        }
    }
}
