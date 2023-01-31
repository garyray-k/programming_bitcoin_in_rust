use std::fmt;
use std::ops::{Add, AddAssign};

use num::BigUint;

use super::field_element::FieldElement;

#[derive(Clone, Debug)]
struct Secp256k1Point {
    x: Option<FieldElement>,
    y: Option<FieldElement>,
    a: FieldElement,
    b: FieldElement,
}

impl Secp256k1Point {
    pub fn new(x: Option<FieldElement>, y: Option<FieldElement>) -> Self {
        let a = FieldElement::zero();
        let b = FieldElement::new(BigUint::from(7u64));
        if (x.is_none()) || (y.is_none()) {
            return Self { x, y, a, b };
        }
        let x = x.unwrap();
        let y = y.unwrap();
        if (y.to_the_power_of(2u64.into()))
            != x.to_the_power_of(3u64.into()) + a.clone() * x.clone() + b.clone()
        {
            panic!("{:?}, {:?} is not on the curve.", x, y);
        }
        Self {
            x: Some(x),
            y: Some(y),
            a,
            b,
        }
    }

    pub fn multiply_by(self, coefficient: u64) -> Secp256k1Point {
        let mut coef = coefficient;
        let mut current = self;
        let mut result = Self::infinity_point();
        while coef != 0 {
            if coef & 1 == 1 {
                result = result + current.clone();
            }
            current = current.clone() + current;
            coef >>= 1;
        }
        result
    }

    fn infinity_point() -> Secp256k1Point {
        Secp256k1Point::new(None, None)
    }
}

impl fmt::Display for Secp256k1Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Point: {{\n\t x:{:?}\n\t y:{:?}\n\t a:{:?}\n\t b:{:?}\n }}",
            self.x, self.y, self.a, self.b
        )
    }
}

impl Add for Secp256k1Point {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        if (other.clone().a != self.a) || (other.clone().b != self.b) {
            panic!(
                "{:?}, {:?} is not on the curve for this Point.",
                other.x, other.y
            );
        }

        if self.x.is_none() || self.y.is_none() {
            return other;
        }
        if other.clone().x.is_none() || other.clone().y.is_none() {
            return self;
        }
        let self_x = self.x.as_ref().unwrap();
        let self_y = self.y.as_ref().unwrap();
        let other_x = other.clone().x.unwrap();
        let other_y = other.clone().y.unwrap();
        let zero = FieldElement::zero();

        if ((self_y.clone() + other_y.clone() == zero) && (self_x.clone() == other_x.clone()))
            || (self == other && self_y.clone() == zero)
        {
            return Secp256k1Point::infinity_point();
        }

        let slope: FieldElement;

        if self == other {
            let x_to_the_second = self_x.to_the_power_of(2u64.into());
            slope = ((x_to_the_second.clone() + x_to_the_second.clone() + x_to_the_second)
                + self.a)
                / (self_y.clone() + self_y.clone());
        } else {
            slope = (other_y - self_y.clone()) / (other_x.clone() - self_x.clone());
        }

        let binding = slope.to_the_power_of(2u64.into());
        let x = &binding - self_x - other_x;
        let y = slope * (self_x - &x) - self_y.clone();

        let x = Some(x.clone());
        let y = Some(y.clone());

        return Secp256k1Point::new(x, y);
    }
}

impl AddAssign for Secp256k1Point {
    fn add_assign(&mut self, rhs: Self) {
        *self = self.clone() + rhs
    }
}

impl PartialEq for Secp256k1Point {
    fn eq(&self, other: &Self) -> bool {
        return self.a == other.a && self.b == other.b && self.x == other.x && self.y == other.y;
    }
}

impl Eq for Secp256k1Point {}

#[cfg(test)]
mod point_tests {

    use num::One;

    use super::*;

    #[test]
    #[should_panic]
    fn bad_point() {
        let _b = Secp256k1Point::new(
            Some(FieldElement::new(BigUint::one())),
            Some(FieldElement::new(BigUint::one())),
        );
    }

    #[test]
    fn eq_works() {
        // Had to find the points on the curve for use elsewhere.
        // let mut list = vec![];
        // panic::set_hook(Box::new(|_| {
        //     // do nothing
        // }));
        // (0..=191).for_each(|x| {
        //     (0..=191).for_each(|y| {
        //         let result = panic::catch_unwind(|| {
        //             Point::new(
        //                 Some(FieldElement::new(x, PRIME)),
        //                 Some(FieldElement::new(y, PRIME)),
        //                 FieldElement::new(0, PRIME),
        //                 FieldElement::new(7, PRIME),
        //             )
        //         });

        //         match result {
        //             Ok(value) => {
        //                 list.push(value);
        //             }
        //             Err(_) => (),
        //         }
        //     })
        // });
        // list.iter().for_each(|f| {
        //     println!(
        //         "{}, {} is on the curve.",
        //         f.x.unwrap().get_number(),
        //         f.y.unwrap().get_number()
        //     )
        // });

        let a = Secp256k1Point::new(
            Some(FieldElement::new(BigUint::one())),
            Some(FieldElement::new(BigUint::from(77u64))),
        );
        let b = Secp256k1Point::new(
            Some(FieldElement::new(BigUint::one())),
            Some(FieldElement::new(BigUint::from(77u64))),
        );
        let c = Secp256k1Point::new(
            Some(FieldElement::new(BigUint::from(180u64))),
            Some(FieldElement::new(BigUint::from(108u64))),
        );

        assert!(a == b);
        assert!(a != c);
    }

    #[test]
    fn add_identity_test() {
        let p1 = Secp256k1Point::new(
            Some(FieldElement::new(BigUint::one())),
            Some(FieldElement::new(BigUint::from(77u64))),
        );
        let p2 = Secp256k1Point::new(
            Some(FieldElement::new(BigUint::one())),
            Some(FieldElement::new(BigUint::from(77u64))),
        );
        let identity_point = Secp256k1Point::infinity_point();

        // exercise 3
        assert!(p1.clone() + identity_point.clone() == p1);
        assert!(p2.clone() + identity_point == p2);
    }

    #[test]
    fn add_test() {
        // exercise 4 and 5
        // For the curve y 2 = x 3 + 5x + 7, what is (2,5) + (–1,–1)?
        let p1 = Secp256k1Point::new(
            Some(FieldElement::new(BigUint::from(57u64))),
            Some(FieldElement::new(BigUint::from(180u64))),
        );
        let p2 = Secp256k1Point::new(
            Some(FieldElement::new(BigUint::from(47u64))),
            Some(FieldElement::new(BigUint::from(58u64))),
        );
        let expected = Secp256k1Point::new(
            Some(FieldElement::new(BigUint::from(190u64))),
            Some(FieldElement::new(BigUint::from(31u64))),
        );

        assert_eq!(p1 + p2, expected);
    }

    #[test]
    fn add_self_test() {
        // add to itself
        let p1 = Secp256k1Point::new(
            Some(FieldElement::new(BigUint::from(57u64))),
            Some(FieldElement::new(BigUint::from(180u64))),
        );
        let p2 = Secp256k1Point::new(
            Some(FieldElement::new(BigUint::from(57u64))),
            Some(FieldElement::new(BigUint::from(180u64))),
        );
        let expected = Secp256k1Point::new(
            Some(FieldElement::new(BigUint::from(156u64))),
            Some(FieldElement::new(BigUint::from(38u64))),
        );

        assert_eq!(p1 + p2, expected);
    }

    #[test]
    fn scalar_multiple() {
        let point = Secp256k1Point::new(
            Some(FieldElement::new(BigUint::from(47u64))),
            Some(FieldElement::new(BigUint::from(71u64))),
        );
        let expected = Secp256k1Point::new(
            Some(FieldElement::new(BigUint::from(139u64))),
            Some(FieldElement::new(BigUint::from(137u64))),
        );
        let result = point.multiply_by(6);

        assert_eq!(expected, result);

        let point = Secp256k1Point::new(
            Some(FieldElement::new(BigUint::from(15u64))),
            Some(FieldElement::new(BigUint::from(86u64))),
        );
        let expected = Secp256k1Point::infinity_point();

        assert_eq!(point.multiply_by(7), expected)
    }

    // secp256k1 and Bitcoin use a predetermined Generation point, so deprecating this test.
    // #[test]
    // fn exercise_five() {
    //     // For the curve y2 = x3 + 7 over F223,
    //     // find the order of the group generated by (15,86)
    //     let generation_point = Secp256k1Point::new(
    //         Some(FieldElement::new(15, 223)),
    //         Some(FieldElement::new(86, 223)),
    //     );
    //     let mut order: u32 = 0;
    //     let mut sum = generation_point.clone();
    //     loop {
    //         println!("{:?}", sum);
    //         order += 1;
    //         sum = generation_point + sum;
    //         if sum.x.is_none() && sum.y.is_none() {
    //             order += 1;
    //             break;
    //         }
    //     }
    //     println!("Order of set: {}", order)
    // }
}
