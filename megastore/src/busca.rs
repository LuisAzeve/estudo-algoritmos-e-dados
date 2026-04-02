use std::collections::HashMap;
use crate::produto::Produto;

pub fn criar_indice(produtos: Vec<Produto>) -> HashMap<String, Vec<Produto>> {
    let mut indice = HashMap::new();

    for produto in produtos {
        indice
            .entry(produto.categoria.clone())
            .or_insert(Vec::new())
            .push(produto);
    }

    indice
}