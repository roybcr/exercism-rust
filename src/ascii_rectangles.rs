use std::collections::{HashMap, HashSet};

#[derive(Debug, Eq, PartialEq)]
struct Line {
    xl: u32,
    xr: u32,
}

impl Line {
    fn new(xl: u32, xr: u32) -> Self {
        match xr.gt(&xl) {
            true => Line { xl, xr },
            false => Line { xl: xr, xr: xl },
        }
    }
}

pub fn count(lines: &[&str]) -> u32 {
    let rows = lines.iter().enumerate();
    let mut rhash = HashMap::<usize, HashSet<usize>>::new();

    for (i, row) in rows {
        let mut chash = HashSet::<usize>::new();
        for (j, cole) in row.chars().enumerate() {
            if cole == '+' {
                chash.insert(j);
            }
        }
        if !chash.is_empty() {
            rhash.insert(i, chash);
        }
    }

    for key in rhash.keys() {
        if let Some(values) = rhash.get(key) {
            for val in values {
                print!("{val: >10}");
            }
        }
        println!();
    }
    2u32
}

// + --> + -- >
// |     |
// + --> +
//
