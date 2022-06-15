# Rust Class, Practice Session 3

This practice session deals with the heap-allocated data, generics,
and lifetimes. We will implement a simple stack data structure. The
code is based on Section 3 ("An Ok Stack") from the excellent
(but advanced) tutorial
[Learning Rust With Entirely Too Many Linked Lists](https://rust-unofficial.github.io/too-many-lists/index.html).

Use `cargo test` to run the tests.

## Exercise 1

File `main.rs` defines the layout, method signatures, and tests for our
generic stack datastructure.

Implement the methods `new`, `push`, and `pop`. Then you should be able
to run the first test using `cargo test`.

## Exercise 2

Implement the methods `peek` and `peek_mut`. Uncomment the test `peek` and
run it.

## Exercise 3

Add iterator support for the stack datastructure. The `iter` method returns
a value of type `Iter<'a, T>`, where `'a` is the lifetime of the whole stac
and `T` is the type of the stack elements. The `Iter` type should then
implement the `Iterator` trait. Every type that implements `Iterator`
supports iteration via the `for`-loop.

Fill in the missing pieces, uncomment the two tests `iter` and `iter2`, and
make sure the tests are running.

