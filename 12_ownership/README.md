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


## 参照と借用

- String型を呼び出し元に返さないとムーブされてしまい参照できなくなる
- `&xxx` とすることで ownership を渡す代わりにオブジェクトへの参照を渡す
  - 参照外しは `*` を使う

```rs
fn main() {
  let s = String::from("Hello");
  // &s で参照を生成し渡す
  let len = calculate_length(&s);
  println!("{}, {}", s, len);
}

fn calculate_length(s: &String) -> usize {
  // s は参照で ownership は持っていないためスコープを外れても有効（dropされない）
  s.len()

  // s.push_str(", world");  // 変更はできない
  // ^^ cannot borrow as mutable
}
```

### 可変な参照

```rs
fn main() {
  let mut s = String::from("hello");
  change(&mut s)

  println!("{}", s);
  // > hello, world
}

fn change(s: &mut String) {
  s.push_str(", world");
}
```

### ダングリングポインタ

- コンパイラが絶対ダングリング参照にならないように保証してくれる

```rs
fn main() {
  let reference_to_nothing = dangle();
}

fn dangle() -> &String { // s がドロップされるのにその参照を返している
  let s = String::from("hello");
  &s
} // s のスコープが抜けるのでドロップされる

// Error: missing lifetime specifier

fn no_dangle() -> String {
  let s = String::from("hello");
  s
}
```

## スライス型


```rs
let s = String::from("hello world");
let word = first_word(&s);
// word と s は関連がある

s.clear();
// s をクリアすると word の意味がなくなるが、コンパイルエラーにならない
```

```rs
let s = String::from("hello world");

// 文字列スライス
let hello = &s[0..5];
let world = &s[6..11];

//---
let len = s.len();

// 0〜2
let slice = &s[0..2];
let slice = &s[..2];

// 3〜最後
let slice = &s[3..len];
let slice = &s[3..];

// 0〜最後
let slice = &s[0..len];
let slice = &s[..];
```

- String: 文字列型
- str: 文字列スライス型
  - 文字列リテラル = スライス

```rs
// 文字列型（String）
let s = String::from("...");

// スライス型（&str）
let s = "hello world";