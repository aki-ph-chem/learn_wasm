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

## 2 Hello, World

### プロジェクトの作成

`cargo-generate`コマンドを用いてテンプレートをcloneする。

```bash
$ cargo generate --git https://github.com/rustwasm/wasm-pack-template
```

プロジェクト名を聞かれるので、チュートリアル通りに`wasm-game-of-life`とした。
これによって以下のようなディレクトリが作成される。

```text
wasm-game-of-life
├── Cargo.toml
├── LICENSE_APACHE
├── LICENSE_MIT
├── README.md
├── src
│   ├── lib.rs
│   └── utils.rs
└── tests
    └── web.rs

3 directories, 7 files
```
このディレクトリの中で重要なのは依存関係を記述している`Cargo.toml`とクレートのルートとなる`src/lib.rs`である。

<!-- draft -->

wasm\_bindgen: JavaScriptとのインターフェース

ビルド: 
```bash
$ wasm-pack build
```

ビルド後の`pkg`の中身

```text
pkg
├── package.json
├── README.md
├── wasm_game_of_life_bg.js
├── wasm_game_of_life_bg.wasm
├── wasm_game_of_life_bg.wasm.d.ts
├── wasm_game_of_life.d.ts
└── wasm_game_of_life.js

1 directory, 7 files
```

```bash
$ npm init wasm-app www
```

wwwに入って

```bash
$ npm install
```

Error:
最後にサーバーを建てるところ

```bash
$ npm run start
```

でwebpackのOpenSSLエラーが出てサーバーが建たない。

これは以下のように`game-of-life/www/pckage.json`をのscriptの中のstartに以下のように`--openssl-legacy-provider`を追加して編集する必要がある

```JSON
"script": {
    "build": "webpack --config webpack.config.js"
    "start": "NODE_OPTIONS='--openssl-legacy-provider' wbpack-dev-server"
}
```
