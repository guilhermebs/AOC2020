use std::{collections::VecDeque, fs};

fn play(hands: &Vec<Vec<usize>>) -> usize {
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

    let part1_sol = play(&hands);
    println!("Part 1: {:?}", part1_sol)
}
