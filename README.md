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
Digite a categoria:
Notebook
Resultados:
Notebook Dell
Notebook Lenovo
Recomendações:
Mouse
Teclado
Mochila
Outro exemplo:
Digite a categoria:
Mouse
Resultados:
Mouse Logitech
Recomendações:
Mousepad
Teclado

Categoria não encontrada:
Digite a categoria:
Geladeira
Nada encontrado

---

## Arquitetura do Sistema

O sistema é dividido em módulos independentes com responsabilidades bem definidas:
main.rs
├── dados.rs       → carrega o catálogo de produtos (Vec<Produto>)
├── busca.rs       → cria o índice HashMap e realiza buscas por categoria
├── recomendacao.rs → grafo de recomendações HashMap<String, Vec<String>>
└── produto.rs     → define a struct Produto

**Fluxo de execução:**

1. `carregar_produtos()` retorna o catálogo como `Vec<Produto>`
2. `criar_indice()` constrói um `HashMap<String, Vec<Produto>>` indexado por categoria
3. O usuário digita uma categoria
4. `buscar()` consulta o HashMap em O(1) e retorna os produtos encontrados
5. `recomendar()` consulta o grafo de recomendações em O(1) e retorna categorias relacionadas

---

## Algoritmos e Estruturas de Dados

### HashMap (Tabela Hash)

Utilizado em dois contextos:

**Índice de produtos:**
```rust
HashMap<String, Vec<Produto>>
// Chave: categoria ("Notebook", "Mouse", ...)
// Valor: lista de produtos dessa categoria
```

**Grafo de recomendações:**
```rust
HashMap<String, Vec<String>>
// Chave: categoria consultada
// Valor: lista de categorias recomendadas (vizinhos no grafo)
```

A tabela hash garante acesso em **O(1)** médio tanto para indexação quanto para busca, independente do tamanho do catálogo.

### Grafo (Lista de Adjacência)

O módulo de recomendações representa um grafo não-direcionado como lista de adjacência usando HashMap. Cada categoria é um nó, e as recomendações são as arestas:
Notebook ──── Mouse
│           │
└── Teclado ┘
│
└── Mochila

A função `recomendar()` faz uma consulta direta ao grafo em O(1), retornando os vizinhos imediatos do nó consultado.

---

## Considerações sobre Desempenho e Escalabilidade

| Operação | Complexidade | Estrutura |
|---|---|---|
| Indexar produto | O(1) médio | HashMap |
| Buscar por categoria | O(1) médio | HashMap |
| Buscar recomendações | O(1) médio | HashMap (grafo) |
| Carregar catálogo | O(n) | Vec |

**Vantagens do HashMap para este caso:**
- Tempo de busca constante independente do número de produtos
- Inserção eficiente ao indexar novos produtos
- Baixo overhead de memória para catálogos com muitas categorias

**Limitações atuais:**
- A busca é case-sensitive — "notebook" não encontra "Notebook"
- O catálogo está hard-coded em `dados.rs`; em produção seria carregado de um banco de dados
- A busca só funciona por categoria exata, sem suporte a busca parcial por nome ou marca

**Em produção**, o próximo passo seria substituir o catálogo estático por uma fonte de dados externa (banco ou arquivo JSON) e adicionar normalização de texto na busca.

---

## Estrutura do Repositório
├── src/
│   ├── main.rs          # Ponto de entrada e interface de console
│   ├── produto.rs       # Struct Produto
│   ├── dados.rs         # Catálogo simulado de produtos
│   ├── busca.rs         # Indexação e busca por HashMap
│   └── recomendacao.rs  # Grafo de recomendações
├── tests/
│   └── integracao.rs    # Testes de integração
├── Cargo.toml
└── README.md

---

## Licença

Projeto acadêmico desenvolvido para a disciplina **Data Structures Strategy and Implementation — UniFECAF**.  
Distribuído sob a licença [MIT](https://opensource.org/licenses/MIT).

---

## Contribuições

Este é um projeto acadêmico individual. Sugestões podem ser abertas via Issues no repositório
