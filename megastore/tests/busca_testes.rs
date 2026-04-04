use megastore::dados;
use megastore::busca;
use megastore::recomendacao;

#[test]
fn teste_busca_notebook() {

    let produtos = dados::carregar_produtos();
    let indice = busca::criar_indice(produtos);

    let resultado = busca::buscar(&indice, "notebook");

    assert!(resultado.is_some());
}

#[test]
fn teste_busca_sem_resultado() {

    let produtos = dados::carregar_produtos();
    let indice = busca::criar_indice(produtos);

    let resultado = busca::buscar(&indice, "geladeira");

    assert!(resultado.is_none());
}

#[test]
fn teste_recomendacao() {

    let grafo = recomendacao::criar_grafo();

    let resultado = recomendacao::recomendar(&grafo, "Notebook");

    assert!(resultado.is_some());
}