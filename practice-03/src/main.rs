// This code is based on https://rust-unofficial.github.io/too-many-lists/index.html
// (Section 3, "An Ok Stack")

pub struct Stack<T> {
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        panic!("not implemented");
    }

    pub fn push(&mut self, elem: T) {
        panic!("not implemented");
    }

    pub fn pop(&mut self) -> Option<T> {
        panic!("not implemented");
    }

    pub fn peek(&self) -> Option<&T> {
        panic!("not implemented");
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        panic!("not implemented");
    }

    pub fn iter<'a>(&'a self) -> Iter<'a, T> {
        panic!("not implemented");
    }
}

fn main() {
    println!("main function not implemented");
}

pub struct Iter<'a, T> {
    cur: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        panic!("not implemented");
    }
}

#[cfg(test)]
mod test {
    use super::Stack;

    #[test]
    fn basics() {
        let mut stack = Stack::new();

        // Check empty stack behaves right
        assert_eq!(stack.pop(), None);

        // Populate stack
        stack.push(1);
        stack.push(2);
        stack.push(3);

        // Check normal removal
        assert_eq!(stack.pop(), Some(3));
        assert_eq!(stack.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        stack.push(4);
        stack.push(5);

        // Check normal removal
        assert_eq!(stack.pop(), Some(5));
        assert_eq!(stack.pop(), Some(4));

        // Check exhaustion
        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.pop(), None);
    }

    /*
    #[test]
    fn peek() {
        let mut stack = Stack::new();
        assert_eq!(stack.peek(), None);
        assert_eq!(stack.peek_mut(), None);
        stack.push(1); stack.push(2); stack.push(3);

        assert_eq!(stack.peek(), Some(&3));
        assert_eq!(stack.peek_mut(), Some(&mut 3));

        stack.peek_mut().map(|value| {
            *value = 42
        });

        assert_eq!(stack.peek(), Some(&42));
        assert_eq!(stack.pop(), Some(42));
    }
    */

    /*
    #[test]
    fn iter() {
        let mut stack = Stack::new();
        stack.push(1); stack.push(2); stack.push(3);

        let mut iter = stack.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
    }

    #[test]
    fn iter2() {
        let mut stack = Stack::new();
        stack.push(1); stack.push(2); stack.push(3);

        let mut v: Vec<i32> = Vec::new();
        for x in stack.iter() {
            v.push(*x);
        }
        assert_eq!(v, vec![3, 2, 1]);
    }
    */
}
