use std::collections::HashMap;

#[allow(dead_code)]
pub fn brackets_are_balanced(string: &str) -> bool {
    #[rustfmt::skip]
    let hmap: HashMap<char, i32> = HashMap::from([('{', 1),('(', 2),('[', 3),('}', -1),(')', -2),(']', -3)]);
    let mut open_brackets: Vec<i32> = vec![];

    for ch in string.chars().into_iter() {
        if let Some(&bracket) = hmap.get(&ch) {
            if bracket.gt(&0) {
                open_brackets.push(bracket);
            } else {
                match open_brackets.pop() {
                    #[rustfmt::skip]
                    Some(closing) => if (bracket + closing).ne(&0) { return false; },
                    #[rustfmt::skip]
                    None => { return false; },
                }
            }
        }
    }

    open_brackets.is_empty()
}
