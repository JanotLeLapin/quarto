# quarto

jeu quarto écrit en Rust

## bases de code

| nom | description |
|-----|-------------|
| `./core` | définition des structures & implémentation des règles du jeu |
| `./players` | implémentation des IA |
| `./app` | programme d'entrée principal |
| `./wasm` | bindings WebAssembly du moteur quarto |
| `./web` | interface web pour le jeu, dépend de `./wasm` |
