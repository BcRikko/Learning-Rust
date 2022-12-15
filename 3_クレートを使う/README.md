# クレートを使う

- クレートとは、Rustのパッケージのこと
- crates.io にホスティングされている
- `Cargo.toml`に追加することでビルド時にインストールされる

```toml
...
[dependencies]
ferris-says = "0.2"
```

```rs
// クレートのインポート
use crate::doSomething;

...
```

```sh
$ cargo run
----------------------------
< Hello fellow Rustaceans! >
----------------------------
              \
               \
                 _~^~^~_
             \) /  o o  \ (/
               '_   -   _'
               / '-----' \
```

## [おまけ]VSCode 拡張機能

- rust-analyzer
  - https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer
  - 型やパラメータのヒントがうざい時、以下のオプションをオフにする
    - Type Hints
    - Parameter Hints
