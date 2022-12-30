use std::ops::{Div, Mul};

trait Connectable {
    fn connect(&self, other: &Self) -> Vec<usize>;
    fn try_connect(&self, other: &Self) -> Option<Vec<usize>>;
    fn connect_many(&self, others: &[&Self]) -> Vec<Vec<usize>>;
    fn try_connect_many(&self, others: Vec<Box<Self>>) -> Vec<Option<Vec<usize>>>;
}

#[derive(Debug, Clone)]
struct Side {
    start: usize,
    end:   usize,
}

impl Side {
    fn new(start: usize, end: usize) -> Self { Side { start, end } }
}

impl Connectable for Side {
    fn connect(&self, other: &Self) -> Vec<usize> { vec![self.start, other.end] }

    fn connect_many(&self, others: &[&Self]) -> Vec<Vec<usize>> {
        others.into_iter()
              .fold(vec![], |mut acc, curr| {
                  acc.push(self.connect(curr));
                  acc
              })
              .into()
    }

    fn try_connect(&self, other: &Self) -> Option<Vec<usize>> {
        match self.end.eq(&other.start) {
            true => Some(self.connect(other)),
            false => None,
        }
    }

    fn try_connect_many(&self, others: Vec<Box<Self>>) -> Vec<Option<Vec<usize>>> {
        println!("Self: {:#?}, Others: {:#?}", self, others);
        let mut side: Option<&Side> = Some(self);
        while let Some (connection) = side {

        }
        others.into_iter().enumerate()
              .fold(vec![], |mut acc, (idx, curr)| {
                  let mut inner_idx = idx;
                   
                  while let Some(conn) = self.try_connect(&others.get(inner_idx)) {
                        
                  }
                  acc.push();
                  acc
              })
              .into()
    }
}

pub fn count(lines: &[&str]) -> u32 {
    let total_lines = lines.len();
    // let mut rects: Vec<u32> = Vec::with_capacity(total_lines);
    // let mut r_idx = 0_usize; // represents right char index of the current line.
    // let mut l_idx = 0_usize; // represents left char index of the current line.
    let mut c_idx = 0_usize; // represents current line index.

    // INFO:
    // k ( k-1 ) / 2
    // calc total rectangles between k lines.
    // let calc_total = |n: u32| -> u32 { (n.mul(n - 1)).div(2) };
    let len = match lines.first() {
        Some(&line) => line.len(),
        None => {
            return 0u32;
        },
    };

    // NOTE:
    // joints in consecutive lines are not connected vertically,
    // while other joints are connected with '|'.

    // NOTE:
    // joints not adjacent to each other horizontally
    // must be connected with '-'.

    let printer = | idx: usize, txt: &str, vec: Vec<Box<Side>> | println!("LINE =====> {idx}\t{txt}\n{:#?}", vec);
    // outer iterator represents current line we're operating on.
    while c_idx.lt(&total_lines) {
        if let Some(&line) = lines.get(c_idx) {
            let chars = line.chars();
            let mut sides: Vec<Box<Side>> = vec![];
            let mut start: Option<usize> = None;

            for (idx, ch) in chars.into_iter().enumerate() {
                #[rustfmt::skip]
                match ch {
                    '+' => { 
                        if let Some(start) = start { sides.push(Box::from(Side::new(start, idx))); } 
                        start = Some(idx); 
                    },
                    ' ' => { start = None; },
                    ___ => { continue;     },
                };
            }

            let all_sides: Vec<Box<Side>> =
                sides.iter()
                     .enumerate()
                     .fold(vec![], |mut acc, (idx, curr)| {
                         if idx.lt(&(sides.len())) {
                             let others = &sides[(idx + 1)..][..];
                             let connections = curr.try_connect_many(others.to_vec());
                             let connections: Vec<Box<Side>> =
                                 connections.iter()
                                            .filter(|x| x.is_some())
                                            .map(|x| {
                                                let se = x.as_ref().unwrap();
                                                let (&start, &end) =
                                                    (se.get(0).unwrap(), se.get(1).unwrap());
                                                Box::from(Side::new(start, end))}).collect();
                             acc.extend_from_slice(&connections[..]);
                             acc
                         } else { acc }});

            if !sides.is_empty() {
                printer(c_idx.clone(), "Printing Sides:", sides);
            }

            if !all_sides.is_empty() {
                printer(c_idx.clone(), "Printing All Sides:", all_sides);
            }
                            
        }

        c_idx = c_idx + 1_usize;
    }

    23u32
}
                        // make sure it's connected vertically 
                        // if let Some(&next_line) = lines.get(c_idx + 1) {
                        //     let char_below = &next_line[idx..idx + 1];
                        //     if char_below.is_empty() {
                        //         start = None;
                        //         continue;
                        //     }
                        // }
#[cfg(test)]
mod tests {

    use super::count;
    #[test]
    fn test_zero_area_1() {
        let lines = &[];
        assert_eq!(0, count(lines))
    }
    #[test]
    fn test_zero_area_2() {
        let lines = &[""];
        assert_eq!(0, count(lines))
    }
    #[test]
    fn test_empty_area() {
        let lines = &[" "];
        assert_eq!(0, count(lines))
    }
    #[test]
    fn test_one_rectangle() {
        #[rustfmt::skip]
    let lines = &[
        "+-+",
        "| |",
        "+-+",
    ];
        assert_eq!(1, count(lines))
    }
    #[test]
    fn test_two_rectangles_no_shared_parts() {
        #[rustfmt::skip]
    let lines = &[
        "  +-+",
        "  | |",
        "+-+-+",
        "| |  ",
        "+-+  ",
    ];
        assert_eq!(2, count(lines))
    }
    #[test]
    fn test_five_rectangles_three_regions() {
        #[rustfmt::skip]
    let lines = &[
        "  +-+",
        "  | |",
        "+-+-+",
        "| | |",
        "+-+-+",
    ];
        assert_eq!(5, count(lines))
    }
    #[test]
    fn rectangle_of_height_1() {
        #[rustfmt::skip]
    let lines = &[
        "+--+",
        "+--+",
    ];
        assert_eq!(1, count(lines))
    }
    #[test]
    fn rectangle_of_width_1() {
        #[rustfmt::skip]
    let lines = &[
        "++",
        "||",
        "++",
    ];
        assert_eq!(1, count(lines))
    }
    #[test]
    fn unit_square() {
        #[rustfmt::skip]
    let lines = &[
        "++",
        "++",
    ];
        assert_eq!(1, count(lines))
    }
    #[test]
    fn test_incomplete_rectangles() {
        #[rustfmt::skip]
    let lines = &[
        "  +-+",
        "    |",
        "+-+-+",
        "| | -",
        "+-+-+",
    ];
        assert_eq!(1, count(lines))
    }
    #[test]
    #[ignore]
    fn test_complicated() {
        let lines = &["+------+----+",
                      "|      |    |",
                      "+---+--+    |",
                      "|   |       |",
                      "+---+-------+"];
        assert_eq!(3, count(lines))
    }
    #[test]
    #[ignore]
    fn test_not_so_complicated() {
        let lines = &["+------+----+",
                      "|      |    |",
                      "+------+    |",
                      "|   |       |",
                      "+---+-------+"];
        assert_eq!(2, count(lines))
    }
    #[test]
    fn test_large_input_with_many_rectangles() {
        let lines = &["+---+--+----+",
                      "|   +--+----+",
                      "+---+--+    |",
                      "|   +--+----+",
                      "+---+--+--+-+",
                      "+---+--+--+-+",
                      "+------+  | |",
                      "          +-+"];
        assert_eq!(60, count(lines))
    }
    #[test]
    #[ignore]
    fn test_three_rectangles_no_shared_parts() {
        #[rustfmt::skip]
    let lines = &[
        "  +-+  ",
        "  | |  ",
        "+-+-+-+",
        "| | | |",
        "+-+ +-+",
    ];
        assert_eq!(3, count(lines))
    }
}
