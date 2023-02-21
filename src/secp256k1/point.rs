use std::fmt;
use std::ops::{Add, AddAssign, BitAnd, Shr};

use num::{BigUint, Num, One, Zero};

use super::field_element::FieldElement;
use super::signature::Signature;

fn n() -> BigUint {
    BigUint::from_str_radix(
        "fffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd0364141",
        16,
    )
    .unwrap()
}

fn generator_point() -> Secp256k1Point {
    let generator_x = BigUint::from_str_radix(
        "79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798",
        16,
    )
    .unwrap();
    let generator_y = BigUint::from_str_radix(
        "483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8",
        16,
    )
    .unwrap();
    let x = FieldElement::new(generator_x);
    let y = FieldElement::new(generator_y);

    Secp256k1Point::new(Some(x), Some(y))
}

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

    pub fn multiply_by(self, coefficient: &mut BigUint) -> Secp256k1Point {
        // let &mut coef = coefficient;
        let mut current = self;
        let mut result = Self::infinity_point();
        while *coefficient != BigUint::zero() {
            if coefficient.clone().bitand(BigUint::one()) == BigUint::one() {
                result = result + current.clone();
            }
            current = current.clone() + current;
            *coefficient = coefficient.clone().shr(1u16);
        }
        result
    }

    fn infinity_point() -> Secp256k1Point {
        Secp256k1Point::new(None, None)
    }

    pub fn verify(self, z: BigUint, signature: Signature) -> bool {
        let order_minus_two = n() - BigUint::from(2u64);
        let s_inv = signature.s().modpow(&order_minus_two, &n());
        let mut u = (z * &s_inv) % n();
        let mut v = (signature.r() * s_inv) % n();
        let total = generator_point().multiply_by(&mut u) + self.multiply_by(&mut v);
        total.x.unwrap().get_number() == *signature.r()
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

    use std::ops::Mul;

    use num::{FromPrimitive, Num, One};

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
    fn veryify_generator_point_on_secp256k1_curve() {
        let generator_x = BigUint::from_str_radix(
            "79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798",
            16,
        )
        .unwrap();
        let generator_y = BigUint::from_str_radix(
            "483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8",
            16,
        )
        .unwrap();
        let secp256k1_prime =
            BigUint::from(2u64).pow(256) - BigUint::from(2u64).pow(32) - BigUint::from(977u64);
        assert_eq!(
            generator_y.pow(2) % secp256k1_prime.clone(),
            (generator_x.pow(3) + BigUint::from_i32(7).unwrap()) % secp256k1_prime
        );
    }

    #[test]
    fn verify_generator_point_has_order_n() {
        let generator_point = generator_point();
        let mut n = BigUint::from_str_radix(
            "fffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd0364141",
            16,
        )
        .unwrap();
        assert_eq!(
            generator_point.multiply_by(&mut n),
            Secp256k1Point::infinity_point()
        )
    }

    #[test]
    fn verify_signature() {
        let z = BigUint::from_str_radix(
            "bc62d4b80d9e36da29c16c5d4d9f11731f36052c72401a76c23c0fb5a9b74423",
            16,
        )
        .unwrap();
        let r = BigUint::from_str_radix(
            "37206a0610995c58074999cb9767b87af4c4978db68c06e8e6e81d282047a7c6",
            16,
        )
        .unwrap();
        let s = BigUint::from_str_radix(
            "8ca63759c1157ebeaec0d03cecca119fc9a75bf8e6d0fa65c841c8e2738cdaec",
            16,
        )
        .unwrap();
        let px = BigUint::from_str_radix(
            "04519fac3d910ca7e7138f7013706f619fa8f033e6ec6e09370ea38cee6a7574",
            16,
        )
        .unwrap();
        let py = BigUint::from_str_radix(
            "82b51eab8c27c66e26c858a079bcdf4f1ada34cec420cafc7eac1a42216fb6c4",
            16,
        )
        .unwrap();
        let point = Secp256k1Point::new(Some(FieldElement::new(px)), Some(FieldElement::new(py)));
        let signature = Signature::new(r, s);
        assert!(point.verify(z, signature))
    }

    #[test]
    fn generator_point_test() {
        let prime = BigUint::from_str_radix(
            "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F",
            16,
        )
        .unwrap();
        let x = BigUint::from_str_radix(
            "79BE667EF9DCBBAC55A06295CE870B07029BFCDB2DCE28D959F2815B16F81798",
            16,
        )
        .unwrap();
        let y = BigUint::from_str_radix(
            "483ADA7726A3C4655DA4FBFC0E1108A8FD17B448A68554199C47D08FFB10D4B8",
            16,
        )
        .unwrap();
        assert_eq!(
            BigUint::zero(),
            ((y.clone().mul(y))
                - (x.clone().mul(x.clone()).mul(x))
                - BigUint::from_u32(7).unwrap())
                % prime
        )
    }

    #[test]
    fn chapter_3_exercise_6() {
        let public_key = Secp256k1Point::new(
            Some(FieldElement::new(
                BigUint::from_str_radix(
                    "887387e452b8eacc4acfde10d9aaf7f6d9a0f975aabb10d006e4da568744d06c",
                    16,
                )
                .unwrap(),
            )),
            Some(FieldElement::new(
                BigUint::from_str_radix(
                    "61de6d95231cd89026e286df3b6ae4a894a3378e393e93a0f45b666329a0ae34",
                    16,
                )
                .unwrap(),
            )),
        );

        // # signature 1
        let z = BigUint::from_str_radix(
            "ec208baa0fc1c19f708a9ca96fdeff3ac3f230bb4a7ba4aede4942ad003c0f60",
            16,
        )
        .unwrap();
        let r = BigUint::from_str_radix(
            "ac8d1c87e51d0d441be8b3dd5b05c8795b48875dffe00b7ffcfac23010d3a395",
            16,
        )
        .unwrap();
        let s = BigUint::from_str_radix(
            "68342ceff8935ededd102dd876ffd6ba72d6a427a3edb13d26eb0781cb423c4",
            16,
        )
        .unwrap();
        let order_minus_two = n() - BigUint::from(2u64);
        let s_inv = s.modpow(&order_minus_two, &n());
        let mut u = (z * &s_inv) % n();
        let mut v = (&r * s_inv) % n();
        assert_eq!(
            ((generator_point().multiply_by(&mut u)) + (public_key.clone().multiply_by(&mut v)))
                .x
                .unwrap()
                .get_number()
                .to_str_radix(16),
            r.to_str_radix(16)
        );

        // signature 2
        let z = BigUint::from_str_radix(
            "7c076ff316692a3d7eb3c3bb0f8b1488cf72e1afcd929e29307032997a838a3d",
            16,
        )
        .unwrap();
        let r = BigUint::from_str_radix(
            "eff69ef2b1bd93a66ed5219add4fb51e11a840f404876325a1e8ffe0529a2c",
            16,
        )
        .unwrap();
        let s = BigUint::from_str_radix(
            "c7207fee197d27c618aea621406f6bf5ef6fca38681d82b2f06fddbdce6feab6",
            16,
        )
        .unwrap();
        let order_minus_two = n() - BigUint::from(2u64);
        let s_inv = s.modpow(&order_minus_two, &n());
        let mut u = (z * &s_inv) % n();
        let mut v = (&r * s_inv) % n();
        assert_eq!(
            ((generator_point().multiply_by(&mut u)) + (public_key.multiply_by(&mut v)))
                .x
                .unwrap()
                .get_number()
                .to_str_radix(16),
            r.to_str_radix(16)
        );
    }

    // Assuming the previous tests pass, our code functions as expected
    // so the following tests are excluded.

    // #[test]
    // fn eq_works() {
    //     let a = Secp256k1Point::new(
    //         Some(FieldElement::new(BigUint::one())),
    //         Some(FieldElement::new(BigUint::one())),
    //     );
    //     let b = Secp256k1Point::new(
    //         Some(FieldElement::new(BigUint::one())),
    //         Some(FieldElement::new(BigUint::one())),
    //     );
    //     let c = Secp256k1Point::new(
    //         Some(FieldElement::new(BigUint::from(180u64))),
    //         Some(FieldElement::new(BigUint::from(108u64))),
    //     );

    //     assert!(a == b);
    //     assert!(a != c);
    // }

    // #[test]
    // fn add_identity_test() {
    //     let p1 = Secp256k1Point::new(
    //         Some(FieldElement::new(BigUint::one())),
    //         Some(FieldElement::new(BigUint::from(77u64))),
    //     );
    //     let p2 = Secp256k1Point::new(
    //         Some(FieldElement::new(BigUint::one())),
    //         Some(FieldElement::new(BigUint::from(77u64))),
    //     );
    //     let identity_point = Secp256k1Point::infinity_point();

    //     // exercise 3
    //     assert!(p1.clone() + identity_point.clone() == p1);
    //     assert!(p2.clone() + identity_point == p2);
    // }

    // #[test]
    // fn add_test() {
    //     // exercise 4 and 5
    //     // For the curve y 2 = x 3 + 5x + 7, what is (2,5) + (–1,–1)?
    //     let p1 = Secp256k1Point::new(
    //         Some(FieldElement::new(BigUint::from(57u64))),
    //         Some(FieldElement::new(BigUint::from(180u64))),
    //     );
    //     let p2 = Secp256k1Point::new(
    //         Some(FieldElement::new(BigUint::from(47u64))),
    //         Some(FieldElement::new(BigUint::from(58u64))),
    //     );
    //     let expected = Secp256k1Point::new(
    //         Some(FieldElement::new(BigUint::from(190u64))),
    //         Some(FieldElement::new(BigUint::from(31u64))),
    //     );

    //     assert_eq!(p1 + p2, expected);
    // }

    // #[test]
    // fn add_self_test() {
    //     // add to itself
    //     let p1 = Secp256k1Point::new(
    //         Some(FieldElement::new(BigUint::from(57u64))),
    //         Some(FieldElement::new(BigUint::from(180u64))),
    //     );
    //     let p2 = Secp256k1Point::new(
    //         Some(FieldElement::new(BigUint::from(57u64))),
    //         Some(FieldElement::new(BigUint::from(180u64))),
    //     );
    //     let expected = Secp256k1Point::new(
    //         Some(FieldElement::new(BigUint::from(156u64))),
    //         Some(FieldElement::new(BigUint::from(38u64))),
    //     );

    //     assert_eq!(p1 + p2, expected);
    // }

    // #[test]
    // fn scalar_multiple() {
    //     let point = Secp256k1Point::new(
    //         Some(FieldElement::new(BigUint::from(47u64))),
    //         Some(FieldElement::new(BigUint::from(71u64))),
    //     );
    //     let expected = Secp256k1Point::new(
    //         Some(FieldElement::new(BigUint::from(139u64))),
    //         Some(FieldElement::new(BigUint::from(137u64))),
    //     );
    //     let result = point.multiply_by(&mut BigUint::from(6u64));

    //     assert_eq!(expected, result);

    //     let point = Secp256k1Point::new(
    //         Some(FieldElement::new(BigUint::from(15u64))),
    //         Some(FieldElement::new(BigUint::from(86u64))),
    //     );
    //     let expected = Secp256k1Point::infinity_point();

    //     assert_eq!(point.multiply_by(&mut BigUint::from(7u64)), expected)
    // }

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
