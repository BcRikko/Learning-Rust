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