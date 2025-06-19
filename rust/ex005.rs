use std::io;

fn main() {
    let mut number = String::new();
    println!("Digite um numero: ");

    io::stdin()
        .read_line(&mut number)
        .expect("Erro ao ler número, tente novamente.");

    // ::<> Turbofish
    match number.trim().parse::<i32>() {
        Ok(num) => {
            println!(
                "O antecessor do numero {:#?} é {:#?} e o sucessor é {:#?}",
                num,
                num - 1,
                num + 1
            );
        }
        Err(_) => {
            println!("Por favor, digite um número válido!");
        }
    }

}
