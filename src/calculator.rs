use std::ops::{Add, Div, Mul, Sub};

/// ## Description
/// This trait defines methods for primitive math operations: '+', '-', '*', '/'
pub trait Operator {
    /// ## Description
    /// Performs '+' operation
    fn add<T>(a: T, b: T) -> T
    where
        T: Add<Output = T>;

    /// ## Description
    /// Performs '-' operation
    fn sub<T>(a: T, b: T) -> T
    where
        T: Sub<Output = T>;

    /// ## Description
    /// Performs '*' operation
    fn mul<T>(a: T, b: T) -> T
    where
        T: Mul<Output = T>;

    /// ## Description
    /// Performs '/' operation
    fn div<T>(a: T, b: T) -> T
    where
        T: Div<Output = T>;
}

pub struct Calculator {}

impl Operator for Calculator {
    fn add<T>(a: T, b: T) -> T
    where
        T: Add<Output = T>,
    {
        a + b
    }

    fn sub<T>(a: T, b: T) -> T
    where
        T: Sub<Output = T>,
    {
        a - b
    }

    fn mul<T>(a: T, b: T) -> T
    where
        T: Mul<Output = T>,
    {
        a * b
    }

    fn div<T>(a: T, b: T) -> T
    where
        T: Div<Output = T>,
    {
        a / b
    }
}

mod test {
    use std::ops::{Add, Div, Mul, Sub};

    #[derive(Debug, PartialEq)]
    struct Point {
        x: i32,
        y: i32,
    }

    impl Add for Point {
        type Output = Self;

        fn add(self, rhs: Self) -> Self {
            Point {
                x: self.x + rhs.x,
                y: self.y + rhs.y,
            }
        }
    }

    impl Sub for Point {
        type Output = Self;

        fn sub(self, rhs: Self) -> Self {
            Point {
                x: self.x - rhs.x,
                y: self.y - rhs.y,
            }
        }
    }

    impl Mul for Point {
        type Output = Point;

        fn mul(self, rhs: Self) -> Self {
            Point {
                x: self.x * rhs.x,
                y: self.y * rhs.y,
            }
        }
    }

    impl Div for Point {
        type Output = Self;

        fn div(self, rhs: Self) -> Self {
            Point {
                x: self.x / rhs.x,
                y: self.y / rhs.y,
            }
        }
    }

    #[test]
    fn calculator_add_works() {
        use crate::calculator::{Calculator, Operator};

        let num1: u64 = 19;
        let num2: u64 = 15;
        assert_eq!(Calculator::add(num1, num2), 34);

        let num1: i64 = 20;
        let num2: i64 = 5;
        assert_eq!(Calculator::add(num1, num2), 25);

        let num1: f64 = 18.5;
        let num2: f64 = 14.6;
        assert_eq!(Calculator::add(num1, num2), 33.1);

        let p1 = Point { x: 48, y: 56 };
        let p2 = Point { x: 7, y: 8 };
        assert_eq!(Calculator::add(p1, p2), Point { x: 55, y: 64 });
    }

    #[test]
    fn calculator_sub_works() {
        use crate::calculator::{Calculator, Operator};

        let num1: u64 = 19;
        let num2: u64 = 15;
        assert_eq!(Calculator::sub(num1, num2), 4);

        let num1: i64 = 20;
        let num2: i64 = 5;
        assert_eq!(Calculator::sub(num1, num2), 15);

        let num1: f64 = 18.5;
        let num2: f64 = 14.6;
        assert_eq!(format!("{:.2}", Calculator::sub(num1, num2)), "3.90");

        let p1 = Point { x: 48, y: 56 };
        let p2 = Point { x: 7, y: 8 };
        assert_eq!(Calculator::sub(p1, p2), Point { x: 41, y: 48 });
    }

    #[test]
    fn calculator_mul_works() {
        use crate::calculator::{Calculator, Operator};

        let num1: u64 = 19;
        let num2: u64 = 15;
        assert_eq!(Calculator::mul(num1, num2), 285);

        let num1: i64 = 20;
        let num2: i64 = 5;
        assert_eq!(Calculator::mul(num1, num2), 100);

        let num1: f64 = 18.5;
        let num2: f64 = 14.6;
        assert_eq!(format!("{:.2}", Calculator::mul(num1, num2)), "270.10");

        let p1 = Point { x: 48, y: 56 };
        let p2 = Point { x: 7, y: 8 };
        assert_eq!(Calculator::mul(p1, p2), Point { x: 336, y: 448 });
    }

    #[test]
    fn calculator_div_works() {
        use crate::calculator::{Calculator, Operator};

        let num1: u64 = 19;
        let num2: u64 = 15;
        assert_eq!(Calculator::div(num1, num2), 1);

        let num1: i64 = 20;
        let num2: i64 = 5;
        assert_eq!(Calculator::div(num1, num2), 4);

        let num1: f64 = 18.5;
        let num2: f64 = 14.6;
        assert_eq!(format!("{:.5}", Calculator::div(num1, num2)), "1.26712");

        let p1 = Point { x: 48, y: 56 };
        let p2 = Point { x: 7, y: 8 };
        assert_eq!(Calculator::div(p1, p2), Point { x: 6, y: 7 });
    }

    #[test]
    #[should_panic(expected = "attempt to divide by zero")]
    fn should_panic_on_zero_division() {
        use crate::calculator::{Calculator, Operator};

        let num1: u64 = 10;
        let num2: u64 = 0;
        Calculator::div(num1, num2);
    }
}
