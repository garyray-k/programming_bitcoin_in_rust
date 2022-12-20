use std::{f64::INFINITY, ops::Add};

#[derive(Clone, Copy, Debug)]
struct Point {
    x: f64,
    y: f64,
    a: i8,
    b: i8
}

impl Point {
    pub fn new(x: f64, y: f64, a: i8, b: i8) -> Self {
        if (x == INFINITY) || (y == INFINITY) {
            return Self {
                x,
                y,
                a,
                b
            }
        }
        if (y.powf(2.0)) != x.powf(3.0) + a as f64 * x + b as f64 {
            panic!("{}, {} is not on the curve.", x, y);
        }
        Self {
            x,
            y,
            a,
            b
        }
    }
}

impl Add for Point {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        if (other.a != self.a) || (other.b != self.b) {
            panic!("{}, {} is not on the curve for this Point.", other.x, other.y);
        }

        if self.x == INFINITY {
            return other;
        }
        if other.x == INFINITY {
            return self;
        }
        if ((self.y + other.y == 0.0) && (self.x == other.x)) || 
            (self == other && self.y == 0.0) {
            return Point::new(INFINITY, INFINITY, self.a, self.b);
        }

        let slope: f64;
        if self == other {
            slope = ((3.0 * self.x.powf(2.0)) + self.a as f64) / (2.0 * self.y);
        } else {
            slope = (other.y - self.y)/(other.x - self.x);
        }
    
        let x = slope.powf(2.0) - self.x - other.x;
        let y = slope * (self.x - x) - self.y;

        return Point::new(x, y, self.a, self.b);
    }
}


impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        return self.a == other.a && self.b == other.b && self.x == other.x && self.y == other.y;
    }
}

impl Eq for Point {}

#[cfg(test)]
mod point_tests {

    use super::*;

    #[test]
    #[should_panic]
    fn bad_point() {
        let b = Point::new(-1.0, -2.0, 5, 7);
    }

    #[test]
    fn eq_works() {
        let a = Point::new(-1.0, -1.0, 5, 7);
        let b = Point::new(-1.0, -1.0, 5, 7);
        let c = Point::new(18.0, 77.0, 5, 7);
        
        assert!(a == b);
        assert!(a != c);
    }

    #[test]
    fn add_test() {
        let p1 = Point::new(-1.0, -1.0, 5, 7);
        let p2 = Point::new(-1.0, 1.0, 5, 7);
        let identity_point = Point::new(INFINITY, INFINITY, 5, 7);
        
        // exercise 3
        assert!(p1 + identity_point == p1);
        assert!(p2 + identity_point == p2);

        assert!(p1 + p2 == identity_point);

        // exercise 4 and 5
        // For the curve y 2 = x 3 + 5x + 7, what is (2,5) + (–1,–1)?
        let p1 = Point::new(2.0, 5.0, 5, 7);
        let p2 = Point::new(-1.0, -1.0, 5, 7);
        let expected = Point::new(3.0, -7.0, 5, 7);

        assert_eq!(p1 + p2, expected);

        // add to itself
        let p1 = Point::new(-1.0, -1.0, 5, 7);
        let p2 = Point::new(-1.0, -1.0, 5, 7);
        let expected = Point::new(18.0, 77.0, 5, 7);
        
        assert_eq!(p1 + p2, expected);
    
    }
}
