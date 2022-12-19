# 一般的なプログラミングの概念

## 変数 と 可変性

```rs
let x = 5;
x = 6;
// value assigned to `x` is never read
// cannot assign twice to immutable variable
```
↓
```rs
let mut x = 5;
x = 6;
// OK
```

## 定数

```rs
// 型アノテーション必須
const X: u32 = 5;

// Error
const mut Y: u32 = 5;
//    ^^^ cannot be mutable

const MAX_POINTS: u32 = 100_00;
```