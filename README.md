When using [Ustr](https://github.com/anderslanglands/ustr) with the `wasm32-unknown-unknown` target, symbols always resolve to an empty string.

Reproduction steps:
1) Run `wasm-pack build --target web`
2) Serve `index.html` (For example, `python3 -m http.server 8080`)
3) Visit `localhost:8080`
4) Note the console output
