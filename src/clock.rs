use std::ops::{Add, Div, Mul};

#[derive(Debug, PartialEq)]
pub struct Clock(i32, i32);

const MINS: i32 = 86400;

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let tot_min = hours.mul(60).add(minutes);
        let tot_min = if tot_min.le(&0) { MINS + tot_min } else { tot_min };
        let time = f64::from(tot_min).div(60.0);
        Clock((time.trunc() % 24.0) as i32,
              (time.fract() * 60.0).round() as i32)
    }

    #[allow(dead_code)]
    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(self.0, self.1 + minutes)
    }
}

impl std::fmt::Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut h_str = String::from("0");
        let mut m_str = String::from("0");

        h_str.push_str(&(self.0).to_string()[..]);
        m_str.push_str(&(self.1).to_string()[..]);

        let h_len = h_str.len();
        let m_len = m_str.len();

        let h = h_str.to_string()[h_len - 2..h_len].to_string();
        let m = m_str.to_string()[m_len - 2..m_len].to_string();

        write!(f, "{h}:{m}")
    }
}

#[cfg(test)]
mod tests {
    use super::Clock;

    #[test]
    fn new() {
        let c2_40: Clock = Clock::new(0, 160);
        let c4_43: Clock = Clock::new(0, 1723);

        assert_eq!((c2_40.0, c2_40.1), (2i32, 40i32));
        assert_eq!((c4_43.0, c4_43.1), (4i32, 43i32));
        assert_eq!(Clock::new(-12, -268), Clock::new(7, 32));
    }
}
