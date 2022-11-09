use std::{fmt};
use std::ops::{Add, Sub};

#[derive(Debug)]
pub struct FieldElement {
    num: u64,
    prime: u64
}

impl FieldElement {
    pub fn new(num: u64, prime: u64) -> Self {
        if num >= prime {
            panic!("Num {} not in field range 0 to {}", num, prime - 1);
        }
        Self {
            num,
            prime
        }
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
        FieldElement { num: new_num, prime: self.prime }
    }
}

impl Sub for FieldElement {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        if self.prime != other.prime {
            panic!("Cannot add two numbers in different Field.");
        }
        let new_num = (self.num - other.num) % self.prime;
        FieldElement { num: new_num, prime: self.prime }
    }
}

// impl Mul for FieldElement {
//     type Output = Self;

//     fn mul(self, other: Self) -> Self {
        
//     }
// }


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
        assert!(a+b == c);
    }

    #[test]
    fn sub_works() {
        let a = FieldElement::new(2, 19);
        let b = FieldElement::new(11, 19);
        let c = FieldElement::new(9, 19);
        assert!(b-c == a)
    }

    // #[test]
    // fn mul_works() {
    //     let a = FieldElement::new(3, 13);
    //     let b = FieldElement::new(12, 13);
    //     let c = FieldElement::new(10, 13);
    //     assert!(a*b == c)
    // }
}