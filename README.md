# learn wasm

WebAssemblyを[Rust and WebAssembly](https://rustwasm.github.io/docs/book/game-of-life/setup.html)学ぶ

## 0 環境:

Arch Linux x86\_64

## 1 準備: 必要なツールを揃える

1. rustc,rustup & Cargo

コンパイラ、ビルドツール、パッケージマネージャー等の基本的な開発ツール

2. wasm-pack

wasmを生成するのに必要らしい

インストール
```bash
$ cargo install wasm-pack
```

ソースコードをダウンロードしてビルドするので、インストールにはまああまあ時間がかかった

3. cargo-generate

開発をサポートするツールらしい


インストール
```bash
$ cargo install cargo-generate
```

4. npm

Node.jsのパッケージマネージャー
