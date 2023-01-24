use std::fmt;
use std::ops::{Add, Div, Mul, Sub};

use num::{BigUint, Zero};

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
        let exp = (exponent % (self.prime - 1)) as u64;
        let new_num = Self::mod_pow(self.num, exp, self.prime);
        FieldElement {
            num: new_num,
            prime: self.prime,
        }
    }

    // credit to https://rob.co.bb/posts/2019-02-10-modular-exponentiation-in-rust/
    fn mod_pow(mut base: BigUint, mut exp: BigUint, modulus: BigUint) -> BigUint {
        if modulus == 1 {
            return 0;
        }
        let mut result = 1;
        base = base % modulus;
        while exp > 0 {
            if exp % 2 == 1 {
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
            panic!("Cannot add two numbers in different Field.");
        }

        let difference: i64 = self.num as i64 - other.num as i64;
        // Use .rem_euclid() because of how Rust handles negative numbers and modulo
        let new_num = difference.rem_euclid(self.prime as i64);

        FieldElement {
            num: new_num as u64,
            prime: self.prime,
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
        let new_num =
            self.num * mod_exp::mod_exp(divisor.num, self.prime - 2, self.prime) % self.prime;
        FieldElement::new(new_num, self.prime)
    }
}

// num = self.num * pow(other.num,(self.prime-2),self.prime)%self.prime

#[cfg(test)]
mod field_element_tests {

    use super::*;

    #[test]
    fn eq_works() {
        let a = FieldElement::new(7, 13);
        let b = FieldElement::new(6, 13);
        assert!(a != b);
        let a = FieldElement::new(7, 13);
        let c = FieldElement::new(7, 13);
        assert!(a == c);
    }

    #[test]
    fn add_works() {
        let a = FieldElement::new(7, 13);
        let b = FieldElement::new(12, 13);
        let c = FieldElement::new(6, 13);
        assert!(a + b == c);
    }

    #[test]
    fn sub_works() {
        let a = FieldElement::new(2, 19);
        let b = FieldElement::new(11, 19);
        let c = FieldElement::new(9, 19);
        assert!(b - c == a)
    }

    #[test]
    fn mul_works() {
        let a = FieldElement::new(3, 13);
        let b = FieldElement::new(12, 13);
        let c = FieldElement::new(10, 13);
        assert!(a * b == c);
        let a = FieldElement::new(24, 31);
        let b = FieldElement::new(19, 31);
        let c = FieldElement::new(22, 31);
        assert!(a * b == c);
        assert!(3 % 13 == 3);
        assert!(8231 % 73829138 == 8231);
    }

    #[test]
    fn pow_works() {
        let a = FieldElement::new(3, 13);
        let b = FieldElement::new(1, 13);
        assert!(a.to_the_power_of(3) == b);
        let a = FieldElement::new(17, 31);
        assert_eq!(a.to_the_power_of(3), FieldElement::new(15, 31));

        let a = FieldElement::new(5, 31);
        let b = FieldElement::new(18, 31);
        assert!((a.to_the_power_of(5) * b) == FieldElement::new(16, 31));
    }

    #[test]
    fn div_works() {
        let a = FieldElement::new(2, 19);
        let b = FieldElement::new(7, 19);
        let c = FieldElement::new(3, 19);
        assert!(c == a / b)
    }
}
