use std::io;

fn main() {
    let number = read_number("Digite um número: ");
    tabuada(number);
}

fn tabuada (number: i32) {
    let mut i = 1;
    while i < 11 {
        println!("{}x{} = {}", i, number, i * number);
        i = i + 1;
    }
}

fn read_number(prompt: &str) -> i32 {
    loop {
        let mut user_input = String::new();

        println!("{}", prompt);

        io::stdin()
            .read_line(&mut user_input)
            .expect("Erro ao ler a linha.");

        match user_input.trim().parse::<i32>() {
            Ok(num) => return num,
            Err(_) => {
                println!("Por favor, Digite um número válido.")
            }
        }
    }
}
