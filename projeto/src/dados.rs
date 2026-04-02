use crate::produto::Produto;

pub fn carregar_produtos() -> Vec<Produto> {
    vec![
        Produto {
            id: 1,
            nome: String::from("Notebook Dell"),
            categoria: String::from("Notebook"),
            marca: String::from("Dell"),
        },
        Produto {
            id: 2,
            nome: String::from("Mouse Logitech"),
            categoria: String::from("Mouse"),
            marca: String::from("Logitech"),
        },
    ]
}