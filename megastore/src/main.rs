mod produto;
mod dados;
mod busca;
mod recomendacao;

use std::io;

fn main() {

    let produtos = dados::carregar_produtos();
    let indice = busca::criar_indice(produtos);
    let grafo = recomendacao::criar_grafo();

    println!("Digite a categoria:");

    let mut entrada = String::new();

    io::stdin()
        .read_line(&mut entrada)
        .expect("Erro leitura");

    let entrada = entrada.trim();

    match busca::buscar(&indice, entrada) {

        Some(resultados) => {

            println!("Resultados:");

            for produto in resultados {
                println!("{}", produto.nome);
            }

            println!("\nRecomendações:");

            match recomendacao::recomendar(&grafo, entrada) {

                Some(rec) => {

                    for item in rec {
                        println!("{}", item);
                    }

                }

                None => println!("Sem recomendações"),
            }
        }

        None => println!("Nada encontrado"),
    }
}