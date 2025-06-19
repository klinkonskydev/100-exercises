use std::io;

fn main() {
    let distance = read_number("Uma distância em metros: ");
    println!(
        "A média de {} corresponde a \n{:.3}km\n{:.2}hm\n{:.1}dam\n{}dm\n{}cm\n{}mm",
        distance, distance / 1000.0, distance / 100.0, distance / 10.0, distance * 10.0, distance * 100.0, distance * 1000.0
    );
}

fn read_number(prompt: &str) -> f32 {
    loop {
        let mut number = String::new();
        println!("{prompt}");

        io::stdin()
            .read_line(&mut number)
            .expect("Erro ao ler input");

        match number.trim().parse::<f32>() {
            Ok(num) => return num,
            Err(_) => {
                println!("Por favor, digite um numero valido.");
            }
        }
    }
}
