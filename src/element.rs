use num::traits::identities::Zero;
pub use num_bigint::BigUint;
use std::fmt::{Display, Formatter};
use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};
use rand_distr::{Normal, Distribution};
use rand::{
    rngs::StdRng,
    SeedableRng,
};

#[derive(Debug, PartialEq)]
pub struct Element {
    pub(crate) q: BigUint,
    pub(crate) uint: BigUint,
}

impl Element {
    pub fn new(q: BigUint) -> Self {
        Self {
            q,
            uint: BigUint::zero(),
        }
    }

    pub fn from(q: BigUint, uint: BigUint) -> Self {
        assert!(q < BigUint::from(u64::MAX));
        assert!(uint < BigUint::from(u64::MAX));

        Self { q, uint }
    }

    /// Generate a random Element following a normal (Gaussian) distribution.
    ///
    /// # Parameters 
    ///
    /// - `q`: The element modulus
    /// - `std_dev`: The standard deviation of the distribution.
    pub fn gen_normal_rand(q: BigUint, std_dev: f64) -> Self {
        let mean_buint = &q / BigUint::from(2u64);
        let mean = mean_buint.to_u64_digits()[0] as f64;
        let normal = Normal::new(mean, std_dev).unwrap();

        let mut rng = StdRng::from_entropy();
        let v = BigUint::from(normal.sample(&mut rng) as u64);
        Self::from(q, v)
    }
}

impl Clone for Element {
    fn clone(&self) -> Self {
        Self {
            uint: self.uint.clone(),
            q: self.q.clone(),
        }
    }

    fn clone_from(&mut self, source: &Self) {
        let c = source.clone();
        *self = c;
    }
}

impl Mul for Element {
    type Output = Element;
    fn mul(self, rhs: Element) -> Self::Output {
        assert_eq!(self.q, rhs.q);
        Self {
            q: self.q.clone(),
            uint: (self.uint * rhs.uint) % self.q,
        }
    }
}

impl MulAssign for Element {
    fn mul_assign(&mut self, rhs: Self) {
        assert_eq!(self.q, rhs.q);
        *self = Self {
            q: self.q.clone(),
            uint: (self.uint.clone() * rhs.uint) % self.q.clone(),
        }
    }
}

impl Add for Element {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        assert_eq!(self.q, rhs.q);
        Self {
            q: self.q.clone(),
            uint: (self.uint + rhs.uint) % self.q,
        }
    }
}

impl AddAssign for Element {
    fn add_assign(&mut self, rhs: Self) {
        assert_eq!(self.q, rhs.q);
        *self = Self {
            q: self.q.clone(),
            uint: (self.uint.clone() + rhs.uint) % self.q.clone(),
        }
    }
}

impl Sub for Element {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        assert_eq!(self.q, other.q);
        if self.uint < other.uint {
            let d = other.uint - self.uint;
            return Self {
                q: self.q.clone(),
                uint: self.q - d,
            };
        }

        Self {
            q: self.q.clone(),
            uint: self.uint - other.uint,
        }
    }
}

impl SubAssign for Element {
    fn sub_assign(&mut self, other: Self) {
        assert_eq!(self.q, other.q);
        if self.uint < other.uint {
            let d = other.uint - &self.uint;
            *self = Self {
                q: self.q.clone(),
                uint: &self.q - d,
            }
        } else {
            *self = Self {
                q: self.q.clone(),
                uint: self.uint.clone() - other.uint,
            }
        }
    }
}

impl Display for Element {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.uint)
    }
}

#[cfg(test)]
mod tests {
    use super::{BigUint, Element};

    fn gen_q() -> BigUint {
        return BigUint::from(101u64);
    }

    #[test]
    fn test_new() {
        let q = gen_q();
        let f = Element::new(q);
        assert_eq!(f.uint, BigUint::from(0u64));
    }

    #[test]
    fn test_add() {
        let f = Element::from(gen_q(), BigUint::from(101u64));
        let g = Element::from(gen_q(), BigUint::from(1u64));
        let r = f + g;
        assert_eq!(r.uint, BigUint::from(1u64));

        let f = Element::from(gen_q(), BigUint::from(1u64));
        let g = Element::from(gen_q(), BigUint::from(1u64));
        let r = f + g;
        assert_eq!(r.uint, BigUint::from(2u64));
    }

    #[test]
    fn test_add_assign() {
        let mut f = Element::from(gen_q(), BigUint::from(101u64));
        let g = Element::from(gen_q(), BigUint::from(1u64));
        f += g;
        assert_eq!(f.uint, BigUint::from(1u64));
    }

    #[test]
    fn test_sub() {
        let f = Element::from(gen_q(), BigUint::from(0u64));
        let g = Element::from(gen_q(), BigUint::from(1u64));
        let r = f - g;
        assert_eq!(r.uint, BigUint::from(100u64));
    }

    #[test]
    fn test_sub_assign() {
        let mut f = Element::from(gen_q(), BigUint::from(101u64));
        let g = Element::from(gen_q(), BigUint::from(1u64));
        f -= g;
        assert_eq!(f.uint, BigUint::from(100u64));
    }

    #[test]
    fn test_mul() {
        let f = Element::from(gen_q(), BigUint::from(101u64));
        let g = Element::from(gen_q(), BigUint::from(2u64));
        let r = f * g;
        assert_eq!(r.uint, BigUint::from(0u64));

        let f = Element::from(gen_q(), BigUint::from(3u64));
        let g = Element::from(gen_q(), BigUint::from(5u64));
        let r = f * g;
        assert_eq!(r.uint, BigUint::from(15u64));
    }

    #[test]
    fn test_mul_assign() {
        let mut f = Element::from(gen_q(), BigUint::from(100u64));
        let g = Element::from(gen_q(), BigUint::from(2u64));
        f *= g;
        assert_eq!(f.uint, BigUint::from(99u64));
    }

    /*
    #[test]
    fn test_gen_normal_rand() {
        let q = gen_q();
        for i in 0..100 {
            let e = Element::gen_normal_rand(q.clone(), 6.4 as f64);
            println!("{}", e);
        }
    }
    */
}
