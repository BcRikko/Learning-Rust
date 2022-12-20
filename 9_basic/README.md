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


## データ型

### スカラー型
- 整数: `123`
- 浮動小数点: `123.456`
- 論理値: `true`
- 文字: `"text"`

#### 整数型

|大きさ|符号付き|符号なし|
|-----|-----|------|
|8bit|i8|u8|
|16bit|i16|u16|
|32bit|i32|u32|
|64bit|i64|u64|
|arch|isize|usize|

- isize/usize はプログラムが動作するコンピュータに依存する
- 基準型はi32型, 64bitシステム上でも最速


#### 浮動小数点型

|大きさ|型|
|-----|----|
|32bit|f32|
|64bit|f64|

- 基準型はf64
- f32とほぼ同スピードで、より精度が高くなるため
- f32:単精度浮動小数点数(float)
- f64:倍精度浮動小数点(double)

#### 文字型

- `char`: シングルクオート
  - Unicodeのスカラー値を表す
  - ASCIIより多くのものを表現できる
- 文字列: ダブルクオート

###　複合型

#### タプル型

```rs
let tup: (i32, f64, u8) = (500, 6.4, 1);
let (x, y, z) = tup;
tup.0, tup.1, tup.2
```

#### 配列型

- 配列は固定長
  - 可変長にしたい場合はベクタ型を使う
- ヒープよりスタックにデータのメモリを確保したい時に使う

```rs
// i32型で要素数5の配列
let arr: [i32; 5] = ...

// `3`を5個含む配列
let threes = [3; 5];
```