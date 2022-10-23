use std::{
    collections::{hash_map::DefaultHasher, HashSet, VecDeque},
    fs,
    hash::{Hash, Hasher},
};

fn combat(hands: &Vec<Vec<usize>>) -> usize {
    let mut hand_player1 = VecDeque::from_iter(hands.first().unwrap());
    let mut hand_player2 = VecDeque::from_iter(hands.last().unwrap());

    while hand_player1.len() > 0 && hand_player2.len() > 0 {
        let card_p1 = hand_player1.pop_front().unwrap();
        let card_p2 = hand_player2.pop_front().unwrap();
        if card_p1 > card_p2 {
            hand_player1.push_back(card_p1);
            hand_player1.push_back(card_p2);
        } else if card_p2 > card_p1 {
            hand_player2.push_back(card_p2);
            hand_player2.push_back(card_p1);
        } else {
            panic!("Cards are the same!")
        }
    }

    let winning_hand = match hand_player2.len() == 0 {
        true => hand_player1,
        false => hand_player2,
    };

    return winning_hand
        .iter()
        .enumerate()
        .map(|(i, &c)| (winning_hand.len() - i) * c)
        .sum();
}

fn create_hash(hand_p1: &VecDeque<usize>, hand_p2: &VecDeque<usize>) -> u64 {
    let mut hasher = DefaultHasher::new();
    hand_p1.hash(&mut hasher);
    hand_p2.hash(&mut hasher);
    return hasher.finish();
}

fn recursive_game<'a>(start_hand_p1: &[usize], start_hand_p2: &[usize]) -> (bool, usize) {
    //println!("Starting Game");
    let mut hand_p1 = VecDeque::from_iter(start_hand_p1.iter().map(|&x| x));
    let mut hand_p2 = VecDeque::from_iter(start_hand_p2.iter().map(|&x| x));
    let mut round_record = HashSet::new();
    let p1_wins_game = loop {
        // check the record
        let hands = create_hash(&hand_p1, &hand_p2);
        if round_record.contains(&hands) {
            //println!("Breaking due to repeated hands");
            break true;
        } else {
            round_record.extend([hands])
        }
        let card_p1 = hand_p1.pop_front().unwrap();
        let card_p2 = hand_p2.pop_front().unwrap();
        //println!("Round {:?}", round_record.len());
        //println!("P1: {:?} P2: {:?}", hand_p1, hand_p2);
        //println!("P1: {:?} P2: {:?}", card_p1, card_p2);
        // Determine winer
        let p1_wins_round;
        if hand_p1.len() >= card_p1 && hand_p2.len() >= card_p2 {
            p1_wins_round = recursive_game(
                &hand_p1.make_contiguous()[..card_p1],
                &hand_p2.make_contiguous()[..card_p2],
            )
            .0;
        } else {
            p1_wins_round = card_p1 > card_p2;
        }
        // Update hands
        if p1_wins_round {
            //println!("P1 wins round!");
            hand_p1.push_back(card_p1);
            hand_p1.push_back(card_p2);
        } else {
            //println!("P2 wins round!");
            hand_p2.push_back(card_p2);
            hand_p2.push_back(card_p1);
        };
        if hand_p2.len() == 0 {
            break true;
        } else if hand_p1.len() == 0 {
            break false;
        }
    };

    let n_cards = hand_p1.len() + hand_p2.len();
    let winning_hand = match p1_wins_game {
        true => hand_p1,
        false => hand_p2,
    }
    .iter()
    .enumerate()
    .map(|(i, &c)| (n_cards - i) * c)
    .sum();
    //println!("End of Game!");
    return (p1_wins_game, winning_hand);
}

#[allow(dead_code)]
pub fn day22() {
    let file_contents = fs::read_to_string("inputs/day22").unwrap();
    let hands = file_contents
        .split("\n\n")
        .map(|h| {
            h.split("\n")
                .skip(1)
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let part1_sol = combat(&hands);
    println!("Part 1: {:?}", part1_sol);
    println!(
        "Part 2: {:?}",
        recursive_game(hands.first().unwrap(), hands.last().unwrap()).1
    )
}
