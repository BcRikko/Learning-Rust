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

## シャドーイング

```rs
let x = 5;
// ブロックスコープ
{
  let x = x * 2;
}
```

### `mut` とシャドーイングの違い

- `mut`の場合は型が保持されるため、異なる型の値を代入できない
- シャドーイングをすると、同じ変数名だけど異なる型を受け入れられる

```rs
// mut を使う場合
let mut spaces = "    ";
// mismatched types
spaces = spaces.len();
//       ^^^^^^^^^^^^ expected `&str`, found `usize`

// シャドーイングの場合
let spaces = "    ";
let spaces = spaces.len();
// OK
```