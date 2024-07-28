# rust_fizzbuzz

Rustの勉強をしてみた。
[参考URL](https://qiita.com/hinastory/items/543ae9749c8bccb9afbc)

## 気づき

- `touch test.rs` ではなく `cargo new test --bin` で始めるべき
  - VSCode で `rust-analyzer` を使うには `Cargo.toml` がルートディレクトリに必要
  - コンパイルは `rustc test.rc` ではなく `cargo build`
- コンパイラがエラー文だけでなくヘルプを表示する形で提案してくれる
- `println!` の `!` はマクロを表す、Rust の関数には可変長引数がないためマクロになっている
- 変数の再代入を許可する場合は キーワード `mut` をつける
- キーワード `match` でパターンを網羅的に処理することが保証される
- マッチガードで `match` 内の複雑な条件を表現できる
