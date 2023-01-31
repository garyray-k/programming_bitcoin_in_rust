use std::fmt;
use std::ops::{Add, Div, Mul, Sub};

use num::{BigInt, BigUint, FromPrimitive, One, Zero};

#[derive(Debug, Clone)]
pub struct FieldElement {
    num: BigUint,
    prime: BigUint,
}

impl FieldElement {
    pub fn new(num: BigUint, prime: BigUint) -> Self {
        if num >= prime {
            panic!("Num {} not in field range", num);
        }
        Self { num, prime }
    }

    pub fn zero(prime: BigUint) -> Self {
        Self {
            num: BigUint::zero(),
            prime,
        }
    }

    pub fn get_prime(self) -> BigUint {
        self.prime
    }

    pub fn get_number(self) -> BigUint {
        self.num
    }

    pub fn to_the_power_of(self, exponent: BigUint) -> Self {
        let exp = exponent % (self.prime - BigUint::from_u64(1u64).unwrap());
        let new_num = Self::mod_pow(self.num, exp.into(), self.prime);
        FieldElement {
            num: new_num,
            prime: self.prime,
        }
    }

    // credit to https://rob.co.bb/posts/2019-02-10-modular-exponentiation-in-rust/
    fn mod_pow(mut base: BigUint, mut exp: BigUint, modulus: BigUint) -> BigUint {
        if modulus == BigUint::one() {
            return BigUint::zero();
        }
        let mut result = BigUint::one();
        base = base % modulus;
        while exp > BigUint::zero() {
            if exp % BigUint::from_u64(2u64).unwrap() == BigUint::one() {
                result = result * base % modulus;
            }
            exp = exp >> 1;
            base = base * base % modulus
        }
        result
    }
}

impl PartialEq for FieldElement {
    fn eq(&self, other: &Self) -> bool {
        return self.num == other.num && self.prime == other.prime;
    }
}

impl Eq for FieldElement {}

impl fmt::Display for FieldElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "FieldElement_{}({}))", self.prime, self.num)
    }
}

impl Add for FieldElement {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        if self.prime != other.prime {
            panic!("Cannot add two numbers in different Field.");
        }
        let new_num = (self.num + other.num) % self.prime;
        FieldElement {
            num: new_num,
            prime: self.prime,
        }
    }
}

impl Sub for FieldElement {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        if self.prime != other.prime {
            panic!("Cannot add two numbers in different Fields.");
        }

        let difference: BigInt = BigInt::from(self.num) - BigInt::from(other.num);
        let big_prime = BigInt::from(self.prime);
        let remainder = difference % big_prime;
        if remainder < BigInt::zero() {
            let new_number = remainder + big_prime;
            FieldElement {
                num: new_number.try_into().unwrap(),
                prime: self.prime,
            }
        } else {
            FieldElement {
                num: remainder.try_into().unwrap(),
                prime: self.prime,
            }
        }
    }
}

impl Mul for FieldElement {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        if self.prime != other.prime {
            panic!("Cannot multiply two numbers in different Order.");
        }
        let new_num = (self.num * other.num) % self.prime;
        FieldElement {
            num: new_num,
            prime: self.prime,
        }
    }
}

impl Div for FieldElement {
    type Output = Self;

    fn div(self, divisor: Self) -> Self::Output {
        if self.prime != divisor.prime {
            panic!("Cannot divide two numbers in different Order.");
        }
        let new_num = self.num
            * divisor.num.modpow(
                &(self.prime - BigUint::from_u64(2u64).unwrap()),
                &self.prime,
            )
            % self.prime;
        FieldElement::new(new_num, self.prime)
    }
}

// num = self.num * pow(other.num,(self.prime-2),self.prime)%self.prime

#[cfg(test)]
mod field_element_tests {

    use super::*;

    #[test]
    fn eq_works() {
        let a = FieldElement::new(7u64.into(), 13u64.into());
        let b = FieldElement::new(6u64.into(), 13u64.into());
        assert!(a != b);
        let a = FieldElement::new(7u64.into(), 13u64.into());
        let c = FieldElement::new(7u64.into(), 13u64.into());
        assert!(a == c);
    }

    #[test]
    fn add_works() {
        let a = FieldElement::new(7u64.into(), 13u64.into());
        let b = FieldElement::new(12u64.into(), 13u64.into());
        let c = FieldElement::new(6u64.into(), 13u64.into());
        assert!(a + b == c);
    }

    #[test]
    fn sub_works() {
        let a = FieldElement::new(2u64.into(), 19u64.into());
        let b = FieldElement::new(11u64.into(), 19u64.into());
        let c = FieldElement::new(9u64.into(), 19u64.into());
        assert!(b - c == a)
    }

    #[test]
    fn mul_works() {
        let a = FieldElement::new(3u64.into(), 13u64.into());
        let b = FieldElement::new(12u64.into(), 13u64.into());
        let c = FieldElement::new(10u64.into(), 13u64.into());
        assert!(a * b == c);
        let a = FieldElement::new(24u64.into(), 31u64.into());
        let b = FieldElement::new(19u64.into(), 31u64.into());
        let c = FieldElement::new(22u64.into(), 31u64.into());
        assert!(a * b == c);
        assert!(3 % 13 == 3);
        assert!(8231 % 73829138 == 8231);
    }

    #[test]
    fn pow_works() {
        let a = FieldElement::new(3u64.into(), 13u64.into());
        let b = FieldElement::new(1u64.into(), 13u64.into());
        assert!(a.to_the_power_of(3u64.into()) == b);
        let a = FieldElement::new(17u64.into(), 31u64.into());
        assert_eq!(
            a.to_the_power_of(3u64.into()),
            FieldElement::new(15u64.into(), 31u64.into())
        );

        let a = FieldElement::new(5u64.into(), 31u64.into());
        let b = FieldElement::new(18u64.into(), 31u64.into());
        assert!(
            (a.to_the_power_of(5u64.into()) * b) == FieldElement::new(16u64.into(), 31u64.into())
        );
    }

    #[test]
    fn div_works() {
        let a = FieldElement::new(2u64.into(), 19u64.into());
        let b = FieldElement::new(7u64.into(), 19u64.into());
        let c = FieldElement::new(3u64.into(), 19u64.into());
        assert!(c == a / b)
    }
}
