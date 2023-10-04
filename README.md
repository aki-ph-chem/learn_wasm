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

疑問点?:

JavaScriptの開発はよくわからないが、プロジェクトのディレクトリで生じたファイルはどこまでリポジトリに上げれば良いのだろうか。

## 3 life game の実装

### life game のルール

セルのうち黒色を生きている状態、白色を死んでいる状態とする。各ステップ(世代ごとに)ごとに以下のルールに従って遷移を行う。

- 誕生: 死んでいるセルに隣接する生きているセルが3個あれば次の世代が誕生する
- 生存: 生きているセルに隣接する生きているセルが２個もしくは、３個であれば、次の世代でも生存する
- 過疎: 生きているセルに隣接する生きたセルが一つ以下ならば、過疎により死滅する
- 過密: 生きているセルに隣接する生きたセルが4個以上ならば、過密により死滅する。

### 実装方針

life gameは本来無限に広い宇宙で繰り広げられるが、無限のメモリや無限の計算機のパワーなどは存在しない。
この厄介な制限を回避するために、通常の実装では以下の三種類のうちどれかのを選ぶこととなる

1. 宇宙のどこで興味のある事象が起こっているかを追いかけ続け、必要ならばこの領域を拡張する。最悪のケースではこの拡張は際限なく行われ、実装はおそくなり、最終的にはメモリ不足に陥るであろう。

2. 端のセルに隣接セルが真ん中のセルに比べて少ないサイズを固定した宇宙を作る。この方法の欠点はグライダーパターンのような宇宙の端にたどり着くような無限のパターンが消えてしまうことである。

3. サイズを固定した周期的な宇宙を作る、ただし端のセルはもう片方の端へと回り込むようにする(周期的境界条件が課された系)。この場合、近接するセルは宇宙の端に回り込むので、グライダーは永遠に動き続けることができる。

ここでは、三番目の方法で実装する。

<!-- draft -->
### RustとJavaScriptのインターフェース

- JavaScriptのガーベージコレクタに管理化にあるヒープ(`Object`, `Array`, DOM node)はWebAssembly の線形メモリからは分離されている。
- WebAssembly からは JavaScript のヒープにアクセスすることはできないが、JavaScriptからはWebAssemblyの線形メモリにアクセスことが可能である。
- しかし、可能なのはスカラー型の`ArrayBuffer`としてのみアクセスが可能である
- wasmの関数はスカラー型を受けてスカラー型を返す
