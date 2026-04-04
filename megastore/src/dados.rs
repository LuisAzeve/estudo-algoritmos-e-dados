use crate::produto::Produto;

pub fn carregar_produtos() -> Vec<Produto> {
    vec![
        Produto {
            id: 1,
            nome: "Notebook Dell".to_string(),
            categoria: "Notebook".to_string(),
            marca: "Dell".to_string(),
        },
        Produto {
            id: 2,
            nome: "Mouse Logitech".to_string(),
            categoria: "Mouse".to_string(),
            marca: "Logitech".to_string(),
        },
        Produto {
            id: 3,
            nome: "Teclado Gamer".to_string(),
            categoria: "Teclado".to_string(),
            marca: "Redragon".to_string(),
        },
        Produto {
            id: 4,
            nome: "Notebook Lenovo".to_string(),
            categoria: "Notebook".to_string(),
            marca: "Lenovo".to_string(),
        },
    ]
}