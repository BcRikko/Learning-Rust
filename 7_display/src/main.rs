use std::fmt;

struct Structure(i32);

// Structureを出力するときにフォーマットさせたい（C#の toString() が実行されるようなイメージ？）
impl fmt::Display for Structure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct MinMax(i64, i64);

impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // self.0/self.1はデータの位置を示す
        write!(f, "({}, {})", self.0, self.1)
    }
}

#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}

impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

#[derive(Debug)]
struct Complex {
    real: f64,
    imag: f64,
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} + {}i", self.real, self.imag)
    }
}

fn main() {
    let structure = Structure(10);
    println!("{}", structure);

    let minmax = MinMax(0, 14);
    println!("Display: {}", minmax);
    // > Display: (0, 14)
    println!("Debug  : {:?}", minmax);
    // > Debug  : MinMax(0, 14)

    let big_range = MinMax(-300, 300);
    let small_range = MinMax(-3, 3);
    println!("{big} - {small}",
        small = small_range,
        big = big_range
    );

    let point = Point2D { x: 3.3, y: 7.2 };
    println!("Display: {}", point);
    // > Display: x: 3.3, y: 7.2
    println!("Debug  : {:?}", point);
    // > Debug  : Point2D { x: 3.3, y: 7.2 }

    let complex = Complex { real: 3.3, imag: 7.2 };
    println!("Display: {}", complex); 
    // > Display: 3.3 + 7.2i
    println!("Debug  : {:?}", complex); 
    // > Debug  : Complex { real: 3.3, imag: 7.2 }
}