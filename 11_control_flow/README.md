# 制御フロー

## 条件分岐
```rs
let falsy = 0;
// Error: expected `bool`, found integer
if falsy {
    println!("falsy")
}

let num = if condition { 5 } else { 6 };
```

- if の条件式は bool でないとエラーになる
  - JSのようなtruthy/falsyのようには使えない
- ワンライナーでも書ける


## ループ

- loop
- while
- for

```rs
// ラベル付きループ
'label: loop {
  ...
  // 'label のループを break(終了)する
  break 'label;
}

while condition {
  ...
}

let collection = ...
for item in collection {
  ...
}

// Range
for num in (1..4).rev() {
  ...
}
```