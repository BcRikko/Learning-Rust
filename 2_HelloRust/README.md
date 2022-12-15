# Hello Rust

1. 新しいプロジェクトを作成する
   ```sh
   $ cargo new hello-rust

   $ tree
   .
   └─ hello-rust
       ├─ Cargo.toml
       └─ src
            └─ main.rs
   ```
   - `Cargo.toml` ... マニフェストファイル、メタ情報や依存関係が記録される
   - `src/main.rs` ... エントリーポイント
2. Rustを実行する
   ```rs
   // src/main.rs
   fn main() {
     println!("Hello, world!");
   }
   ```
   ```sh
   $ cargo run
   Compiling hello-rust v0.1.0 (/path/to/hello-rust)
    Finished dev [unoptimized + debuginfo] target(s) in 1.68s
     Running `target/debug/hello-rust`

   Hello, world!
   ```
