# 環境構築
https://www.rust-lang.org/ja/learn/get-started

## インストール
```sh
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
...
1) Proceed with installation (default)
2) Customize installation
3) Cancel installation
>1 

...
To configure your current shell, run:
source "$HOME/.cargo/env"

$ source "$HOME/.cargo/env"
```

## fish-shell の設定
```sh
$ vim ~/.config/fish/config.fish
```

```
# rust
set -gx PATH $HOME/.cargo/bin $PATH;
```


## バージョン確認

```sh
$ cargo --version
cargo 1.65.0 (4bc8f24d3 2022-10-20)

$ rustup show
Default host: x86_64-apple-darwin
rustup home:  /Users/{username}/.rustup

stable-x86_64-apple-darwin (default)
rustc 1.65.0 (897e37553 2022-11-02)
```

## Cargo とは

Rustのビルドツール & パッケージマネージャ

```sh
# ビルド
$ cargo build

# 実行
$ cargo run

# テスト
$ cargo test

# ドキュメントのビルド
$ cargo doc

# ライブラリを公開
$ cargo publish
```