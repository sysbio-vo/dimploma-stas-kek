#![allow(dead_code)]
#![feature(duration_as_u128)]
use std::time::Instant;

#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
enum Nucleotide {
    A,
    G,
    T,
    C,
    N,
    END,
}

fn string_to_nucleotides(input_string: &str) -> Vec<Nucleotide> {
    let mut output_vec = Vec::new();
    for character in input_string.chars() {
        output_vec.push(match character {
            'A' => Nucleotide::A,
            'G' => Nucleotide::G,
            'T' => Nucleotide::T,
            'C' => Nucleotide::C,
            '$' => Nucleotide::END,
            _ => Nucleotide::N,
        })
    }

    output_vec.push(Nucleotide::END);
    output_vec
}

fn main() {
    let some_sequence = string_to_nucleotides("GGTAAGTGCTCTAGTACAAACACCCCCAATATTGTGATATAATTAAAATTATATTCATATTCTGTTGCCAGATTTTACACTTTTAGGCTATATTAGAGCCATCTTCTTTGAAGCGTTGTCTATGCATCGATCGACGACTG");

    // First step of BWT: we need to generate array of rotations
    let mut rotations = Vec::new();
    let sequence_length = some_sequence.len();
    let time_start = Instant::now();
    for rotation in 0..sequence_length {
        let generated_rotation = [
            &some_sequence[rotation..sequence_length],
            &some_sequence[0..rotation],
        ]
        .concat();
        rotations.push(generated_rotation.clone());
    }

    println!("Our sequence: {:?}", some_sequence);
    //println!("Rotations: {:?}", rotations);
    rotations.sort();
    //println!("Sorted rotations {:?}", rotations);
    let mut bwt = Vec::new();
    for rotation in rotations.iter() {
        bwt.push(rotation.last().unwrap());
    }
    println!("BWT: {:?}", bwt);
    println!("Time took millis: {:?}", time_start.elapsed().as_millis());
}
