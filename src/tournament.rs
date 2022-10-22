use core::fmt;
use std::collections::HashMap;

// Tournament - https://exercism.org/tracks/rust/exercises/tournament
// Tally the results of a small football competition, based on an input
// containing which team played against which and what the outcome was.

pub type R = (u32, u32, u32);

#[derive(Debug, Clone, Copy)]
pub enum MatchResult {
    W,
    D,
    L,
}

impl From<&str> for MatchResult {
    fn from(mr: &str) -> Self {
        match mr {
            "loss" => Self::L,
            "draw" => Self::D,
            _ => Self::W,
        }
    }
}

#[derive(Debug)]
pub struct Team<'a> {
    pub name:    &'a str,
    pub points:  u32,
    pub results: R,
}

#[allow(dead_code)]
impl<'a> Team<'a> {
    pub fn new(name: &'a str, results: &R) -> Self {
        let points = results.0 * 3 + results.1;
        Team { name,
               points,
               results: *results }
    }
}

impl<'a> fmt::Display for Team<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let matches_played = self.results.0 + self.results.1 + self.results.2;

        let (name, mp, p, (w, d, l)): (&'a str, u32, u32, R) =
            (self.name, matches_played, self.points, self.results);

        let display = format!("{name: <31}|  {mp} |  {w} |  {d} |  {l} |  {p}");

        write!(f, "{}", display)
    }
}

#[allow(dead_code)]
pub fn tally(match_results: &str) -> String {
    let zeros = (0u32, 0u32, 0u32);
    let mut table: String = format!("{: <31}| MP |  W |  D |  L |  P", "Team");
    let mut teams: HashMap<&str, R> = HashMap::new();
    let result_mapper = |(a, b): (&R, &R), score: MatchResult| -> (R, R) {
        match score {
            MatchResult::W => ((a.0 + 1, a.1, a.2), (b.0, b.1, b.2 + 1)),
            MatchResult::D => ((a.0, a.1 + 1, a.2), (b.0, b.1 + 1, b.2)),
            MatchResult::L => ((a.0, a.1, a.2 + 1), (b.0 + 1, b.1, b.2)),
        }
    };

    for line in match_results.lines() {
        let line: Vec<&str> = line.split(';').take(3).collect();
        if line.len().ne(&3) {
            return table;
        }
        let [a, b, r]: [&str; 3] = [line[0], line[1], line[2]];
        let score: MatchResult = r.into();
        let (results_a, results_b) = match (teams.get(&a), teams.get(&b)) {
            (Some(ra), Some(rb)) => result_mapper((ra, rb), score),
            (Some(ra), None) => result_mapper((ra, &zeros), score),
            (None, Some(rb)) => result_mapper((&zeros, rb), score),
            (None, None) => result_mapper((&zeros, &zeros), score),
        };

        teams.insert(a, results_a);
        teams.insert(b, results_b);
    }

    let mut hash_vec: Vec<(&&str, &R)> = teams.iter().collect();

    hash_vec.sort_by(|a, b| {
                let atot = (a.1).0 * 3 + (a.1).1;
                let btot = (b.1).0 * 3 + (b.1).1;
                if atot.eq(&btot) {
                    a.0.cmp(b.0)
                } else {
                    btot.cmp(&atot)
                }
            });

    for t in hash_vec {
        let team = Team::new(t.0, t.1);
        table = format!("{}\n{}", table, &team.to_string()[..]);
    }

    table
}
