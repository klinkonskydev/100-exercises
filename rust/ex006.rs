use std::io;

fn main() {
    println!("Type a number: ");
    let mut number = String::new();

    io::stdin()
        .read_line(&mut number)
        .expect("Ocorreu um erro, tente novamente!");

    match number.trim().parse::<i32>() {
        Ok(num) => {
            let double: i32 = num * 2;
            let triple: i32 = num * 3;
            let square_root = (num as f64).sqrt();

            println!(
                "Your number is {:#?}, double is {}, triple is {} and square is {}",
                num, double, triple, square_root
            );
        }
        Err(_) => {
            println!("invalid number.");
        }
    }
}
