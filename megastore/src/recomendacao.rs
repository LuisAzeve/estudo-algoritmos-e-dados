use std::collections::HashMap;

pub fn criar_grafo() -> HashMap<String, Vec<String>> {
    let mut grafo = HashMap::new();

    grafo.insert(
        "notebook".to_string(),
        vec![
            "mouse".to_string(),
            "teclado".to_string(),
            "mochila".to_string(),
        ],
    );

    grafo.insert(
        "mouse".to_string(),
        vec![
            "mousepad".to_string(),
            "teclado".to_string(),
            "headset".to_string(),
        ],
    );

    grafo.insert(
        "teclado".to_string(),
        vec![
            "mouse".to_string(),
            "headset".to_string(),
            "mousepad".to_string(),
        ],
    );

    grafo
}

pub fn recomendar<'a>(
    grafo: &'a HashMap<String, Vec<String>>,
    categoria: &str,
) -> Option<&'a Vec<String>> {
    grafo.get(&categoria.to_lowercase())
}