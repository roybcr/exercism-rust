#![feature(unboxed_closures)]

mod allergies;
mod circular_buffer;
mod clock;
mod dot_dsl;
mod lowpow_embedded_game;
mod luhn;
mod matching_brackets;
mod react;
mod rna_transcription;
mod robot_simulator;
mod short_fibonacci_seq;
mod spiral_matrix;

use crate::{rna_transcription::{Dna, Rna},
            short_fibonacci_seq::create_buffer};
use circular_buffer::CircularBuffer as CBuff;
use clock::Clock;
use lowpow_embedded_game::evens;
use luhn::is_valid;
pub use maplit;
use robot_simulator::{Direction::*, Robot};
use spiral_matrix::spiral_matrix;
pub use unicode_segmentation::UnicodeSegmentation;

fn main() {
    let allergies = allergies::Allergies::new(248);
    println!("{:#?}", allergies.allergies());

    let robot = Robot::new(7i32, 3i32, North);
    let instructions = "AAA";
    let rob = robot.instructions(instructions);
    println!("1: {:#?}\n2:{:#?}\n", Robot::new(7, 3, North), rob);

    let buff = create_buffer(12);
    println!("{:#?}", buff);
    println!("{:#?}", short_fibonacci_seq::fibonacci(12));

    let dna_strand = "ACGTGGTCTTAA";
    let rna_strand = "UGCACCAGAAUU";
    println!("{:#?}", Dna::new(dna_strand));
    println!("{:#?}", Rna::new(rna_strand));
    println!("{:#?}", Rna::from(Dna::new(dna_strand).unwrap()));
    println!("{:#?}", Dna::new("T"));
    assert_eq!(Dna::new("T").unwrap().into_rna(), Rna::new("A").unwrap());

    let c2_40: Clock = Clock::new(0, 160);
    let c4_43: Clock = Clock::new(0, 1723);
    let c = Clock::new(2, 20).add_minutes(-3000);
    println!("{:#?}\n{:#?}\n{:#?}\n",
             c2_40.to_string(),
             c4_43.to_string(),
             c.to_string());

    let matrix = spiral_matrix(5);
    println!("{:#?}", matrix);

    let mut eveniter = evens(0u8..);
    println!("{}", eveniter.next().unwrap());
    println!("{}", eveniter.next().unwrap());
    println!("{}", eveniter.next().unwrap());
    println!("{}", eveniter.next().unwrap());
    println!("{}", eveniter.next().unwrap());
    println!("{}", eveniter.next().unwrap());

    let mut c = CBuff::<i32>::new(10usize);

    for i in 0i32..10i32 {
        c.write(i).unwrap();
    }

    println!("{:#?}", c);
    for i in 10i32..14i32 {
        c.overwrite(i);
    }

    c.read().unwrap();
    c.write(99i32).unwrap();
    c.read().unwrap();
    c.write(340i32).unwrap();
    c.read().unwrap();
    c.write(23).unwrap();

    println!("{:#?}", c);

    let credit_number = "4539 3195 0343 6467";
    let credit_number2 = "5326 1412 0008 3994";
    let credit_number3 = "5326 1412 0008 3995";

    println!("IS VALID: {}", is_valid(credit_number));
    println!("IS VALID: {}", is_valid(credit_number2));
    println!("IS VALID: {}", is_valid(credit_number3));

    println!("IS VALID 59: {}", is_valid("59"));
    println!("IS VALID: {}", is_valid("055 444 286"));
    println!("IS VALID: {}", is_valid("055 444 285"));
    println!("IS VALID: {}", is_valid("7253 2262 5312 0539"));
    println!("IS VALID: {}", is_valid("8273 1232 7352 0569"));
}
