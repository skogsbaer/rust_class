// Declare
#[derive(Debug, PartialEq)]
pub struct Point2D {
    x: i32,
    y: i32
}

// pub fn new(x: i32, y: i32) -> Point2D { }

/*
pub fn problem1() {
    let p1 = new(1, 2);
    let p2 = p1;
    println!("p1={:?}", p1);
    println!("p2={:?}", p2);
}
*/

/*
pub fn problem2() {
    let mut p1 = new(1, 2);
    let p2: &Point2D = &p1;
    move_mut(&mut p1, 5, 2);
    print_point(p2);
}
*/

fn main() {
    println!("Hello, world!");
    //let p = new(1, 2);
    //println!("point: {:?}", p);
}

#[cfg(test)]
mod tests {
    use crate::*;

    /*
    #[test]
    fn test_is_zero() {
        let p1 = new(1, 2);
        assert_eq!(is_zero(&p1), false);
        assert_eq!(is_zero(&p1), false);
        let p2 = new(0, 0);
        assert_eq!(is_zero(&p2), true);
    }
    */

    /*
    #[test]
    fn test_move_new() {
        let p1 = new(1, 2);
        let p2 = move_new(&p1, -2, 3);
        assert_eq!(p2, new(-1, 5));
        assert_eq!(p1, new(1, 2));
    }
    */

    /*
    #[test]
    fn test_move_mut() {
        let mut p1 = new(1, 2);
        move_mut(&mut p1, -2, 3);
        assert_eq!(p1, new(-1, 5));
    }
    */
}
