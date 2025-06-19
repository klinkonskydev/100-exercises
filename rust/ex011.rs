use std::io;

fn main() {
    let width = read_float("Largura da parede: ");
    let height = read_float("Altura da parede: ");

    let area = width * height;
    let tint = area / 2.0;

    println!("Sua parede tem a dimensão de {width}x{height} e sua área é de {area}m2");
    println!("Para pintar essa parede, você precisa de {tint}l de tinta.");
}

fn read_float(prompt: &str) -> f32 {
    loop {
        let mut entry = String::new();

        println!("{prompt}");

        io::stdin()
            .read_line(&mut entry)
            .expect("Error reading line.");

        match entry.trim().parse::<f32>() {
            Ok(float_number) => return float_number,
            Err(_) => {
                println!("Type a valid number.")
            }
        }
    }
}
