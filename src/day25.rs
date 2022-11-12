const SUBJECT_NUMBER: u64 = 7;

fn encrypt(subject_number: u64, loop_size: u64) -> u64 {
    let mut result = 1;
    for _i in 0..loop_size {
        result = (result * subject_number)%20201227;
    }
    return result;
}


fn find_loop_size(subject_number: u64, public_key: u64) -> u64 {
    let mut result = 0;
    let mut key = 1;
    while key != public_key {
        result += 1; 
        key = (key * subject_number)%20201227;
    }
    return result;
}


#[allow(dead_code)]
pub fn day25() {
    assert!(5764801 == encrypt(SUBJECT_NUMBER, 8));
    assert!(8 == find_loop_size(SUBJECT_NUMBER, 5764801));

    let door_public_key = 18356117;
    let card_public_key = 5909654;

    let door_loop_size = find_loop_size(SUBJECT_NUMBER, door_public_key);
    let card_loop_size = find_loop_size(SUBJECT_NUMBER, card_public_key);


    let solution = encrypt(door_public_key, card_loop_size);
    let solution_check = encrypt(card_public_key, door_loop_size);
    assert!(solution == solution_check);

    println!("Solution: {:}", solution)



}
