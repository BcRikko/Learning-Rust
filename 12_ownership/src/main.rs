fn main() {
    // 文字列リテラルはImmutable
    let literal = "This is Literal";
    println!("{}", literal);

    // String型: ヒープにメモリ確保するのでサイズが変わってもOK
    let mut s = String::from("Hello");
    s.push_str(", World!");
    // s += ", World!"; との違いは？
    println!("{}", s);
    
    // メモリ解放
    drop(s);
    // println!("{}", s);
    // ERR:           ^ value borrowed here after move


    // スタック
    let mut x = 5;
    let y = x;
    x = 3;
    println!("x: {}, y: {}", x, y);
    // > x: 3, y: 5

    // ヒープ
    let s1 = String::from("hello");
    let s2 = s1;
    // println!("s1: {}, s2: {}", s1, s2);
    // Error:                     ^^ value borrowed here after move
    
    // クローン
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1: {}, s2: {}", s1, s2);

    // 関数
    let s = String::from("Hello");
    takes_ownership(s);
    // println!("{}", s);
    // Error:         ^^ borrow of moved value: `s`

    let x = 5;
    makes_copy(x);


    // 戻り値
    let r1 = gives_ownership();
    println!("r1: {}", r1);

    let r2 = String::from("hello");
    let r3 = takes_and_gives_back(s2);
    println!("r2: {}, r3: {}", r2, r3);

    let t1 = String::from("Tuple");
    let (t2, len) = calculate_length(t1);
    // println!("t1: {}, t2: {}, len: {}", t1, t2, len);
    //                                     ^^ value borrowed here after move

    let ref1 = String::from("Reference");
    let len = calculate_length2(&ref1);
    println!("ref1: {}, len: {}", ref1, len);

    let mut mutRef = String::from("Hello");
    change(&mut mutRef);
    println!("mutRef: {}", mutRef);

    // let reference_to_nothing = dangle();

    let s = String::from("Hello world");
    let word = first_word(&s);
    println!("word: {}", word);
    // s.clear(); // Error: mutable borrow ocurs here
    println!("word: {}", word);
    
    let s = "Hello world";
    let word = first_word(&s);
    println!("word: {}", word);

}


fn takes_ownership(some_string: String) {
    println!("{}", some_string);
} // スコープを抜けるので `drop` が呼ばれる

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
} // スコープを抜けるが特別なにもしない


fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string // 呼び出し元にムーブされる
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

fn calculate_length2(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(", world")
}
  
// fn dangle() -> &String {
//     //         ^ expected named lifetime parameter
//     let s = String::from("hello");
//     &s
// }

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        // b' ' … バイトリテラル表記
        if item == b' ' {
            return &s[0..i]
        }
    }

    &s[..]
}