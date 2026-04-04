use std::collections::HashMap;
use crate::produto::Produto;

pub fn criar_indice(produtos: Vec<Produto>) -> HashMap<String, Vec<Produto>> {
    let mut indice: HashMap<String, Vec<Produto>> = HashMap::new();

    for produto in produtos {
        indice
            .entry(produto.categoria.to_lowercase())
            .or_default()
            .push(produto);
    }

    indice
}

pub fn buscar<'a>(
    indice: &'a HashMap<String, Vec<Produto>>,
    termo: &str,
) -> Option<&'a Vec<Produto>> {
    indice.get(&termo.to_lowercase())
}