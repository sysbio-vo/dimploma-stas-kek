#![allow(dead_code)]
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

fn naive_bwt(some_sequence: &Vec<Nucleotide>) -> Vec<Nucleotide> {
    let sequence_length = some_sequence.len();

    // First step of BWT: we need to generate array of rotations
    let mut rotations = Vec::with_capacity(sequence_length);
    for rotation in 0..sequence_length {
        let generated_rotation = [
            &some_sequence[rotation..sequence_length],
            &some_sequence[0..rotation],
        ]
        .concat();
        rotations.push(generated_rotation.clone());
    }

    //println!("Rotations: {:?}", rotations);
    rotations.sort();
    //println!("Sorted rotations {:?}", rotations);
    let mut bwt = Vec::with_capacity(sequence_length);
    for rotation in rotations.iter() {
        bwt.push(*rotation.last().unwrap());
    }
    bwt
}

fn suffix_bwt_40(sequence: &Vec<Nucleotide>) -> Vec<Nucleotide> {
    let sequence_length = sequence.len();
    let mut rotations = Vec::with_capacity(sequence_length);
    for rotation in 1..sequence_length {
        let diff = sequence_length as i64 - rotation as i64 + 40;
        let generated_rotation = if (sequence_length - rotation) > 40 {
            [
                &sequence[rotation..(rotation + 20)],
                &sequence[(rotation + 20)..(rotation + 40)],
            ]
            .concat()
        } else {
            [
                &sequence[rotation..sequence_length],
                &sequence[0..diff as usize],
            ]
            .concat()
        };
        rotations.push(generated_rotation.clone());
    }
    //println!("Rotations: {:?}", rotations);
    //rotations.sort();
    println!("{:?}", std::mem::size_of_val(&rotations));
    //println!("Rotations: {:?}", rotations);
    //rotations.sort();
    //println!("Sorted rotations {:?}", rotations);
    let mut bwt = Vec::with_capacity(sequence_length);
    for rotation in rotations.iter() {
        bwt.push(*rotation.last().unwrap());
    }
    bwt
}

fn main() {
    let some_sequence = string_to_nucleotides(&"GGTAA".repeat(200_000_000));
    println!("Sequence length :{:?}", some_sequence.len());
    //println!("Our sequence: {:?}", some_sequence);
    let time_start = Instant::now();
    println!("{:?}", &some_sequence[0..10]);
    //let result = suffix_bwt_40(&some_sequence);
    std::thread::sleep(std::time::Duration::from_secs(20));
    //let result = naive_bwt(&some_sequence);
    //println!("BWT: {:?}", result);
    println!("Time took millis: {:?}", time_start.elapsed().as_millis());
}
