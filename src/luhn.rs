pub fn is_valid(code: &str) -> bool {
    let code: Vec<char> = code.chars().filter(|c| !c.is_whitespace()).collect();
    if code.len().le(&1) {
        return false;
    }
    let mut valid = true;
    let mut sum = 0;

    code.iter()
        .rev()
        .enumerate()
        .for_each(|(i, &c)| match char::to_digit(c, 10) {
            Some(d) =>
                if (i + 1) % 2 != 0 {
                    sum += d;
                } else {
                    let calc = d * 2;
                    sum += if calc > 9 { calc - 9 } else { calc };
                },
            None => {
                valid = false;
                return;
            },
        });

    valid && sum % 10 == 0
}
