use std::io;

fn main() {
    let mut name = String::new();

    println!("What's your name? ");
    io::stdin()
        .read_line(&mut name)
        .expect("Deu um erro aqui meu.");

    println!("Ola {}", name);
}
