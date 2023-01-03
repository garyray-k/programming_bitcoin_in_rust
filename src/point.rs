use std::{ops::Add};

use crate::field_element::FieldElement;

#[derive(Clone, Copy, Debug)]
struct Point {
    x: Option<FieldElement>,
    y: Option<FieldElement>,
    a: FieldElement,
    b: FieldElement
}

impl Point {
    pub fn new(
        x: Option<FieldElement>, 
        y: Option<FieldElement>, 
        a: FieldElement, 
        b: FieldElement
    ) -> Self {
        if (x.is_none()) || (y.is_none()) {
            return Self {
                x,
                y,
                a,
                b
            }
        }
        if (y.unwrap().to_the_power_of(2)) != x.unwrap().to_the_power_of(3) + a * x.unwrap() + b {
            panic!("{:?}, {:?} is not on the curve.", x, y);
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
            panic!("{:?}, {:?} is not on the curve for this Point.", other.x, other.y);
        }

        if self.x.is_none() || self.y.is_none() {
            return other;
        }
        if other.x.is_none() || other.y.is_none() {
            return self;
        }
        let self_x = self.x.unwrap();
        let self_y = self.y.unwrap();
        let other_x = other.x.unwrap();
        let other_y = other.y.unwrap();
        let zero = FieldElement::zero(self_x.get_prime());

        if ((self_y + other_y == zero) && (self_x == other_x)) || 
            (self == other && self_y == zero) {
            return Point::new(None, None, self.a, self.b);
        }

        let slope: usize;
        if self == other {
            slope = ((3 * self_x.to_the_power_of(2).get_number()) + self.a.get_number()) 
                    / (2 * self_y.get_number());
        } else {
            slope = ((other_y - self_y)
                    /(other_x - self_x)).get_number();
        }
    
        let x = slope.pow(2) - self_x.get_number() - other_x.get_number();
        let y = slope * (self_x.get_number() - x) - self_y.get_number();

        let x = Some(FieldElement::new(x, self_x.get_prime()));
        let y = Some(FieldElement::new(y, self_y.get_prime()));

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

    const PRIME: usize = 191;

    #[test]
    #[should_panic]
    fn bad_point() {
        let _b = Point::new(
            Some(FieldElement::new(1,PRIME)), 
            Some(FieldElement::new(1,PRIME)), 
            FieldElement::new(5, PRIME),
            FieldElement::new(7, PRIME)
        );
    }

    #[test]
    fn eq_works() {
        let a = Point::new(
            Some(FieldElement::new(1, PRIME)), 
            Some(FieldElement::new(1, PRIME)), 
            FieldElement::new(5, PRIME), 
            FieldElement::new(7, PRIME)
        );
        let b = Point::new(
            Some(FieldElement::new(1, PRIME)), 
            Some(FieldElement::new(1, PRIME)), 
            FieldElement::new(5, PRIME), 
            FieldElement::new(7, PRIME)
        );
        let c = Point::new(
            Some(FieldElement::new(18, PRIME)), 
            Some(FieldElement::new(77, PRIME)), 
            FieldElement::new(5, PRIME), 
            FieldElement::new(7, PRIME)
        );
        
        assert!(a == b);
        assert!(a != c);
    }

    #[test]
    fn add_identity_test() {
        let p1 = Point::new(
            Some(FieldElement::new(1, PRIME)), 
            Some(FieldElement::new(1, PRIME)), 
            FieldElement::new(5, PRIME), 
            FieldElement::new(7, PRIME)
        );
        let p2 = Point::new(
            Some(FieldElement::new(1, PRIME)), 
            Some(FieldElement::new(1, PRIME)), 
            FieldElement::new(5, PRIME), 
            FieldElement::new(7, PRIME)
        );
        let identity_point = Point::new(
            None, 
            None, 
            FieldElement::new(5, PRIME), 
            FieldElement::new(7, PRIME)
        );
        
        // exercise 3
        assert!(p1 + identity_point == p1);
        assert!(p2 + identity_point == p2);
        assert!(p1 + p2 == identity_point);
    }

    #[test]
    fn add_test() {
        // exercise 4 and 5
        // For the curve y 2 = x 3 + 5x + 7, what is (2,5) + (–1,–1)?
        let p1 = Point::new(
            Some(FieldElement::new(1, PRIME)), 
            Some(FieldElement::new(1, PRIME)), 
            FieldElement::new(5, PRIME), 
            FieldElement::new(7, PRIME)
        );
        let p2 = Point::new(
            Some(FieldElement::new(1, PRIME)),
            Some(FieldElement::new(1, PRIME)), 
            FieldElement::new(5, PRIME),
            FieldElement::new(7, PRIME)
        );
        let expected = Point::new(
            Some(FieldElement::new(3, PRIME)), 
            Some(FieldElement::new(7, PRIME)), 
            FieldElement::new(5, PRIME),
            FieldElement::new(7, PRIME)
        );

        assert_eq!(p1 + p2, expected);
    }

    #[test]
    fn add_self_test() {
        // add to itself
        let p1 = Point::new(
            Some(FieldElement::new(1, PRIME)), 
            Some(FieldElement::new(1, PRIME)), 
            FieldElement::new(5, PRIME), 
            FieldElement::new(7, PRIME)
        );
        let p2 = Point::new(
            Some(FieldElement::new(1, PRIME)), 
            Some(FieldElement::new(1, PRIME)), 
            FieldElement::new(5, PRIME), 
            FieldElement::new(7, PRIME)
        );
        let expected = Point::new(
            Some(FieldElement::new(18, PRIME)), 
            Some(FieldElement::new(77, PRIME)),
            FieldElement::new(5, PRIME), 
            FieldElement::new(7, PRIME),
        );
        
        assert_eq!(p1 + p2, expected);
    
    }
}
