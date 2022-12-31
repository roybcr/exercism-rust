#[derive(Debug)]
pub struct ChessPosition(i32, i32);

#[derive(Debug)]
pub struct Queen(ChessPosition);

#[allow(dead_code)]
impl ChessPosition {
      pub fn new(rank: i32, file: i32) -> Option<Self> {
            match (rank, file) {
                  (0..=7, 0..=7) => Some(ChessPosition(rank, file)),
                  _ => None,
            }
      }
}

#[allow(dead_code)]
impl Queen {
      pub fn new(position: ChessPosition) -> Self { Queen(position) }

      pub fn can_attack(&self, other: &Queen) -> bool {
            let ((ro, fo), (rs, fs)) = ((other.0 .0, other.0 .1), (self.0 .0, self.0 .1));
            let abs_diff_eq = rs.abs_diff(ro) == fo.abs_diff(fs);
            abs_diff_eq || ro == rs || fo == fs
      }
}
