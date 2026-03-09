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
    //v.push(12);            // Mutate the vector
    println!("{}", vptr); // Compile error
}

fn consume(w: Vec<i32>) {
    println!("w addr {:p}", &w);
    println!("Length of vector: {}", w.len());
    // Memory of w is released automatically
}

fn sample2() {
    let mut v = vec![10, 11];
    println!("v addr {:p}", &v);
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

use std::mem;

// This function borrows a slice.
fn analyze_slice(slice: &[i32]) {
    println!("First element of the slice: {}", slice[0]);
    println!("The slice has {} elements", slice.len());
}

fn nested_array_samples() {
    let matrix: [[i32; 3]; 2] = [[1,2,3],[4,5,6]];
    analyze_slice(&matrix[0]);
    analyze_slice(&matrix[1]);
}

fn array_samples() {
    // Fixed-size array (type signature is optional).
    let xs: [i32; 5] = [1, 2, 3, 4, 5];

    // All elements can be initialized to the same value.
    let ys: [i32; 500] = [0; 500];

    // Indexing starts at 0.
    println!("First element of the array: {}", xs[0]);
    println!("Second element of the array: {}", xs[1]);

    // `len` returns the count of elements in the array.
    println!("Number of elements in array: {}", xs.len());

    // Arrays are stack allocated.
    println!("Array xs occupies {} bytes", mem::size_of_val(&xs));
    println!("Array ys occupies {} bytes", mem::size_of_val(&ys));

    // Arrays can be automatically borrowed as slices.
    println!("Borrow the whole array as a slice.");
    analyze_slice(&xs);

    // Slices can point to a section of an array.
    // They are of the form [starting_index..ending_index].
    // `starting_index` is the first position in the slice.
    // `ending_index` is one more than the last position in the slice.
    println!("Borrow a section of the array as a slice.");
    analyze_slice(&ys[1 .. 4]);

    let xs_slice: &[i32] = &xs;
    let ys_slice: &[i32] = &ys[1 .. 4];
    // Example of empty slice `&[]`:
    let empty_array: [u32; 0] = [];
    assert_eq!(&empty_array, &[]);
    assert_eq!(&empty_array, &[][..]); // Same but more verbose

    // Arrays can be safely accessed using `.get`, which returns an
    // `Option`. This can be matched as shown below, or used with
    // `.expect()` if you would like the program to exit with a nice
    // message instead of happily continue.
    for i in 0..xs.len() + 1 { // Oops, one element too far!
        match xs.get(i) {
            Some(xval) => println!("{}: {}", i, xval),
            None => println!("Slow down! {} is too far!", i),
        }
    }

    // Out of bound indexing on array with constant value causes compile time error.
    //println!("{}", xs[5]);
    // Out of bound indexing on slice causes runtime error.
    //println!("{}", xs[..][5]);
}

fn vector_samples() {
    // Vectors
    let mut v1: Vec<i32> = vec![1, 2, 3, 4, 5];
    let mut v2: Vec<i32> = vec![42; 500];

    // Vectors can grow or shrink
    v1.push(6);
    v2.push(1);

    // Vectors are heap allocated.
    println!("Vector v1 occupies {} bytes", mem::size_of_val(&v1));
    println!("Vector v2 occupies {} bytes", mem::size_of_val(&v2));

    println!("Borrow whole vector as a slice.");
    analyze_slice(&v1);

    println!("Borrow a section of vector as a slice.");
    analyze_slice(&v2[3..6]);

    let v1_slice: &[i32] = &v1;
    let v2_slice: &[i32] = &v2[1 .. 4];
    analyze_slice(v1_slice);
    analyze_slice(v2_slice);
}

macro_rules! print_result {
    // This macro takes an expression of type `expr` and prints
    // it as a string along with its result.
    // The `expr` designator is used for expressions.
    ($expression:expr) => {
        // `stringify!` will convert the expression *as it is* into a string.
        println!("{:?} = {:?}",
                 stringify!($expression),
                 $expression);
    };
}

fn macro_samples() {
    print_result!(1 + 2);
}

fn main() {
    //start_tcp_server_1();
    //play_with_strings();
    //play_with_vectors();
    //let v = vec![1,2,3];
    //print_all(v.iter());
    //sample2();
    //sample1();
    array_samples();
    vector_samples();
    //nested_array_samples();
    //macro_samples();
}
