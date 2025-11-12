## Requisitos

- Rust 1.91 ou superior
- Cargo

## Instalação

Clone o repositório:

```bash
git clone https://github.com/lwbaleeiro/rust-project
cd rust-project
```

Para verificar a versão do compilador Rust:
```bash
rustc --version
```

Para verificar a versão do gerenciador de pacotes Cargo:
```bash
cargo --version
```

## Como Usar

Execute a aplicação em modo de desenvolvimento:

```bash
cargo run
```

Compile a aplicação para produção:

```bash
cargo build --release
```

O executável estará disponível em `target/release/`.

## Executar Testes

```bash
cargo test
```

## Estrutura do Projeto

```
.
├── src/
│   ├── main.rs       # Ponto de entrada da aplicação
│   └── lib.rs        # Biblioteca (se aplicável)
├── tests/            # Testes de integração
├── Cargo.toml        # Dependências e configuração
└── README.md
```

## Dependências

As principais dependências estão listadas no arquivo `Cargo.toml`.