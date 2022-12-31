// Given the nature of numbers in the set 2^0 ... 2^n,
// for any n, we see that the sum of 2^1 + ... + 2^(n - 1) equals ((2^n) - 2).
// With that in mind, we can say that for any score x, if x > 2^n, the allergy
// n, must be included in x.

// STEPS TO SOLVE:
// ----------------------------------------------------------------------------------
// As long as score > 0, subtract from score 2^n where n = log2(score) rounded
// down. if score > 2^7 (the allergy with the largest score), continue doing the
// step above. else, include the allergy (represented by 2^n) in the list of
// allergies.

#[derive(Debug)]
pub struct Allergies(u32);

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Allergen {
      Eggs,
      Peanuts,
      Shellfish,
      Strawberries,
      Tomatoes,
      Chocolate,
      Pollen,
      Cats,
      Other,
}

impl From<u32> for Allergen {
      fn from(n: u32) -> Self {
            use Allergen::*;
            match n {
                  0u32 => Eggs,
                  1u32 => Peanuts,
                  2u32 => Shellfish,
                  3u32 => Strawberries,
                  4u32 => Tomatoes,
                  5u32 => Chocolate,
                  6u32 => Pollen,
                  7u32 => Cats,
                  _ => Other,
            }
      }
}

pub const BASE: u32 = 2;

#[allow(dead_code)]
impl Allergies {
      pub fn new(score: u32) -> Self { Allergies(score) }

      pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
            self.0.ge(&BASE.pow(allergen.clone() as u32))
      }

      pub fn allergies(&self) -> Vec<Allergen> {
            let mut score_left = self.0;
            let mut allergies: Vec<Allergen> = Vec::new();

            while score_left.gt(&0u32) {
                  let allergy: u32 = f64::from(score_left).log2().floor() as u32;
                  score_left -= BASE.pow(allergy);

                  if Allergen::from(allergy).ne(&Allergen::Other) {
                        allergies.push(allergy.into());
                  }
            }

            allergies
      }
}
