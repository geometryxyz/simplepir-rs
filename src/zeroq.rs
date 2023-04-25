pub trait ZeroQ {
    fn zero(q: u64) -> Self;
    fn is_zero(&self) -> bool;
}
