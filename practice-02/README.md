# Rust Class, Practice Session 2

This practice session deals with the `Option` and the `Result` types,
with pattern matching, error handling, strings, slices, iteration,
and closures.

Use `cargo run` to run the main function.

## Exercise 1

The file `main.rs` defines a struct representing persons. The `main` function defines
a vector of sample persons.

Write a function that searches a vector of persons for a person with a given ID. The function
should have the following signature:

```rust
pub fn lookup_person(persons: &[Person], id: i32) -> Option<&Person>
```

Use a `for`-loop to iterate over the vector of persons.

## Exercise 2

Program a variant `lookup_person2` of the function from exercise 1, with the same signature.
This time, you should use the `iter()` method of a vector to get an iterator. Iterator
has a `filter` method, which you need to pass a closure matching the person
you want to find.

## Exercise 3

Write a function `lookup_first_name` that retrieves the first name of a person with some ID
from a vector of persons. Use the following signature:

```rust
pub fn lookup_first_name(persons: &[Person], id: i32) -> Option<&str>
```

Invoke `lookup_person` for your solution and use explicit pattern matching via
`match`.

## Exercise 4

Write a function `lookup_last_name` that retrieves the last name of a person with some ID
from a vector of persons. Use the following signature:

```rust
pub fn lookup_last_name(persons: &[Person], id: i32) -> Option<&str>
```

Again, invoke `lookup_person` for your solution. But this time, use the `map` method
of the `Option` type to convert the resulting value of type `Option<&Person>`
into a value of type `Option<&str>`. Getting the conversion right is slightly tricky
and requires the dereference operator `*`.

## Exercise 5

Write a function `read_file` for reading a file as a string.

```rust
pub fn read_file(path: &str) -> Result<String, io::Error>
```

You need to following functions/methods from Rust's standard library:

* `File::open(p)` takes a pathname `p` (a `&str` value) and returns `io::Result<File>`.
* `f.read_to_string(s)`, where `f` is a file and `s` is a mutable string reference,
  writes the contens of the file into `s`. The result has type `io::Result<usize>`.

Use explicit pattern matching for your solution.

## Exercise 6

Improve `read_file` from the preceding exercise to write a new function
`read_file2` with the same signature. This time, you should use the question mark operator
`?` to propagate errors directly.
