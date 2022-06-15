use std::io::{self, Read};
use std::fs::File;

#[derive(Debug)]
pub struct Person {
    id: i32,
    first_name: String,
    last_name: String
}

pub fn lookup_person(persons: &[Person], id: i32) -> Option<&Person> {
    panic!("not implemented")
}

pub fn lookup_person2(persons: &[Person], id: i32) -> Option<&Person> {
    panic!("not implemented")
}


pub fn lookup_first_name(persons: &[Person], id: i32) -> Option<&str> {
    panic!("not implemented")
}

pub fn lookup_last_name(persons: &[Person], id: i32) -> Option<&str> {
    panic!("not implemented")
}

pub fn read_file(path: &str) -> Result<String, io::Error> {
    panic!("not implemented")
}

pub fn read_file2(path: &str) -> Result<String, io::Error> {
    panic!("not implemented")
}

fn main() {
    let persons = vec![
        Person{id: 1, first_name: "Alan".to_string(), last_name: "Turing".to_string()},
        Person{id: 2, first_name: "Frances".to_string(), last_name: "Allen".to_string()},
        Person{id: 3, first_name: "Christiane".to_string(), last_name: "Floyd".to_string()},
        Person{id: 4, first_name: "Donald".to_string(), last_name: "Knuth".to_string()}
    ];
    let p1 = lookup_person(&persons, 2);
    let p2 = lookup_person(&persons, 11);
    println!("p1: {:?}", p1);
    println!("p2: {:?}", p2);
    let s = read_file("src/main.rs").unwrap();
    for l in s.lines().take(1) {
        println!("first line: {}", l)
    }
}
