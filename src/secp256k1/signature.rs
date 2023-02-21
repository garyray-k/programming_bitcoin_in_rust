use num::BigUint;

#[derive(Debug)]
pub struct Signature {
    // Random point x value
    r: BigUint,
    s: BigUint,
}

impl Signature {
    pub fn new(r: BigUint, s: BigUint) -> Self {
        Signature { r, s }
    }

    pub fn r(&self) -> &BigUint {
        &self.r
    }

    pub fn s(&self) -> &BigUint {
        &self.s
    }
}
