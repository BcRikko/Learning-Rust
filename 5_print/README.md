# print
https://doc.rust-jp.rs/rust-by-example-ja/hello/print.html

## コンソールへの出力メソッド

- `format!` … テキストをString型に書き込む
- `print!` … `format!` と同じ、コンソール(`io::stdout`)に出力する
- `println!`
- `eprint!` … `format!`と同じ、標準エラー（`io::stderr`)に出力する
- `eprintln!`


## macro_rules!
https://doc.rust-jp.rs/rust-by-example-ja/macros.html

### マクロとは

- 関数と違うのは `format!` のように `!` で終わる
- 関数呼び出しを生成する代わりに、ソースコード内に展開されコンパイルされる

### マクロの作り方
```rs
macro_rules! say_hello {
  // () は引数なしという意味
  () => {
    // コンパイル段階でここに書かれた内容が展開される
    println!("Hello!");
  };
}

fn main() {
  say_hello!(); // ← println!("Hello!") に置き換わる
}
```

詳細は別で……


## トレイト
https://doc.rust-lang.org/std/fmt/#formatting-traits

### トレイトとは

> コンピュータープログラミングでの概念であり、専らオブジェクト指向プログラミングで用いられている。トレイトはメソッドの集合体であり、クラスの機能を拡張するために使われる。
> https://ja.wikipedia.org/wiki/%E3%83%88%E3%83%AC%E3%82%A4%E3%83%88

### `std::fmr` はトレイトを持つ

- `fmt::Debug`
- `fmt::Display`