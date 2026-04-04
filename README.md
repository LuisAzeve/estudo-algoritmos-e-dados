# Sistema de Busca Otimizado para Catálogo de Produtos - MegaStore

Sistema de busca e recomendação de produtos desenvolvido em Rust para o catálogo da MegaStore, utilizando tabelas hash (HashMap) e grafos para indexação eficiente e recomendações baseadas em categoria.

---

## Tecnologias Utilizadas

- **Rust** (edition 2021)
- **std::collections::HashMap** — indexação e grafo de recomendações
- **std::io** — interface de entrada do usuário
- **cargo test** — framework de testes integrado ao Rust

---

## Como Rodar

**Pré-requisitos:** Rust instalado ([rustup.rs](https://rustup.rs))
```bash
# Clonar o repositório
git clone https://github.com/LuisAzeve/estudo-algoritmos-e-dados
cd estudo-algoritmos-e-dados

# Compilar e executar
cargo run
```

## Como Executar os Testes
```bash
cargo test
```

---

## Exemplos de Uso

Ao rodar o programa, ele solicita uma categoria:
