use crate::{Solution, SolutionType};
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::read_to_string;
use std::time::Instant;
///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> Solution {
    // Your solution here...
    let text = read_to_string("src/aoc2023/texts/day7").unwrap();
    let time = Instant::now();

    let mut p1_cards = text
        .lines()
        .map(|line| line.split_once(" ").unwrap())
        .map(|(cards, bet)| Hand::new(cards, bet, false))
        .collect::<Vec<Hand>>();

    p1_cards.sort();

    let p1: u64 = p1_cards
        .iter()
        .enumerate()
        .map(|(idx, hand)| hand.bet * (idx as u64 + 1)).sum();

    let mut p2_cards = text
        .lines()
        .map(|line| line.split_once(" ").unwrap())
        .map(|(cards, bet)| Hand::new(cards, bet, true))
        .collect::<Vec<Hand>>();

    p2_cards.sort();

    let p2: u64 = p2_cards
        .iter()
        .enumerate()
        .map(|(idx, hand)| hand.bet * (idx as u64 + 1)).sum();

    let sol1: u64 = p1;
    let sol2: u64 = p2;

    let solution = (SolutionType::U64(sol1), SolutionType::U64(sol2));
    let time_ms = time.elapsed().as_nanos() as f64 / 1000000.0;
    Solution {
        solution,
        time_ms,
    }
}

#[derive(Debug, PartialEq, Eq, Ord)]
struct Hand {
    cards: [Cards; 5],
    value: HandValue,
    bet: u64,
}

// reversed casue it makes it sort by Greatest to Highest
impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.value == other.value {
            for idx in 0..5 {
                let cmp = other.cards[idx].cmp(&self.cards[idx]);
                if cmp != Ordering::Equal {
                    return Some(cmp);
                }
            }
            Some(Ordering::Equal)
        } else if self.value > other.value {
            Some(Ordering::Less)
        } else {
            Some(Ordering::Greater)
        }
    }
}

impl Hand {
    pub fn new(cards: &str, bet: &str, j: bool) -> Self {
        let bet = bet.parse::<u64>().unwrap();
        let cards = cards
            .chars()
            .map(|c| Cards::new(c, j))
            .collect::<Vec<Cards>>();
        let hand_value = HandValue::new(cards.clone(), j);

        Self {
            cards: [cards[0], cards[1], cards[2], cards[3], cards[4]],
            value: hand_value,
            bet,
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
enum Cards {
    Ace,
    King,
    Queen,
    Jack,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
    One,
    Joker,
}

impl Cards {
    pub fn new(c: char, j: bool) -> Self {
        if j {
            match c {
                'A' => Self::Ace,
                'K' => Self::King,
                'Q' => Self::Queen,
                'J' => Self::Joker,
                'T' => Self::Ten,
                '9' => Self::Nine,
                '8' => Self::Eight,
                '7' => Self::Seven,
                '6' => Self::Six,
                '5' => Self::Five,
                '4' => Self::Four,
                '3' => Self::Three,
                '2' => Self::Two,
                '1' => Self::One,
                _ => unreachable!()
            }
        } else {
            match c {
                'A' => Self::Ace,
                'K' => Self::King,
                'Q' => Self::Queen,
                'J' => Self::Jack,
                'T' => Self::Ten,
                '9' => Self::Nine,
                '8' => Self::Eight,
                '7' => Self::Seven,
                '6' => Self::Six,
                '5' => Self::Five,
                '4' => Self::Four,
                '3' => Self::Three,
                '2' => Self::Two,
                '1' => Self::One,
                _ => unreachable!()
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandValue {
    FiveKind,
    FourKind,
    FullHouse,
    ThreeKind,
    TowPairs,
    OnePairs,
    HighCard,
}

impl HandValue {
    pub fn new(mut cards: Vec<Cards>, j: bool) -> Self {
        cards.sort();

        let mut counts = HashMap::new();
        for card in cards {
            let count = counts.entry(card).or_insert(0);
            *count += 1;
        }

        if j {
            let joker_count = *counts
                .get(&Cards::Joker)
                .unwrap_or(&0);

            counts.remove(&Cards::Joker);

            let mut highest_card = Cards::Joker;
            let mut higest_count = 0;

            for (card, count) in counts.iter() {
                if *count > higest_count {
                    higest_count = *count;
                    highest_card = *card;
                }
            }

            let count = counts.entry(highest_card).or_insert(0);
            *count += joker_count;
        }

        let mut counts = counts.iter().collect::<Vec<(&Cards, &u64)>>();
        counts.sort_by(|a, b| b.1.cmp(a.1));
        match counts[0].1 {
            5 => Self::FiveKind,
            4 => Self::FourKind,
            3 => {
                if counts[1].1 == &2 {
                    Self::FullHouse
                } else {
                    Self::ThreeKind
                }
            }
            2 => {
                if counts[1].1 == &2 {
                    Self::TowPairs
                } else {
                    Self::OnePairs
                }
            }
            1 => Self::HighCard,
            _ => unreachable!()
        }
    }
}