## 関数

- Rustの命名規則
  - 変数、関数は snake_case を使う
- fnを書く順番は関係ない


```rs
fn main() {
  another_function(5 /** Argument:実引数 */);
}

fn another_function(x: i32 /** Parameter:仮引数 */) {
  ...
}
```

### 文と式

- Rustは式指向言語
- 文: なんらかの動作をして値を返さない命令、セミコロン必須
- 式: 結果値に評価される、セミコロン不要

```rs
fn main() {
  // 文
  let x = 5;
 
  let y = {
    let x = 3;
    x + 1 // ← 式
  };

  println!("The value of y is: {}", y);
  // > 4
}
```

```rs
fn five() -> i32 {
  5
}

fn plus_one(x: i32) -> i32 {
  x + 1

  // x + 1; // セミコロンをつけるとエラーになる
  // fn plus_one(x: i32) -> i32 {
  //                        ^^^ expected `i32`, found `()`

}
```

- `return`キーワードで早期リターンや明示的なリターンも可能
  - 最後の式を暗黙的に return することが多い
