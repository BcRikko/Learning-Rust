fn main() {
    let val = format!("Hello, {}", "Rust");

    // > Hello, Rust
    println!("{}", val);
    // > [ERR]Hello, Rust
    eprintln!("[ERR]{}", val);

    // プレースホルダー
    println!("{0} -> {1}, {1} <- {0}", "Foo", "Bar");

    // 名前付きプレースホルダー
    println!(
        "{title}, {content}",
        content="Rust",
        title="Hello",
    );

    // 演習
    let pi = 3.141592;
    // let msg = format!()
    println!("Pi is roughly {pi:.perc$}", pi=pi, perc=3)
}
