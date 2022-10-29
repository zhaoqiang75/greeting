mod mod2;
mod mod3;
mod mod4;
fn main() {
    let name = "zhaoqiang";
    println!("Hello, {}!", name);
    println!("name is {0}, name again is {0}", name);
    println!("{{或}}");
    test1();
    let c = add(1, 2);
    println!("c is {}", c);
    // panic!("错误，程序退出");
    // test2();
    // test3();
    // test4();
    // test7();
    // test8();
    // mod2::test1();
    // mod2::test2();
    // mod2::test3();
    // mod2::test4();
    mod2::test5();
    mod2::test6();
    mod2::test7();
    mod2::test8();
    mod2::test9();
    // mod3::test1();
    // mod4::test1();
}
fn test1() {
    let x = 5;

    let y = {
        let x = 3;
        x + 1
    };

    println!("x 的值为 : {}", x);
    println!("y 的值为 : {}", y);
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn test2() {
    let a = 0;
    let number = if a > 0 { 1 } else { -1 };
    println!("number 为 {}", number);
}

fn test3() {
    let a = [10, 20, 30, 40, 50];
    for i in a.iter() {
        println!("值为 : {}", i);
    }
    let a = [100, 200, 300, 400, 500];
    for i in 0..5 {
        println!("a[{}] = {}", i, a[i]);
    }
    let s = ['R', 'U', 'N', 'O', 'O', 'B'];
    let mut i = 0;
    let location = loop {
        if s[i] == 'O' {
            break i;
        }
        i += 1;
    };
    println!(" \'O\' 的索引为 {}", location);
}

fn test4() {
    let s = String::from("broadcast");

    let part1 = &s[0..5];
    let part2 = &s[5..9];

    println!("{}={}+{}", s, part1, part2);
}

fn test7() {
    enum Book {
        Papery(u32),
        Electronic { url: String },
        Both(u32, String),
    }

    let book = Book::Papery(1001);
    let ebook = Book::Electronic {
        url: String::from("url..."),
    };
    let bbook = Book::Both(1, String::from("www.baidu.com"));

    match bbook {
        Book::Papery(i) => {
            println!("Papery book {}", i);
        }
        Book::Electronic { url } => {
            println!("E-book {}", url);
        }
        Book::Both(i, url) => {
            println!("Both {}-{}", i, url);
        }
    }
}

fn test8() {
    let t = None;
    match t {
        Some(64) => println!("Yes"),
        _ => println!("No"),
    }
    let i = 0;
    if let 0 = i {
        println!("zero");
    }
}
