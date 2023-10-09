# learn wasm

WebAssemblyを[Rust and WebAssembly](https://rustwasm.github.io/docs/book/game-of-life/setup.html)学ぶ

## 0 環境:

- OS: Arch Linux x86\_64
- web browser: Google Chrome
- Node.js: 20.8.0
- npm: 10.2.0
- rustc: 1.70.0 
- cargo: 1.70.0

## 1 準備: 必要なツールを揃える

1. rustc,rustup & Cargo

コンパイラ、ビルドツール、パッケージマネージャー等の基本的な開発ツール

2. wasm-pack

RustのコードをWasmにコンパイルをサポートして、JavaScripもしくはTypeScriptのグルーコードまで生成してくれる。


インストール
```bash
$ cargo install wasm-pack
```

ソースコードをダウンロードしてビルドするので、インストールにはまああまあ時間がかかった

3. cargo-generate

開発をサポートするツールで、プロジェクトのテンプレートを作成してくれて非常に便利である。


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

wasm\_bindgenはwasmとJavaScriptとのインターフェースである。

rustからwasmへのビルドは以下のコマンドで実行する。

```bash
$ wasm-pack build
```

ビルド後にはディレクトリ`pkg`が作成されていて、中身は以下のようになっている。

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

.wasmが付くファイルはwasmにコンパイルされて生じたファイルである。

npmコマンドを使ってアプリを作成する

```bash
$ npm init wasm-app www
```

このコマンドの実行後にはディレクトリwwwが作成されている。

次にwwwに入って以下のコマンドを実行する。

```bash
$ npm install
```

最後に以下のコマンドを実行してサーバーを建てる。

```bash
$ npm run start
```

このときwebpackのOpenSSLエラーが出てサーバーが建たなかった。どうやらNode.jsのバージョンが17より新しいことによるものらしい。
今回はあくまでlocalhostで動かすだけでセキュリティなどは気にしないのでOpenSSLのレガシーモードで実行することにした。

これを設定をするには、`game-of-life/www/pckage.json`をの"script"の中の"start"に以下のように`--openssl-legacy-provider`を追加して編集する必要がある

```JSON
"script": {
    "build": "webpack --config webpack.config.js"
    "start": "NODE_OPTIONS='--openssl-legacy-provider' wbpack-dev-server"
}
```

- 疑問点?:
    - JavaScriptの開発はよくわからないが、プロジェクトのディレクトリで生じたファイルはどこまでリポジトリに上げれば良いのだろうか。
    - 答え: JavaScriptのファイルや設定ファイル等はgitで追跡する。しかし`wasm-pack`によって生成された\*.wasmファイルや\*.js,\*.tsファイルは追跡しないようにする。 


## 3 life game のルール

life gameでは、セルのうち黒色を生きている状態、白色を死んでいる状態とする。各ステップ(世代ごとに)ごとに以下のルールに従って遷移を行う。

- 誕生: 死んでいるセルに隣接する生きているセルが3個あれば次の世代が誕生する
- 生存: 生きているセルに隣接する生きているセルが２個もしくは、３個であれば、次の世代でも生存する
- 過疎: 生きているセルに隣接する生きたセルが一つ以下ならば、過疎により死滅する
- 過密: 生きているセルに隣接する生きたセルが4個以上ならば、過密により死滅する。

## 4 life game の実装

### 実装方針

life gameは本来無限に広い宇宙で繰り広げられるが、無限のメモリや無限の計算機のパワーなどは存在しない。
この厄介な制限を回避するために、通常の実装では以下の三種類のうちどれかのを選ぶこととなる

1. 宇宙のどこで興味のある事象が起こっているかを追いかけ続け、必要ならばこの領域を拡張する。最悪のケースではこの拡張は際限なく行われ、実装はおそくなり、最終的にはメモリ不足に陥るであろう。

2. 端のセルに隣接セルが真ん中のセルに比べて少ないサイズを固定した宇宙を作る。この方法の欠点はグライダーパターンのような宇宙の端にたどり着くような無限のパターンが消えてしまうことである。

3. サイズを固定した周期的な宇宙を作る、ただし端のセルはもう片方の端へと回り込むようにする(周期的境界条件が課された系)。この場合、近接するセルは宇宙の端に回り込むので、グライダーは永遠に動き続けることができる。

ここでは、三番目の方法で実装する。

<!-- draft -->
### RustとJavaScriptのインターフェース

- 毎回の遷移でwasm中の全宇宙のメモリのコピーはしたくない。

JavaScript(以下JS)のガーベージコレクタに管理化にあるヒープ(`Object`, `Array`, DOM node)はWebAssembly(以下Wasm) の線形メモリからは分離されている。
そのため、WasmからJSのヒープにアクセスすることはできないが、JSからはWebAssemblyの線形メモリにアクセスことが可能である。
しかし、アクセス可能なのはスカラー型(`u8`,`i32`,`f64`, etc..)の`ArrayBuffer`としてのみアクセスが可能である(本チュートリアルでは`Uint8Array`が登場する)
Wasmの関数はスカラー型の値を受けてスカラー型を返す。

- `wasm_bindgen`はこの境界を跨ぐ複合構造の共通の理解を定義する
- これらは以下を含む: 
    - Rustの箱の構造(Box\<T\>のことではないはず) 
    - JavaScriptのクラスのラップされたポインタ
    - JavaScript のtableやobjectをRustからインデキシングする機能
- `wasm_bindgen`はとても便利であるが、どのような値、構造体がこの境界を超えて渡されるかを考えることを要さなくなるものではない。

- wasmとjsのインターフェースをデザインするにあたって、以下の点を最適化したい
    - wasmの線形メモリへのコピーや線形メモリへのコピーを最小限にする: 不要なコピーはオーバーヘッドを引き起こす
    - シリアライズ、デシリアライズを最小限にする: コピーと同じくオーバーヘッドを減らすため、代わりに不透明な(??)ハンドラを使おう

- 良いデザインの例としては、大きくて、長い寿命を持つデータはRust側(線形メモリ)に持たせて、不透明なハンドラ(??)としてjs側に公開するデザインである。
- jsから不透明なハンドラを取る公開されたwasmの関数を呼んで、データを変換し、重たい計算、データのクエリを処理し最終的には小さいコピー可能な結果の値を返す。
- 結果の小さな値を返すことで、jsの前後のガベージコレクタの管理化にあるヒープとwasmの線形メモリとの間のコピーやシリアライズを避けることができる。

<!-- draft -->
### liefe game におけるRustとjsとの間のインターフェース 

- 避けるべき危険:
    - 宇宙全体をwasmからコピーしようとしすること
    - 宇宙におけるすべてのセルのためにobjectを割り当てること(境界を跨ぐ関数呼び出しで各セルを読み書きするようにしよう)

- 4x4の宇宙は長さ16の一次元配列を4個おきに折り返しているとみて考える
    - index(一次元配列におけるindex) = row * 4 + colum

- 宇宙をjsに公開する方法
    - 宇宙に`std::fmt::Display`を実装する 
    - そしてそれからRustの`String`を生成する
    - これをwasmのメモリからjsのメモリにコピーしてHTMLの`textContent`として描画する
    - 後にこのコピーを避ける実装に書き換える(`canvas`を使う)

<!-- draft -->
### Rustによる実装  

`wasm-game-of-life/src/lib.rs`にセルの定義を実装していく

- ここで重要なのは`#[repr(u8)]`でこれはそれぞれのセルを1バイトで表現するという意味である。
- `Dead`を`0`に`Alive`を`1`にするのは近接する生きていいるセルのカウントを容易にするためである。

続いて宇宙を構造体`Universe`として実装していく

<!-- draft -->
### Rustのメモリをコピーせず直接レンダリングを行う

Rustの`String`をjsのstringに変換することでコピーなしでレンダリングできる。

<!-- draft -->
### life game のテスト

ここではRustによるWebAssemblyのプロジェクトでのテストの行い方について説明する。

- `tick()`を例に挙げてテストをする
- wasmにコンパイルされたRustではborrowされた値を返せない

特定の配置の宇宙に対して`tick()`を呼び出した後の宇宙が期待した通りになるかを自動テストでテストする。

<!-- draft -->
### デバッグ

コンストラクタの関数内の一行目に`utils::set_panic_hook();`を挿入する。

```Rust
pub fn new() -> Universe {
    utils::set_panic_hook();
    // ...
}
```

wasm側から`console.log`を使うには`web-sys`クレート経由で行う

### ベンチ

```bash
$ cargo install cargo-benchmp
```
