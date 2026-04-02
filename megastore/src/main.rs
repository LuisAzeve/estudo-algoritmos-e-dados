use std::io;

fn main() {
    println!("Digite o produto:");

    let mut busca = String::new();

    io::stdin()
        .read_line(&mut busca)
        .expect("Erro ao ler");

    println!("Buscando por {}", busca);
}