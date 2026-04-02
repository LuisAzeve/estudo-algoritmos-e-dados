use std::io;

fn main(){
    println!("Estou aprendendo!");
    exemplo();

}

fn exemplo(){
    let mut s = String::new();
    println!("Digite um texto!");

    io::stdin()
        .read_line(&mut s);
        

    println!("Você digitou {s}");
}