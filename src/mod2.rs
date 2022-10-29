struct Site {
    domain: String,
    name: String,
    nation: String,
    found: u32,
}

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

pub fn test1() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("rect1's area is {}", rect1.area());
}

pub fn test2() {
    struct Color(u8, u8, u8);
    struct Point(f64, f64);

    let black = Color(0, 0, 0);
    let origin = Point(0.0, 0.0);

    println!("black = ({}, {}, {})", black.0, black.1, black.2);
    println!("origin = ({}, {})", origin.0, origin.1);
}

fn longer<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s2.len() > s1.len() {
        s2
    } else {
        s1
    }
}

pub fn test3() {
    let s1 = String::from("rust");
    let s2 = String::from("ecmascript");
    let r = longer(&s1, &s2);
    println!("{} is longer", r);
}

pub fn test4() {
    let s = String::from("hello中文");
    for c in s.chars() {
        println!("{}", c);
    }
}
use std::collections::HashMap;
pub fn test5() {
    let mut map = HashMap::new();
    map.insert(1, "a");

    if let Some(x) = map.get_mut(&1) {
        *x = "b";
    }
    for p in map.iter() {
        println!("{:?}", p);
    }
}
use std::thread;
use std::time::Duration;
pub fn test6() {
    let handle = thread::spawn(|| {
        for i in 0..5 {
            println!("spawned thread print {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 0..3 {
        println!("main thread print {}", i);
        thread::sleep(Duration::from_millis(1));
    }

    let inc = |num: i32| -> i32 { num + 1 };
    println!("inc(5) = {}", inc(5));
    handle.join().unwrap();
}

pub fn test7() {
    let s = "hello";

    let handle = thread::spawn(move || {
        println!("{}", s);
    });
    println!("{}", s);

    handle.join().unwrap();
}

use std::sync::mpsc;

pub fn test8() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi zhaoq");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}

pub fn test9() {
    let doubler = |x: i32| -> i32 { x *2 };
    let value = 5;
    let twice = doubler(value);
    println!("{}doubledis{}", value, twice);
    let big_closure = |b, c|->i32 {
        let z = b + c;
        z * twice
    };
    let some_number = big_closure(1, 2);
    println!("Resultfromclosure:{:?}", some_number);
}
