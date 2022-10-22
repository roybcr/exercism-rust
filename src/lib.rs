mod allergies;
mod clock;
mod dot_dsl;
mod queen_attack;
mod rna_transcription;
mod robot_simulator;
mod tournament;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tournament() {
        let tournament_scoreboard =
            tournament::tally(format!("{}\n{}\n{}\n{}\n{}\n{}",
                                      "Allegoric Alaskans;Blithering Badgers;win",
                                      "Devastating Donkeys;Courageous Californians;draw",
                                      "Devastating Donkeys;Allegoric Alaskans;win",
                                      "Courageous Californians;Blithering Badgers;loss",
                                      "Blithering Badgers;Devastating Donkeys;loss",
                                      "Allegoric Alaskans;Courageous Californians;win",).as_str());

        assert_eq!(tournament_scoreboard,
                   format!("{}\n{}\n{}\n{}\n{}",
                           format!("{: <31}| MP |  W |  D |  L |  P", "Team"),
                           "Devastating Donkeys            |  3 |  2 |  1 |  0 |  7",
                           "Allegoric Alaskans             |  3 |  2 |  0 |  1 |  6",
                           "Blithering Badgers             |  3 |  1 |  0 |  2 |  3",
                           "Courageous Californians        |  3 |  0 |  1 |  2 |  1"));
    }

    #[test]
    fn queen_attack() {
        use queen_attack::{ChessPosition as CP, Queen as Q};
        let (q1, q2): (Q, Q) =
            (Q::new(CP::new(5, 7).unwrap()), Q::new(CP::new(2, 4).unwrap()));

        assert!(q1.can_attack(&q2));
    }
}
