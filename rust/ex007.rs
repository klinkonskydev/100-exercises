use std::io;

fn main() {
    let n1 = read_note("Primeira nota do aluno: ");
    let n2 = read_note("Segunda nota do aluno: ");
    println!("A média entre {} e {} é {}", n1, n2, (n1+n2)/2.0);
}

fn read_note(prompt: &str) -> f64 {
    loop {
        let mut note = String::new();
        println!("{prompt}");

        io::stdin()
            .read_line(&mut note)
            .expect("Erro ao ler input.");

        match note.trim().parse::<f64>() {
            Ok(num) => return num,
            Err(_) => {
                println!("Por favor, digite um numero valido.");
            }
        }
    }
}
