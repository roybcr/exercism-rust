pub const ISIZE: usize = 2;

#[allow(dead_code)]
pub fn create_empty() -> Vec<u8> { create_buffer(0usize) }

pub fn create_buffer(count: usize) -> Vec<u8> { vec![0u8; count] }

pub fn fibonacci(size: usize) -> Vec<u8> {
      let mut buff = vec![1u8; ISIZE];

      buff.extend_from_slice(&create_buffer(size - ISIZE)[..]);

      for i in ISIZE..buff.len() {
            buff[i] = buff[i - 1] + buff[i - 2];
      }

      buff
}
