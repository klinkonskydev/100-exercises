use std::io;

fn main() {
    let salary = read_amount();
    let salary_increased = give_a_raise(&salary);
    println!("O novo salario do functonario com 15% de aumento e de {:.2}", salary_increased);
}

fn read_amount() -> f32 {
    loop {
        let mut entry = String::new();
        println!("Qual e o salario do funcionario? ");
        io::stdin()
            .read_line(&mut entry)
            .expect("Erro ao ler linha");
        match entry.trim().parse::<f32>() {
            Ok(amount) => return amount,
            Err(_) => {
                println!("Digite um salario valido.");
            }
        }
    }
}

fn give_a_raise(amount: &f32) -> f32 {
    return amount + ((amount * 15.0) / 100.0);
}
