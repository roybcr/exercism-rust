#![allow(unused)]

pub fn divmod(dividend: i16, divisor: i16) -> (i16, i16) {
      (dividend.div_euclid(divisor), dividend.rem_euclid(divisor))
}

pub fn evens<T>(iter: impl Iterator<Item = T>) -> impl Iterator<Item = T> {
      iter.enumerate()
            .filter(|i| i.0.rem_euclid(2).eq(&0))
            .map(|i| i.1)
            .into_iter()
      // iter.step_by(2) --> A better solution
}

pub struct Position(pub i16, pub i16);
impl Position {
      pub fn manhattan(&self) -> i16 { (self.0).abs() + (self.1).abs() }
}
