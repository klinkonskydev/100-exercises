use std::io;

fn main() {
    let dollar_price = 3.27;
    let real = read_number("Quanto dinheiro você tem na carteira? R$");
    println!("Com R${} você pode comprar R${:.2}", real, real / dollar_price);
}

fn read_number(prompt: &str) -> f32 {
    loop {
        let mut real = String::new();

        println!("{}", prompt);

        io::stdin()
            .read_line(&mut real)
            .expect("Falha ao ler linha.");

        match real.trim().parse::<f32>() {
            Ok(num) => return num,
            Err(_) => {
                println!("Digite um número válido.")
            }
        }
    }
}
