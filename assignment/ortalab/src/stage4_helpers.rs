use ortalib::{Card, Enhancement, Rank, Suit};
use std::collections::HashSet;

fn get_rank_ordinal(rank: Rank) -> u8 {
    match rank {
        Rank::Ace => 14,
        Rank::King => 13,
        Rank::Queen => 12,
        Rank::Jack => 11,
        Rank::Ten => 10,
        Rank::Nine => 9,
        Rank::Eight => 8,
        Rank::Seven => 7,
        Rank::Six => 6,
        Rank::Five => 5,
        Rank::Four => 4,
        Rank::Three => 3,
        Rank::Two => 2,
    }
}

/// check blackboard joker
/// spades or clubs or no cards
pub fn check_blackboard(cards: &[Card]) -> bool {
    if cards.is_empty() {
        return true;
    }
    cards.iter().all(|card| {
        if card.enhancement == Some(Enhancement::Wild) {
            return true;
        }
        matches!(card.suit, Suit::Spades | Suit::Clubs)
    })
}

/// check flower pot
/// play contains four colour
pub fn check_flower_pot(cards: &[Card]) -> bool {
    if cards.len() < 4 {
        return false;
    }

    let suits_present: HashSet<Suit> = cards
        .iter()
        .filter_map(|c| {
            if c.enhancement != Some(Enhancement::Wild) {
                Some(c.suit)
            } else {
                None
            }
        })
        .collect();

    let wild_count = cards
        .iter()
        .filter(|c| c.enhancement == Some(Enhancement::Wild))
        .count();
    suits_present.len() + wild_count >= 4
}

/// check Raised Fist
/// the lowest card plus 2 mult
pub fn get_raised_fist_mult(cards: &[Card]) -> f64 {
    if cards.is_empty() {
        return 0.0;
    }

    // find the lowest card
    let lowest_ordinal = match cards.iter().map(|c| get_rank_ordinal(c.rank)).min() {
        Some(o) => o,
        None => return 0.0,
    };

    let rightmost_lowest_card = cards
        .iter()
        .rev()
        .find(|c| get_rank_ordinal(c.rank) == lowest_ordinal);

    if let Some(card) = rightmost_lowest_card {
        card.rank.rank_value() * 2.0
    } else {
        0.0
    }
}
