use std::fmt::{Display, Formatter};
use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};
use num::traits::identities::Zero;
use rand_distr::{Normal, Distribution};
use rand::{
    rngs::StdRng,
    SeedableRng,
};

#[derive(Debug, PartialEq)]
pub struct Element {
    pub(crate) q: u64,
    pub(crate) uint: u64,
}

impl Element {
    pub fn new(q: u64) -> Self {
        Self {
            q,
            uint: u64::zero(),
        }
    }

    pub fn from(q: u64, uint: u64) -> Self {
        assert!(q < u64::MAX);
        assert!(uint < u64::MAX);

        Self { q, uint }
    }

    /// Generate a random Element following a normal (Gaussian) distribution.
    ///
    /// # Parameters 
    ///
    /// - `q`: The element modulus
    /// - `std_dev`: The standard deviation of the distribution.
    pub fn gen_normal_rand(q: u64, std_dev: f64) -> Self {
        assert!(std_dev < q as f64);
        let mean = (&q / 2u64) as f64;
        let normal = Normal::new(mean, std_dev).unwrap();

        let mut rng = StdRng::from_entropy();
        let v = normal.sample(&mut rng) as u64;
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

impl Zero for Element {
    fn zero() -> Self {
        return Element {
            q: 0u64,
            uint: 0u64,
        }
    }

    fn is_zero(&self) -> bool {
        self.uint == 0u64
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
    use super::Element;

    fn gen_q() -> u64 {
        return 101u64;
    }

    #[test]
    fn test_new() {
        let q = gen_q();
        let f = Element::new(q);
        assert_eq!(f.uint, 0u64);
    }

    #[test]
    fn test_add() {
        let f = Element::from(gen_q(), 101u64);
        let g = Element::from(gen_q(), 1u64);
        let r = f + g;
        assert_eq!(r.uint, 1u64);

        let f = Element::from(gen_q(), 1u64);
        let g = Element::from(gen_q(), 1u64);
        let r = f + g;
        assert_eq!(r.uint, 2u64);
    }

    #[test]
    fn test_add_assign() {
        let mut f = Element::from(gen_q(), 101u64);
        let g = Element::from(gen_q(), 1u64);
        f += g;
        assert_eq!(f.uint, 1u64);
    }

    #[test]
    fn test_sub() {
        let f = Element::from(gen_q(), 0u64);
        let g = Element::from(gen_q(), 1u64);
        let r = f - g;
        assert_eq!(r.uint, 100u64);
    }

    #[test]
    fn test_sub_assign() {
        let mut f = Element::from(gen_q(), 101u64);
        let g = Element::from(gen_q(), 1u64);
        f -= g;
        assert_eq!(f.uint, 100u64);
    }

    #[test]
    fn test_mul() {
        let f = Element::from(gen_q(), 101u64);
        let g = Element::from(gen_q(), 2u64);
        let r = f * g;
        assert_eq!(r.uint, 0u64);

        let f = Element::from(gen_q(), 3u64);
        let g = Element::from(gen_q(), 5u64);
        let r = f * g;
        assert_eq!(r.uint, 15u64);
    }

    #[test]
    fn test_mul_assign() {
        let mut f = Element::from(gen_q(), 100u64);
        let g = Element::from(gen_q(), 2u64);
        f *= g;
        assert_eq!(f.uint, 99u64);
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
