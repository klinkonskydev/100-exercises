use std::io;

fn main() {
    let celsius = get_temperature();
    let fahrenheit = 9.0 * celsius / 5.0 + 32.0;
    println!("A temperatura de {}℃  corresponde a {}℉ ", celsius, fahrenheit);
}

fn get_temperature() -> f32 {
    loop {
        let mut entry = String::new();
        println!("Informe a temperatura em ℃ ");

        io::stdin()
            .read_line(&mut entry)
            .expect("Erro ao ler linha.");

        match entry.trim().parse::<f32>() {
            Ok(celsius) => return celsius,
            Err(_) => {
                println!("Digite uma temperatura valida.")
            }
        }
    }
}
