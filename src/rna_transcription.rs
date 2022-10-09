#[derive(Debug, PartialEq, Eq)]
pub struct Dna(String);

#[derive(Debug, PartialEq, Eq)]
pub struct Rna(String);

impl From<Dna> for Rna {
    fn from(dna: Dna) -> Rna {
        Rna(dna
            .0
            .chars()
            .map(|c| match c {
                'G' => 'C',
                'C' => 'G',
                'T' => 'A',
                _ => 'U',
            })
            .collect())
    }
}

pub fn map_chars(strand: &str, chars: [char; 4]) -> String {
    strand
        .chars()
        .map_while(|c| match chars.contains(&c) {
            true => Some(c),
            false => None,
        })
        .collect()
}

#[allow(dead_code)]
impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        let new_dna = map_chars(dna, ['G', 'C', 'A', 'T']);
        match dna.len().eq(&new_dna.len()) {
            true => Ok(Dna(dna.to_string())),
            false => {
                return Err(new_dna.len());
            }
        }
    }

    pub fn into_rna(self) -> Rna {
        Rna::from(self)
    }
}

#[allow(dead_code)]
impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        let new_rna = map_chars(rna, ['G', 'C', 'A', 'U']);
        match rna.len().eq(&new_rna.len()) {
            true => Ok(Rna(rna.to_string())),
            false => Err(new_rna.len()),
        }
    }
}
