// fmt::Display, fmt::Debug などで出力できない
struct UnPrintable(i32);
// help: consider annotating `UnPrintable` with `#[derive(Debug)]`
// 2| #[derive(Debug)]
// error: could not compile `debug` due to previous error


// deriveアトリビュートをつけると、fmt::Debugで出力するための実装を自動で提供する
#[derive(Debug)]
struct DebugPrintable(i32);

#[derive(Debug)]
struct Deep(DebugPrintable);

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8
}

fn main() {
    // エラーになる
    // println!("{:?}", UnPrintable(1));

    println!("{:?}", DebugPrintable(1));
    // > DebugPrintable(1)

    println!("{:?}", Deep(DebugPrintable(1)));
    // > Deep(DebugPrintable(1))

    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    println!("{:#?}", peter);
    // > Person {
    // >     name: "Peter",
    // >     age: 27,
    // > }
}

