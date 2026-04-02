use std::collections::HashMap;

pub fn criar_recomendacoes() -> HashMap<String, Vec<String>> {
    let mut grafo = HashMap::new();

    grafo.insert(
        "Notebook".to_string(),
        vec![
            "Mouse".to_string(),
            "Teclado".to_string(),
            "Mochila".to_string(),
        ],
    );

    grafo
}