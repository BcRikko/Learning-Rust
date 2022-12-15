# monorepo

JavaScript の感覚で monorepo のプロジェクトを開こうとすると、VSCode拡張機能の Rust-Analyzer が「rust-analyzer failed to discover workspace」というエラーを吐く

プロジェクトルートに `Cargo.toml` がないといけないらしい

以下のようなディレクトリ構造の場合。
```
.
├─ Cargo.toml  // ルートの Cargo ファイル
├─ 2_HelloRust
｜  ├─ Cargo.toml
｜  └─ src
｜      └─ main.rs
└─ 3_グレートを使う
    ├─ Cargo.toml
    └─ src
        └─ main.rs

```


ルートに `Cargo.toml` を作成し、以下のように workspace を定義する
```toml
[workspace]

members = [
    "2_HelloRust",
    "3_クレートを使う"
]
```

参考: https://doc.rust-jp.rs/book-ja/ch14-03-cargo-workspaces.html
