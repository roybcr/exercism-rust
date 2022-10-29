mod allergies;
mod clock;
mod dot_dsl;
mod lowpow_embedded_game;
mod queen_attack;
mod reverse_string;
mod rna_transcription;
mod robot_simulator;
mod spiral_matrix;
mod tournament;

#[cfg(test)]
mod tests {
    use super::*;
    use reverse_string::reverse as rev;

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

    #[test]
    fn reverse() {
        const SOME_STR: &'static str = "uüu";
        const REV_STR: &'static str = "uüu";

        let revstr = rev(SOME_STR);
        let somestr = rev(REV_STR);
        println!("{:#?}", rev(SOME_STR));
        println!("{:#?}", rev(REV_STR));

        assert_eq!(revstr, REV_STR);
        assert_eq!(somestr, SOME_STR);
        assert_eq!(rev(revstr.as_str()), somestr);
    }

    /// Process a single test case for the property `reverse`
    fn process_reverse_case(input: &str, expected: &str) {
        assert_eq!(&rev(input), expected)
    }

    #[test]
    /// empty string
    fn test_an_empty_string() { process_reverse_case("", ""); }

    #[test]
    /// a word
    fn test_a_word() { process_reverse_case("robot", "tobor"); }

    #[test]
    /// a capitalized word
    fn test_a_capitalized_word() { process_reverse_case("Ramen", "nemaR"); }

    #[test]
    /// a sentence with punctuation
    fn test_a_sentence_with_punctuation() {
        process_reverse_case("I'm hungry!", "!yrgnuh m'I");
    }

    #[test]
    /// a palindrome
    fn test_a_palindrome() { process_reverse_case("racecar", "racecar"); }

    #[test]
    /// an even-sized word
    fn test_an_even_sized_word() { process_reverse_case("drawer", "reward"); }

    #[test]
    /// wide characters
    fn test_wide_characters() { process_reverse_case("子猫", "猫子"); }

    #[test]
    #[cfg(feature = "grapheme")]
    /// grapheme clusters
    fn test_grapheme_clusters() { process_reverse_case("uüu", "uüu"); }
}
