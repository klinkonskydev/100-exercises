use std::io;

fn main() {
    let product_price = get_product_price();
    let five_percent_of_discount = (product_price * 5.0) / 100.00;
    let product_with_discount = product_price - five_percent_of_discount;
    println!("O seu produto no valor de {} com 5% de desconto é R${:.2}", product_price, product_with_discount);
}

fn get_product_price() -> f32 {
    loop {
        let mut entry = String::new();
        println!("Preço do produto: ");
        io::stdin()
            .read_line(&mut entry)
            .expect("Error reading line.");

        match entry.trim().parse::<f32>() {
            Ok(product_price) => return product_price,
            Err(_) => {
                println!("Por favor, digite um número válido.")
            }
        }
    }
}
