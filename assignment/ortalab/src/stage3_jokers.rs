use itertools::Itertools;
use ortalib::{Card, Enhancement, Rank, Suit};

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

fn is_straight(sorted_ranks: &[u8]) -> bool {
    if sorted_ranks.len() < 5 {
        return false;
    }

    if sorted_ranks.contains(&2)
        && sorted_ranks.contains(&3)
        && sorted_ranks.contains(&4)
        && sorted_ranks.contains(&5)
        && sorted_ranks.contains(&14)
    {
        return true;
    }

    for window in sorted_ranks.windows(5) {
        if window.windows(2).all(|w| w[1] == w[0] + 1) {
            return true;
        }
    }
    false
}

fn check_flush(cards: &[Card]) -> bool {
    let non_wild_suits: Vec<Suit> = cards
        .iter()
        .filter_map(|c| {
            if c.enhancement == Some(Enhancement::Wild) {
                None
            } else {
                Some(c.suit)
            }
        })
        .collect();

    (non_wild_suits.is_empty() || non_wild_suits.iter().all_equal()) && !cards.is_empty()
}

/// check if cards played contain pair
/// no matter poker hand
pub fn contains_pair(cards: &[Card]) -> bool {
    let rank_counts = cards.iter().map(|c| c.rank).counts();
    rank_counts.values().any(|&count| count >= 2)
}

/// check if cards played contain three of kind
/// no matter poker hand
pub fn contains_three_of_a_kind(cards: &[Card]) -> bool {
    let rank_counts = cards.iter().map(|c| c.rank).counts();
    rank_counts.values().any(|&count| count >= 3)
}

/// check if cards played contain two pairs
/// four or five of a kind do not count
pub fn contains_two_pair(cards: &[Card]) -> bool {
    let rank_counts = cards.iter().map(|c| c.rank).counts();
    let pair_groups = rank_counts.values().filter(|&&count| count >= 2).count();
    pair_groups >= 2
}

/// check if cards played contain straight
pub fn contains_straight(cards: &[Card]) -> bool {
    if cards.len() < 5 {
        return false;
    }
    let mut sorted_ordinals: Vec<u8> = cards
        .iter()
        .map(|c| get_rank_ordinal(c.rank))
        .dedup()
        .collect();
    sorted_ordinals.sort_unstable();
    is_straight(&sorted_ordinals)
}

/// check if cards played contain flush
pub fn contains_flush(cards: &[Card]) -> bool {
    if cards.len() < 5 {
        return false;
    }
    check_flush(cards)
}
