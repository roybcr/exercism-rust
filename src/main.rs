mod allergies;
mod rna_transcription;
mod robot_simulator;
mod short_fibonacci_seq;

use crate::rna_transcription::{Dna, Rna};
use crate::short_fibonacci_seq::create_buffer;
use robot_simulator::{Direction::*, Robot};

fn main() {
    // debugging stuff here
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
}
