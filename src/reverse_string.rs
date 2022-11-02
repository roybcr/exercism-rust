// use unicode_segmentation::UnicodeSegmentation;

#[allow(dead_code)]
pub fn sized_chunks(input: &str, bounds: Vec<usize>) -> Vec<&[u8]> {
    let b = input.as_bytes();
    let mut cursor = 0usize;
    let mut vec: Vec<&[u8]> = Vec::with_capacity(bounds.len());

    for (idx, bound) in bounds.into_iter().enumerate() {
        let innvec = &b[cursor..cursor + bound];
        vec.insert(idx, innvec);
        cursor += bound;
    }

    vec
}

#[allow(dead_code)]
pub fn sized_chunks_rev(input: &str, bounds: Vec<usize>) -> Vec<&[u8]> {
    sized_chunks(input, bounds).into_iter().rev().collect()
}

#[allow(dead_code)]
pub fn reverse(input: &str) -> String {
    if input.is_empty() {
        return String::new();
    }

    let mut all_utf8 = true;

    let bounds = input.chars()
                      .map(|c| {
                          let len_utf8 = c.len_utf8();
                          all_utf8 = all_utf8 && len_utf8.eq(&1);
                          len_utf8
                      })
                      .collect::<Vec<usize>>();
    if all_utf8 {
        return input.chars().into_iter().rev().collect();
    }

    // let grapheme_approach: String = input.graphemes(true).rev().collect();
    sized_chunks_rev(input, bounds).into_iter()
              .fold(String::new(), |mut acc, chunk| {
                  acc.push_str(&String::from_utf8(chunk.to_vec()).unwrap()[..]);
                  acc
              })
}

#[cfg(test)]
mod test {
    use super::reverse as rev;
    fn process_reverse_case(input: &str, expected: &str) {
        assert_eq!(&rev(input), expected);
    }

    #[test]
    fn test_an_empty_string() { process_reverse_case("", ""); }
    #[test]
    fn test_a_word() { process_reverse_case("robot", "tobor"); }
    #[test]
    fn test_a_capitalized_word() { process_reverse_case("Ramen", "nemaR"); }
    #[test]
    fn test_a_palindrome() { process_reverse_case("racecar", "racecar"); }
    #[test]
    fn test_an_even_sized_word() { process_reverse_case("drawer", "reward"); }
    #[test]
    fn test_wide_characters() { process_reverse_case("子猫", "猫子"); }
    #[test]
    #[cfg(feature = "grapheme")]
    fn test_grapheme_clusters() { process_reverse_case("uüu", "uüu"); }
}
