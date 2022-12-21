# Ownership（所有権）

## Rustのメモリ管理

- コンパイラがコンパイル時にチェックする
- Ownershipシステムを通じて管理する

## スタックとヒープ

- スタック
  - 得た順番に値を並べ、逆順に取り除いていく（LIFO）
  - 高速な反面、固定サイズでなければならない
- ヒープ
  - スタックに比べ低速
    - ポインタをたどってデータを探すため
    - 大きな領域を確保するため
  - 可変データを格納できる

## String型

### メモリ確保
```rs
// 文字列リテラルはImmutable
let literal = "This is Literal";

// String型: ヒープにメモリ確保するのでサイズが変わってもOK
let mut s = String::from("Hello");
s.push_str(", World!");
println!("{}", s);

// メモリ解放
// ※スコープからでたとき( `}`のところ )で自動的に drop が実行される
drop(s);

// 以降、 s を参照できなくなる
```

### ムーブ

```rs
let s1 = String::from("hello");
let s2 = s1; // 参照がコピーされる

// コンパイラは「s1 は有効でない」と考え、利用できなくする（メモリ安全性）
// ※メモリの二重開放エラーなどのバグを回避するため
println!("s1: {}, s2: {}", s1, s2);
```

- 本来は Shallow copy なのだが、コンパイラがコピー前の変数を無効化するので「Move」と表現する


### クローン

```rs
let s1 = String::from("hello");
let s2 = s1.clone(); // ディープコピー
```

- `xxx.clone()`でディープコピー
- スタック上にある変数は、不変サイズなので clone しなくてもデータがそのままコピーされる


## 所有権と関数

```rs
fn main() {
  let s = String::from("hello");
  takes_ownership(s);
  // takes_ownership で s が drop されているので s を使えない
  println!("{}", s); // エラーになる
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
} // スコープを抜けるので `drop` が呼ばれる

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
} // スコープを抜けるが特別なにもしない
```


## 戻り値とスコープ

```rs
fn main() {
    let t1 = String::from("Tuple");
    let (t2, len) = calculate_length(t1);
    // println!("t1: {}, t2: {}, len: {}", t1, t2, len);
    //                                     ^^ value borrowed here after move
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}
```