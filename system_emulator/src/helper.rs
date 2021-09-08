use std::collections::HashSet;

pub fn rand_u8_with_exclusion(mut exlusion: Vec<u8>, amount: usize) -> Vec<u8> {
    let mut set1: HashSet<u8> = (1..=255).collect();
    let set2: HashSet<u8> = exlusion.drain(0..).collect();

    set1 = &set1 - &set2;
    set1.drain().collect::<Vec<u8>>()[0..amount].into()
}