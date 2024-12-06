**SISTEMAS LINUX:**
* Linux Mint 21.3 Cinnamon
  
**WSL:**


**VERSÕES:**

* wasmtime 27.0.0 (8eefa236f 2024-11-20)
* rustup 1.27.1 (54dd3d00f 2024-04-24)
* rustc 1.83.0 (90b35a623 2024-11-26)
* cargo 1.83.0 (5ffbef321 2024-10-29)


**Wasmtime:**
```
git clone https://github.com/bytecodealliance/wasmtime.git
```

**Instalação do Wasmtime:**
```
curl https://wasmtime.dev/install.sh -sSf | bash
```

**Instalação do Rustup:**
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup default stable
rustup target add wasm32-unknown-unknown
rustup target add wasm32-wasi
rustup target add wasm32-wasip1
```

**Execução dos Exemplos Wasmtime:**
```
cargo run --example $name
```

**Execução dos Exemplos Wasmtime com invoke:**
```
wasmtime --invoke <func> <wasm/wat> [params]
```

**Desenvolvimentos de Códigos com Rust:**
```
cargo new <program>
cd <program>/src
```
* Modificar main.rs para o código desejado:
	> se quiser fazer um lib, alterar para lib.rs
	> adição de #[no_mangle] nas funções exportáveis
* Modificar Cargo.toml para adicionar dependências:
	> se for um lib, adicionar:
```
[lib]
crate-type = ["cdylib"]
```
**Compilar:** 
```
cargo build --target wasm32-wasi --release
```

**Chamadas WASI com Wasmtime:**
```
WASMTIME_LOG=wasmtime_wasi=trace <comando_exec>
```
