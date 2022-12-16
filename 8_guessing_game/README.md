# 数当てゲームのプログラミング

https://doc.rust-jp.rs/book-ja/ch02-00-guessing-game-tutorial.html

## ライブラリの読み込み
- デフォルトで読み込まれる標準ライブラリのセットを「prelude（プレリュード）」という
  - https://doc.rust-lang.org/std/prelude/index.html
- preludeにない場合は都度 `use`文で明示的に読み込む

## 変数
- Rustの変数はデフォルトで不変（Immutable）
- 可変(Mutable)にするには `mut` をつける

```rs
let immutable = 10;
let mut mutable = 10;
```

## 関数の実行

```rs
// use をする場合
use std::io
io::stdin() ...

// use をしない場合
std::io::stdin() ...
```

```rs
io::stdin()
  // & は参照渡し、ただし &guess はNG
  .read_line(&mut guess)
```

## エラーハンドリング

- Result型でエラーかどうか判定する
- Result型は列挙型（enum） = Ok | Err
- `match`と一緒に使われる

```rs
io::stdin()
  .read_line(&mut guess) // io::Result
  .expect("エラー時のメッセージ");
```
```rs
// Result型の実装
enum Result<T, E> {
   Ok(T),
   Err(E),
}

// .expect() の実装
pub fn expect(self, msg: &str) -> T
where
    E: fmt::Debug,
{
    match self {
        // 成功(OK)ならそのままの値を返す
        Ok(t) => t,
        // 失敗(Err)なら panic!("{msg}: {error :?}") を出力
        Err(e) => unwrap_failed(msg, &e),
    }
}
```

## クレート

### クレートの違い
- バイナリクレート
  - 実行可能なファイル
- ライブラリクレート
  - コードが含まれており、単体では実行できない

### 依存関係

- `Cargo.toml` = NPM の `package.json`
- `Cargo.lock` = NPM の `package-lock.json`

`[dependencies]`にクレートとバージョンを書くと、ビルド時にダウンロードしてくれる
```toml
[dependencies]
...
```

- クレートをアップデート・アップグレードするときも cargo コマンドで行う
- その際、セマンティックバージョニングによってアップグレードする範囲を制限できる
```sh
$ cargo update
```