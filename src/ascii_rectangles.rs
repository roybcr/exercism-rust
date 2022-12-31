use std::collections::HashSet;

trait Connectable {
      fn connect(&self, other: &Self) -> Vec<usize>;
      fn try_connect(&self, other: &Self) -> Option<Vec<usize>>;
      fn try_connect_many(&self, others: Vec<Box<Self>>) -> Vec<Option<Vec<usize>>>;
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Side(usize, usize);

#[allow(dead_code)]
impl Side {
      fn new(start: usize, end: usize) -> Self { Side(start, end) }
}

impl Connectable for Side {
      fn connect(&self, other: &Self) -> Vec<usize> { vec![self.0, other.1] }

      fn try_connect(&self, other: &Self) -> Option<Vec<usize>> { Some(self.connect(other)) }

      fn try_connect_many(&self, others: Vec<Box<Self>>) -> Vec<Option<Vec<usize>>> {
            let mut idx: usize = 0;
            let mut sides: Vec<Option<Vec<usize>>> = Vec::new();
            while idx < others.len() {
                  if let Some(side) = others.get(idx) {
                        let new_side = self.try_connect(&side);
                        sides.push(new_side);
                  }

                  idx += 1usize;
            }

            sides
      }
}

#[allow(dead_code)]
pub fn count(lines: &[&str]) -> u32 {
      let total_lines = lines.len();
      let mut c_idx = 0_usize; // represents current line index.
      let mut hvec: Vec<HashSet<Box<Side>>> = Vec::new();

      loop {
            let mut hset: HashSet<Box<Side>> = HashSet::new();
            if c_idx.ge(&total_lines) {
                  break;
            }

            if let Some(&line) = lines.get(c_idx) {
                  let chars = line.chars();
                  let mut sides: Vec<Box<Side>> = vec![];
                  let mut start: Option<usize> = None;

                  for (idx, ch) in chars.into_iter().enumerate() {
                        match ch {
                              '+' => {
                                    if let Some(start) = start {
                                          sides.push(Box::from(Side::new(start, idx)));
                                    }
                                    start = Some(idx);
                              },
                              ' ' => start = None,
                              ___ => continue,
                        };
                  }

                  let sides_len = sides.len();
                  let sides_cln = sides.clone();

                  sides_cln.into_iter().enumerate().for_each(|(idx, curr)| {
                        if idx.lt(&sides_len) {
                              let others = &sides[(idx + 1)..][..];
                              let connections = curr.try_connect_many(others.to_vec());
                              hset.insert(curr);
                              connections.iter().filter(|x| x.is_some()).for_each(|x| {
                                    let se = x.as_ref().unwrap();
                                    let (&start, &end) = (se.get(0).unwrap(), se.get(1).unwrap());
                                    let boxed_conn = Box::from(Side::new(start, end));
                                    hset.insert(boxed_conn);
                              });
                        }
                  });

                  hvec.push(hset);
            }

            c_idx = c_idx + 1_usize;
      }

      let mut total = 0u32;
      let mut line_idx = 0_usize;
      let hvec = hvec.clone();

      while line_idx.lt(&hvec.len()) {
            let set = &hvec.get(line_idx).unwrap();
            for s in set.iter() {
                  for row in hvec.iter().skip(line_idx + 1_usize) {
                        if row.contains(s) {
                              total += 1u32;
                        }
                  }
            }

            line_idx += 1_usize;
      }

      total
}

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
      fn test_complicated() {
            let lines = &[
                  "+------+----+",
                  "|      |    |",
                  "+---+--+    |",
                  "|   |       |",
                  "+---+-------+",
            ];
            assert_eq!(3, count(lines))
      }

      #[test]
      fn test_not_so_complicated() {
            let lines = &[
                  "+------+----+",
                  "|      |    |",
                  "+------+    |",
                  "|   |       |",
                  "+---+-------+",
            ];
            assert_eq!(2, count(lines))
      }

      #[test]
      fn test_large_input_with_many_rectangles() {
            let lines = &[
                  "+---+--+----+",
                  "|   +--+----+",
                  "+---+--+    |",
                  "|   +--+----+",
                  "+---+--+--+-+",
                  "+---+--+--+-+",
                  "+------+  | |",
                  "          +-+",
            ];
            assert_eq!(60, count(lines))
      }

      #[test]
      fn test_three_rectangles_no_shared_parts() {
            #[rustfmt::skip]
        let lines = &[
            "  +-+  ",
            "  | |  ",
            "+-+-+-+",
            "| | | |",
            "+-+ +-+"];
            assert_eq!(3, count(lines))
      }
}
