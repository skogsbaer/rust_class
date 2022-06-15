use std::io::{self, Read};
use std::fs::File;

#[derive(Debug)]
pub struct Person {
    id: i32,
    first_name: String,
    last_name: String
}

pub fn lookup_person(persons: &[Person], id: i32) -> Option<&Person> {
    for p in persons {
        if p.id == id {
            return Some(p)
        }
    }
    None
}

pub fn lookup_person2(persons: &[Person], id: i32) -> Option<&Person> {
    persons.iter().find(|p| p.id == id)
}


pub fn lookup_first_name(persons: &[Person], id: i32) -> Option<&str> {
    match lookup_person(persons, id) {
        None => None,
        Some(p) => Some(&p.first_name)
    }
}

pub fn lookup_last_name(persons: &[Person], id: i32) -> Option<&str> {
    lookup_person(persons, id).map(|p| &*p.last_name)
    // This looks like magic, especially the &*p.last_name part.
    // Let us explain:
    // - lookup_person(persons, id) has type Option<&Person>
    // - map transforms the content of an Option if it's a Some.
    // - `|p| &*p.last_name` is a closure, where p has type &Person.
    // - p.last_name has type String
    // - &*p.last_name is parsed as &(*(p.last_name)).
    // - If we had written &p.last_name, we would return a value of type
    //   Option<&String>, but we need a Option<&str> value.
    // - * is the dereferencing operator, and type String dereferences to
    //   str. Hence, *p.last_name has type str.
    // - Thus, the type of &*p.last_name is &str, as required.
    // If you don't get this right now: nevermind!
}

pub fn read_file(path: &str) -> Result<String, io::Error> {
    let file_or_err = File::open(path);
    let mut f = match file_or_err {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

pub fn read_file2(path: &str) -> Result<String, io::Error> {
    let mut f = File::open(path)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
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
