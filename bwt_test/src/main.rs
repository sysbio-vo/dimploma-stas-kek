#![allow(dead_code)]
use std::time::Instant;
use std::collections::HashMap;
use itertools::Itertools;


#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Hash)]
enum Nucleotide {
    END,
    A,
    G,
    T,
    C,
}


fn string_to_nucleotides(input_string: &str) -> Vec<Nucleotide> {
    let mut output_vec = Vec::new();
    for character in input_string.chars() {
        output_vec.push(match character {
            'A' => Nucleotide::A,
            'G' => Nucleotide::G,
            'T' => Nucleotide::T,
            'C' => Nucleotide::C,
            'a' => Nucleotide::A,
            'g' => Nucleotide::G,
            't' => Nucleotide::T,
            'c' => Nucleotide::C,
            '$' => Nucleotide::END,
            _ => Nucleotide::END,
        })
    }
    output_vec.push(Nucleotide::END);
    output_vec
}

fn naive_bwt(some_sequence: &Vec<Nucleotide>) -> Vec<Nucleotide> {
    let sequence_length = some_sequence.len();

    // First step of BWT: we need to generate array of rotations
    println!("{:?}", some_sequence);
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

/*
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
*/

fn rank_bwt(bwt: &Vec<Nucleotide>) -> (Vec<usize>, HashMap<Nucleotide, usize>){
    let mut tots = HashMap::new();
    //prefill empty tots
    tots.insert(Nucleotide::A, 0);
    tots.insert(Nucleotide::G, 0);
    tots.insert(Nucleotide::T, 0);
    tots.insert(Nucleotide::C, 0);
    tots.insert(Nucleotide::END, 0);
    
    let mut ranks = Vec::new();

    for nucleotide in bwt{
        ranks.push(tots[nucleotide]);
        *tots.entry(*nucleotide).or_insert(0) += 1;
    }
    //dbg!{&tots};
    return (ranks, tots);
}

fn first_column(tots: &HashMap<Nucleotide, usize>) -> HashMap<&Nucleotide, (usize, usize)>{
    let mut first = HashMap::new();
    let mut totc = 0;
    for (nucleotide, count) in tots.iter().sorted() {
        first.insert(nucleotide, (totc, totc + count));
        totc += count;
    }
    return first

}

fn reverse_bwt(bwt: &Vec<Nucleotide>) -> Vec<Nucleotide>{
    let time_start = Instant::now();
    let (ranks, tots) = rank_bwt(bwt);
    let first = first_column(&tots);
    let mut rowi = 0;
    let mut result = Vec::new();
    while bwt[rowi] != Nucleotide::END{
        let nucleotide = bwt[rowi];
        result.push(nucleotide);
        rowi = first[&nucleotide].0 + ranks[rowi];
    }
    println!("Reverse BWT took: {:?} ms", time_start.elapsed().as_millis());
    result.reverse();
    return result;
}

fn generate_sa(bwt: &Vec<Nucleotide>) -> Vec<usize>{
    let time_start = Instant::now();
    let (ranks, tots) = rank_bwt(bwt);
    let first = first_column(&tots);
    let mut rowi = 0;
    let mut i = 0;
    //let mut suffix_array = Vec::new();
    let mut suffix_array = vec![0; bwt.len()];
    while bwt[rowi] != Nucleotide::END{
        let nucleotide = bwt[rowi];
        //dbg!(&nucleotide);
        //dbg!(bwt.len() - &i);
        //dbg!(&rowi);
        suffix_array[rowi] = i;
        i += 1;
        rowi = first[&nucleotide].0 + ranks[rowi];
    }
    suffix_array[rowi] = i; //FIX END JUST IN CASE

    println!("SA generation took: {:?} ms", time_start.elapsed().as_millis());
    //suffix_array.reverse();
    //println!("{:?}", &suffix_array);
    //println!("{:?}", &bwt);
    return suffix_array;
}


fn rank_all_bwt(bwt: &Vec<Nucleotide>) -> (HashMap<Nucleotide, Vec<usize>>, HashMap<Nucleotide, usize>){
    let mut tots = HashMap::new();
    //prefill empty tots
    tots.insert(Nucleotide::A, 0);
    tots.insert(Nucleotide::G, 0);
    tots.insert(Nucleotide::T, 0);
    tots.insert(Nucleotide::C, 0);
    tots.insert(Nucleotide::END, 0);
    
    let mut rank_all = HashMap::new();
    rank_all.insert(Nucleotide::A, Vec::new());
    rank_all.insert(Nucleotide::G, Vec::new());
    rank_all.insert(Nucleotide::T, Vec::new());
    rank_all.insert(Nucleotide::C, Vec::new());
    rank_all.insert(Nucleotide::END, Vec::new());

    for nucleotide in bwt{
        *tots.entry(*nucleotide).or_insert(0) += 1;
        for nucleotide_2 in tots.keys().sorted(){
            rank_all.entry(*nucleotide_2).or_insert(Vec::new()).push(tots[nucleotide_2]);
        }
    }
    //dbg!{&tots};
    return (rank_all, tots);
}

fn count_matches_and_locations(bwt: &Vec<Nucleotide>, sa: &Vec<usize>, search_pattern: &Vec<Nucleotide>) -> usize{
    //546 - 575
    println!("Searching for pattern: {:?}", search_pattern);
    let time_start = Instant::now();
    let (rank_all, tots) = rank_all_bwt(bwt);
    let first = first_column(&tots);
    let pattern = &search_pattern[..search_pattern.len()-1]; // because we shouldn't serach for END token
    let (mut left, mut right) = first[pattern.last().unwrap()];
    let mut i = (pattern.len() - 2) as i64; // important to convert otherwise it can underflow
    while (i >= 0) & (right > 1){
        let nucleotide = pattern[i as usize];
        if left == 0 {
            left = 1;//bwt.len();
        }
        left  = first[&nucleotide].0 + rank_all[&nucleotide][left-1];
        right = first[&nucleotide].0 + rank_all[&nucleotide][right-1];
        i -= 1;
    }

    //let mut positions = Vec::new();
    let bwt_len  = bwt.len();
    //dbg!(bwt_len - &sa[right]);
    //dbg!(bwt_len - &sa[left]);
    for pos in (left..right){
        println!("FOUND MATCH AT POSITION: {:?}", (bwt_len - &sa[pos]));
    }
    /*
    dbg!(&sa[left]);
    dbg!(&sa[right]);
    for nucleotide in rank_all.keys().sorted(){
        if &pattern[0] == nucleotide{
            dbg!(&nucleotide);
            dbg!(first[&nucleotide].0);
            dbg!(first[&nucleotide].0 + rank_all[&nucleotide][right]);
            dbg!(bwt_len - first[&nucleotide].0 + rank_all[&nucleotide][right]);
            dbg!(rank_all[&nucleotide][left]);
            dbg!(rank_all[&nucleotide][left-1]);
            dbg!(rank_all[&nucleotide][right]);
            dbg!(rank_all[&nucleotide][right-1]);
            dbg!(right);
            dbg!(rank_all[&nucleotide][right]);
        }
    }
    */
    return right-left;
}



fn count_matches(bwt: &Vec<Nucleotide>, search_pattern: &Vec<Nucleotide>) -> usize{
    let time_start = Instant::now();
    let (ranks, tots) = rank_bwt(bwt);
    let first = first_column(&tots);
    let pattern = &search_pattern[..search_pattern.len()-2]; // because we shouldn't serach for END token

    let (mut left, mut right) = first[pattern.last().unwrap()]; 
    let mut i = (pattern.len() - 2) as i64; // ignore END token, important to convert otherwise it can underflow
    while (i >= 0) & (right > 1){
        //println!("{:?}", i);
        let nucleotide = pattern[i as usize];
        //println!("{:?}", pattern);
        //println!("{:?}", nucleotide);
        let mut j = left;
        //println!("{:?}", left);
        //println!("{:?}", right);
        while j < right{
            if bwt[j] == nucleotide{
                left = first[&nucleotide].0 + ranks[j];
                break;
            }
            j += 1;
        }
        if j == right{
            left = right;
            break; // NO MATCH
        }
        right -= 1;
        while bwt[right] != nucleotide{
            right -= 1;
        }
        right = first[&nucleotide].0 + ranks[right] + 1;
        i -= 1;
    }
    println!("{:?}", right);
    println!("{:?}", left);
    println!("Pattern search took : {:?} ms", time_start.elapsed().as_millis());
    return right - left;
}
fn main() {
    //let some_sequence = string_to_nucleotides(&"GGTAA".repeat(1_000_000));
    let some_sequence = string_to_nucleotides(&"AGAGGTGCTGACGTTTGCGCCGCGCACGACTATGTCCGCCATGTTAGTGTCTGACGCAGGCGCCATTACAGGAGCCGTCTCTCTTGAGGCGACCGTGTCCGCATAAGTTGTCAGTCCCCAGCAGTGTTTGGGAGCCTTAACAGGCTTGCTAAGTGCCTGAGATTAAAGGCGAAGAGGAAGAAGAGCAAGAAGGGCAACCCGGTGTTTAACACACGGCTTTCAGGCGCTCTCAGGTGGCGCCAAGTTCAGTTTCGACAGGGATATGCATCCGGGAGAGAATTTTTCAGCCCCCGCCCCCACTGGCATGTGAGAGGAGGAACTCGCTGTTGCCCTGTGTCTTCCCCAGTGCGTTCTATAACCAGCCTCATTTTTTTTTTTAGCAATAGGCCGGATGTCCTAAGCGACCGGCATTCAAAGCCCGGTGCATTTTAGTCTCATCAGGAGTCAGGCGTGTCACGAGATTTTTCCGCGGTAGCTGCTCAGCTCTGACTCGTGAGCGGCGCACAATTGGCTAAGAATGGGTTTAGCATTTTGGCGCTCCCCCCCGGCGCTCCTGGAAACACTGCGTGGCACCCCGTACCACTGTCACATTATAAACGTGCTAAGGCTTGGGTGTCTCGGCTGGGGAGCAGTCAGGCAGCGCCCAGGGCGGGAGGGAATCAGACTAGATTCTCGCCGCGGAGACCCAGGGCCGCGCGGACCAATGGGGCAGGGAAGCCGTCGTAGGGCGGGGCAGTGAGGGGCGGTGCCGCGCTGGTCCCGCGCCGGTCCCGGTGGGCCGACTTCCGGCGAGCGGGCAAGCGGGAACCAGGTGGCCACCCGGTGTCGGTGCGTAAACCCCCTCCGCTTTTACACGGAGCGCGCTTAGGCTTTGGTTTGGGGATTTAAGATCGCGAACTGAGTGAAACCATAGAAACTGATTTGTAGAAATTTTCCTGTTGTTTGTAAATGCGGTGGGTGGGGGTTGGCCATGTGCTTTTCACTTGCTACTGTTAGCTGGTTTCTATTTGCTCTTTTACTTTTCCTGTTTTCTAAGATGTCAAGGCCTTTTTGTGTCTTTATACATTTTTAGGACATAACCCTAAATTGGTGTACTTTCTCTTTATGCACCCGTTGTCTCTGGCAGATTACTTGGTTAATTTTGTAGTTGGTTTCTACTTTATTCTCCATATCCTCTGATACGGCTTTATTTGCCTGTCTTATTCTGCTTATGTTACCCATTCTGAGGAATGAGGCCCAAGGTTAGTGAAGTCCCCCAAGGTCTTCCTGCTAGTATGGGGCAGCGTCTCTACCCAGGCCAAGGCGCTGTGTGAGACTCAGGAGCCCAGGCGTCCTCACCACTGGGTAGTAAGGCTATTTTCAGTGATCCACTAGAGTAAACAGAATTTTCCCACACTACTGCTATATTCAAACTAAAGATGTAACATTTGCTCTAGATTTAAAGATCTGTTTATTTCTGGCCGGATGTGATGTAATCCCAGCACTTGGGAGGCCAAGGTGGGTTGACCATTGGAGGTCAGGAGTTCGAGACCAGCCTGGCCAACGTAGTGAAACCCCGTCGCTACTAAAAATACAAAAATTCGCCTGGCGTGGCGGCGCACGCCCGTAATCCCAGCTACTTGGGAGGCTGTGAGGCAAGAGAATCGCTTGAACCCGGGAGGCGGAGGTTGTAGTGAGCTGAGATCACACCACTGCCCTCCAGCCTGGGCGACAGAGAGAGACTGTCTAAACAAATAAATAAATAAATAAAATAAAAAATCTGTTTATTTCAGATACACATGGCAAGTAAAATGTGTTTTCCCGTTACACTAAAGTAGCTGTTCCTTCCGAGACTGTAATTTAACAGGGTTTTTTTTTTTCCTTTCTCAAATTTTCAATGATCATCGTTTCACTTGTTGAGAGAATCTGAAGCAAAATTGCTTCTGTTAGGCTCCACCCCCTCTTACCTGTGTGACCTCCAGCAAGTTATTTACCTATCTGGGCCTCCTTGTCTTATCTGCAAATTGGTGATAGTAATAGTATCTCCCTCATAGGATTCTTTCATGACAAGCATTTTAAATAGTACCCTGCCCAAGGTTAAGTGCTCAAAAAAATGTTAGTTTTGTATATTGTCAGCTTACATTTTTCTTAAAATATGTAATCCCAACATCATCAAAATAAGTTTTCAATTTTCTTTTTCCTGTTACCAGTTTGTCTTGTGGTAAGTAAATGTAACAAGTTAACATTTACTACTGCTGAGTGCCAGATTGTACTTATTTATAAATGTCATTTCAGTTAATCTCCTAGGTTACACTGCTGTTGTATCCATCCTTCTAAGTCTCAGGTGCCAGAACTAAAGTGAAATGAGTTGGAATTTGATTTTTGCCTTCTCTACCAATATTCTAGCATATTGCGCTATTTGAATTAGGAGTCAGCAAACTATGGCCCACATACCAAATCCTTCTGCTGCCTGCTTTTGTACAGCTCACCAAATATGAATGATTGAGAAAAAGAACAGGAAATAAATGAGGTTTTTGTGACATGTGAAAATTTTATGAAATTCAAATTATAGTATCCGTACATAAAGTTTTATTGAAAGCGAGTCACGTTTATTTGTTACAAACCTTTGATGGTTTCCTGCTACTTAAAGTACTGATAGGCAACAGTGATCTCTGACCCACTGCCTGTTTATGGGTCCATTAAATTTTTCTGGACATTGGCACATTGATTCATCTTCGTATTATCTTTGGTTGCTTTCCTGCTAAGGTGGCCGAGTCGAGTAGTTTTGACAGAACTATGTGGCCCTTTTTGTGTAAAGTTTGCTTACCCCAGTTTTCAGTATATCCTTTAATTTATGATAACCTAATTATCTGTAAGATTTCAAGGTTTAAAAATTGATATTTTATAATACATTTTGAGTCT");
    //let some_sequence = string_to_nucleotides(&"TTTCAAATTTG");

    println!("Sequence length :{:?}", some_sequence.len());
    //println!("Our sequence: {:?}", some_sequence);
    let time_start = Instant::now();
    println!("Ten first symbols of sequence {:?}", &some_sequence[0..10]);
    //let result = suffix_bwt_40(&some_sequence);
    //std::thread::sleep(std::time::Duration::from_secs(20));
    let result = naive_bwt(&some_sequence);
    println!("Sequence len: {:?}", &some_sequence.len());
    println!("BWT len: {:?}", &result.len());
    println!("Reverse BWT len: {:?}", reverse_bwt(&result).len());
    //println!("Reverse BWT: {:?}", reverse_bwt(&result));
    //println!("BWT: {:?}", result);
    println!("Generating BWT took: {:?} ms", time_start.elapsed().as_millis());
    let time_start = Instant::now();
    let sa = generate_sa(&result);
    //let search_pattern = string_to_nucleotides("CGGCGCTCCTGGAAACACTGCGTGGCACC");
    let search_pattern = string_to_nucleotides("AGGTGCTGACGTTTGCGCCGCGCACGACTATGTCCGCCATGTTAGTGTCTGACGCAGGCGCCATTACAGGAGCCGTCTCTCTTGAGGCGACCGTGTCCGCATAAGTTGTCAGTCCCCAG");
    //println!("Found {:?} patterns", count_matches(&result, &search_pattern));
    println!("Found {:?} patterns", count_matches_and_locations(&result, &sa, &search_pattern));
    let search_pattern = string_to_nucleotides("AGGTG");
    println!("Found {:?} patterns", count_matches_and_locations(&result, &sa, &search_pattern));
}
