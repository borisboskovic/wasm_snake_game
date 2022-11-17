#[allow(dead_code)]
pub fn run() {
    // Bool and Option
    let _is_enabled = true;
    let mut _is_active = Option::<bool>::Some(true);

    if let Some(value) = _is_active {
        println!("{}", value);
    } else {
        println!("No value");
    }
    println!("{:?}", _is_active);

    // Integer sizes and formatting
    let decimal_num = 98_000;
    let hex_num = 0xfa;
    let octal_num = 0o741;
    let bin_num = 0b0010_1111;
    let byte_num = b'A';
    let character = 'A';

    println!(
        "* Decimal: {0} {0:E} {0:e} {0:X} {0:x} {0:o} {0:b}",
        decimal_num
    );
    println!(
        "* Hexadecimal: {0} {0:E} {0:e} {0:X} {0:x} {0:o} {0:b}",
        hex_num
    );
    println!(
        "* Octal: {0} {0:E} {0:e} {0:X} {0:x} {0:o} {0:b}",
        octal_num
    );
    println!("* Binary: {0} {0:E} {0:e} {0:X} {0:x} {0:o} {0:b}", bin_num);
    println!("* Byte: {0} {0:E} {0:e} {0:X} {0:x} {0:o} {0:b}", byte_num);
    println!("* Character: {}", character);
    println!("{0:.>10}", decimal_num);

    // Formatting float numbers
    let float_num = 0.546_356;
    println!("{0:.2} {:*<10} {0:*>10}", float_num);

    let tuple = (20, 32.0);
    println!("{:0>8} {:.2}", tuple.0, tuple.1);

    // Tuples
    let (a, b) = tuple;
    println!("{:0>8} {:.2}", a, b);

    // Structs
    let something = SomeStruct::new("Text", "More text");
    println!("{:?}", something);

    // Arrays
    let array = [1, 3, 5, 7, 9];
    for num in array.iter() {
        println!("Num: {}", num);
    }

    let array_2 = ["Hi!"; 5];
    println!("{} {}", array_2[2], array_2[1]);
    println!("{:?}", array_2);

    // Vectors
    let mut vector = vec![1, 2, 3];
    vector.push(4);
    for num in vector.iter() {
        println!("Num: {}", num);
    }
}

#[derive(Debug)]
struct SomeStruct<'a> {
    _a: &'a str,
    _b: &'a str,
}

impl<'a> SomeStruct<'a> {
    pub fn new(a: &'a str, b: &'a str) -> Self {
        Self { _a: a, _b: b }
    }
}
