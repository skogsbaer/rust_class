use std::collections::HashMap;
use std::fmt::Display;
use std::net::TcpListener;
use std::net::TcpStream;
use std::io;
use std::io::Write;
use rayon::*; // for async

fn sample1() {
    let mut v = vec![10, 11];
    let vptr = &v[1];      // Alias, points into v
    v.push(12);            // Mutate the vector
    //println!("{}", *vptr); // Compile error
}

fn consume(w: Vec<i32>) {
    println!("Length of vector: {}", w.len());
    // Memory of w is released automatically
}

fn sample2() {
    let mut v = vec![10, 11];
    consume(v); // Transfers ownership (call by value)
    //v.push(12); // Compile error
}

fn just_use(w: &Vec<i32>) {
    println!("Length of vector: {}", w.len());
    // No release
}

fn sample3() {
    let mut v = vec![10, 11];
    just_use(&v); // Borrows reference
    v.push(12);   // Works
}

fn sample4() {
    let mut v = vec![10,11];
    join(|| println!("v[1] = {}", &v[1]),
         || println!("v[1] = {}", &v[1]));
}

fn sample5() {
    let a = 42;      // immutable
    let mut b = 0;   // mutable
    let c: i32 = 5;  // optional type annotation
}

fn some_function(x: i32, y: i32) -> i32 {
    let z = x + y;
    z + 1
}

fn control_structures() -> i32 {
    let mut a = 5;
    if a == 0 {
        println!("zero");
    } else {
        println!("not zero");
    }
    for x in vec![1,2,3] {
        println!("{:?}", x);
    }
    while a > 0 {
        a = a - 1;
    }
    let res = Some(4);
    match res {
        None => 1,
        Some(i) => i
    }
}

enum Json {
    Object(HashMap<String, Json>),
    Array(Vec<Json>),
    String(String),
    Number(f64),
    Bool(bool),
    Null,
}

fn is_primitive(j: &Json) -> bool {
    match j {
        Json::Object(_) | Json::Array(_) => false,
        _ => true
    }
}
//type Result<T> = Result<T, Error>; // specialized result for io

fn start_tcp_server_1() -> io::Result<()> {
    match TcpListener::bind("127.0.0.1:7878") {
        Err(err) => Err(err),
        Ok(listener) => {
            match listener.accept() {
                Err(err) => Err(err),
                Ok((mut stream, _)) => {
                    match stream.write(&[1]) {
                        Err(err) => Err(err),
                        Ok(bytes_written) =>
                            if bytes_written == 1 {
                                Ok(())
                            } else {
                                Err(io::Error::new(io::ErrorKind::Other, "error"))
                            }
                    }
                }
            }
        }
    }
}

fn start_tcp_server_2() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7878")?;
    let (mut stream, _) = listener.accept()?;
    let bytes_written = stream.write(&[1])?;
    if bytes_written == 1 {
        Ok(())
    } else {
        Err(io::Error::new(io::ErrorKind::Other, "error"))
    }
}

fn start_tcp_server_3() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let (mut stream, _) = listener.accept().unwrap();
    let bytes_written = stream.write(&[1]).unwrap();
    if bytes_written != 1 {
        panic!("error")
    }
}

fn option_closure() {
    let opt1 = Some("some string");
    let opt2 = opt1.map(|s| s.len());
    // opt2 is now Some(11)
}

fn print_all<T, I>(iter: I) where I : Iterator<Item=T>, T: Display {
    for (i, x) in iter.enumerate() {
        println!("Element at index {}: {}", i, x);
    }
}

fn play_with_strings() {
    let mut s = String::from("hello"); // turns a &str into String
    s.push_str(" world");
    println!("{}", s); // prints hello world
    let (first, second): (&str, &str) = s.split_at(5);
    println!("first={first}, second={second}"); // prints first=hello, second= world
    call_me(&s);
    call_me("foo")
}

fn call_me(s: &str) { }

fn play_with_vectors() {
    let mut v = Vec::new(); // type of v: Vec<i32>
    v.push(1);
    v.push(2);
    v.push(3);
    let sub = v.get(0..2); // type of sub: Option<&[i32]>
    println!("v={v:?}, sub={sub:?}"); // prints v=[1, 2, 3], sub=Some([1, 2])
    let v2 = vec![4,5,6];  // type of v: Vec<i32>
}

pub struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>
}

fn play_with_node() {
    let n1 = Node { data: 42, next: None };
    let n2 = Node { data: 10, next: Some(Box::new(n1)) };
    // Owner n2 goes out of scope => automatic free
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

fn main() {
    //start_tcp_server_1();
    //play_with_strings();
    play_with_vectors();
    let v = vec![1,2,3];
    print_all(v.iter());
}
