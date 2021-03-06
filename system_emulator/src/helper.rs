use std::collections::HashSet;

//generates a random number of u8 values that are not already in a list
pub fn rand_u8_with_exclusion(mut exlusion: Vec<u8>, mut amount: usize) -> Vec<u8> {
    let mut set1: HashSet<u8> = (1..=255).collect();
    let set2: HashSet<u8> = exlusion.drain(0..).collect();

    set1 = &set1 - &set2;
    if amount > set1.len() {
        amount = set1.len()
    }
    set1.drain().collect::<Vec<u8>>()[0..amount].into()
}
