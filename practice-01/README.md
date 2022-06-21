# Rust Class, Practice Session 1

This practice session deals with basic rust features (variables,
functions, ...), as well as with ownership, borrowing, and mutability.

Use `cargo run` to run the main function, `cargo test` executes the tests.

## Exercise 0

Make sure that your setup is working. Go inside the `practice-01`
directory. Run the program for this exercise by executing

```
$ cargo run
```

in your shell. (The `$` at the start of the line is a shell prompt). You
should see an output like this:

```
   Compiling practice-01 v0.1.0 (/Users/swehr/repos/rust_class_ew2022/practice-01)
    Finished dev [unoptimized + debuginfo] target(s) in 0.27s
     Running `target/debug/practice-01`
Hello, world!
```

Next, execute the tests:

```
$ cargo test
```

You will see a warning because of an unused import, but at the
end of the output, you should find the following:

```
running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

That's ok, we haven't defined any tests so far.


## Exercise 1

In file `src/main.rs`, you find the definition of `Point2D`, a simple
struct for two dimensional points.

Write a function `new` with the following signature:

```rust
pub fn new(x: i32, y: i32) -> Point2D
```

This function should just return a point with the given coordinates.

You should now be able to remove the comments from the last two lines
of the `main` function. Executing `cargo run` gives

```
   Compiling practice-01-solution v0.1.0 (/Users/swehr/repos/rust_class_ew2022/practice-01-solution)
    Finished dev [unoptimized + debuginfo] target(s) in 0.27s
     Running `target/debug/practice-01-solution`
Hello, world!
point: Point2D { x: 1, y: 2 }
```

## Exercise 2

Write a function `print_point` that accepts a point value and prints
the point to the console.

Add a call of `print_point` for point `p` to
the `main` function. Execute the program with `cargo run`.

Add a second call of `print_point` to the `main` function, again using
`p` as an argument. Does this work? If not, it could be necessary
that you have to change the signature of `print_point`.

## Exercise 3

Uncomment the function `problem1`. Compiling the program now fails with
the following error:

```
error[E0382]: borrow of moved value: `p1`
  --> src/main.rs:19:25
   |
17 |     let p1 = new(1, 2);
   |         -- move occurs because `p1` has type `Point2D`, which does not implement the `Copy` trait
18 |     let p2 = p1;
   |              -- value moved here
19 |     println!("p1={:?}", p1);
   |                         ^^ value borrowed here after move
   |
```

Explain what the problem is and fix it.

## Exercise 4

Write a function `is_zero` that takes a point and returns a `bool` value.
The return value should be `true` if both coordinates of the point are
0. Otherwise, it should return `false`.

In `main.rs`, there also exists a test for `is_zero`. You can now uncomment
the test and use `cargo test` to check whether the test is running.

## Exercise 5

Write a function `move_new` that takes a point `p`, an `x`-value, and a
`y`-value. The function then returns a new point that represents
`p` moved by `x` and `y`. The test for `move_new` is commented out, you should
now activate it.

## Exercise 6

Write a function `move_mut` that takes a point `p`, an `x`-value, and a
`y`-value. The function moves point `p` in-place. The test for `move_mut` is commented out, you should now activate it.

## Exercise 7

Uncomment the function `problem2`. Compiling the program now fails with
the following error:

```
error[E0502]: cannot borrow `p1` as mutable because it is also borrowed as immutable
  --> src/main.rs:40:14
   |
39 |     let p2: &Point2D = &p1;
   |                        --- immutable borrow occurs here
40 |     move_mut(&mut p1, 5, 2);
   |              ^^^^^^^ mutable borrow occurs here
41 |     print_point(p2);
   |                 -- immutable borrow later used here
```

Explain what the problem is and fix it.

## Exercise 8 (for the really fast hackers)

So far, we implemented all functionality as top-level functions. For example,
to print a point `p`, we used `print_point(p)`.

Often, Rust programmers prefer method-like invocation using the dot syntax, like this: `p.print()`

To attach methods to the `Point2D` struct, we use an `impl` block:

```rust
impl Point2D {
    pub fn new(x: i32, y: i32) -> Point2D {
        Point2D { x: x, y: y}
    }

    pub fn print(&self) {
        println!("point: {:?}", self);
    }

    // ...
}
```

A method that has `self` as its first parameter (such as `print`)
is then invoked using the dot syntax. The `self` parameter doesn't
need a type, you only specify whether it's a reference or not.

A method without a `self` parameter (such as `new`) is invoked
with an explicit namespace prefix: `Point2d::new(1, 2)`

Change your program so that invocations of `is_zero`, `move_new`, and `move_mut`
use the dot syntax.
