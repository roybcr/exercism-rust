mod allergies;
mod ascii;
mod ascii_rectangles;
mod clock;
mod rna_transcription;
mod robot_simulator;
mod short_fibonacci_seq;

use crate::{rna_transcription::{Dna, Rna},
            short_fibonacci_seq::create_buffer};
use clock::Clock;
use robot_simulator::{Direction::*, Robot};

fn main() {
    // ------------------------------ allergies ------------------------------------

    let allergies = allergies::Allergies::new(248);
    println!("{:#?}", allergies.allergies());

    // ----------------------------- robot_simulator -------------------------------

    let robot = Robot::new(7i32, 3i32, North);
    let instructions = "AAA";
    let rob = robot.instructions(instructions);
    println!("1: {:#?}\n2:{:#?}\n", Robot::new(7, 3, North), rob,);

    // ---------------------------- short_fibonacci_seq ----------------------------

    let buff = create_buffer(12);
    println!("{:#?}", buff);
    println!("{:#?}", short_fibonacci_seq::fibonacci(12));

    // ----------------------------- rna_transcription -----------------------------

    let dna_strand = "ACGTGGTCTTAA";
    let rna_strand = "UGCACCAGAAUU";
    println!("{:#?}", Dna::new(dna_strand));
    println!("{:#?}", Rna::new(rna_strand));
    println!("{:#?}", Rna::from(Dna::new(dna_strand).unwrap()));
    println!("{:#?}", Dna::new("T"));
    assert_eq!(Dna::new("T").unwrap().into_rna(), Rna::new("A").unwrap());

    // ----------------------------- clock -----------------------------

    let c2_40: Clock = Clock::new(0, 160);
    let c4_43: Clock = Clock::new(0, 1723);
    let c = Clock::new(2, 20).add_minutes(-3000);
    println!("{:#?}\n{:#?}\n{:#?}\n",
             c2_40.to_string(),
             c4_43.to_string(),
             c.to_string());
}

// ------------------------------
// let ascii_rect: &[&str] = &["  +-+", "    |", "+-+-+", "| | -", "+-+-+"];
// let ascii_rect2: &[&str] = &["+---+--+----+",
//                              "|   +--+----+",
//                              "+---+--+    |",
//                              "|   +--+----+",
//                              "+---+--+--+-+",
//                              "+---+--+--+-+",
//                              "+------+  | |",
//                              "          +-+"];
// ascii_rectangles::count(ascii_rect);
// println!("{}", ascii::count(ascii_rect));
