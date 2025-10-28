use itertools::Itertools;
use ortalib::{Card, Enhancement, PokerHand, Rank, Suit};
use std::collections::HashMap;

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
    if sorted_ranks.len() != 5 {
        return false;
    }

    if sorted_ranks == [2, 3, 4, 5, 14] {
        return true;
    }

    sorted_ranks.windows(2).all(|w| w[1] == w[0] + 1)
}

/// identify poker hand
///
/// # Argument
/// cards played
///
/// # Return
/// PokerHand and cards that contribute to score (in a tuple)
pub fn identify_hand(cards: &[Card]) -> (PokerHand, Vec<Card>) {
    if cards.len() != 5 {
        let scoring_cards = cards
            .iter()
            .max_by_key(|c| get_rank_ordinal(c.rank))
            .cloned()
            .map_or_else(Vec::new, |c| vec![c]);
        return (PokerHand::HighCard, scoring_cards);
    }

    let rank_counts: HashMap<Rank, usize> = cards.iter().map(|c| c.rank).counts();

    let mut counts: Vec<usize> = rank_counts.values().cloned().collect();
    counts.sort_unstable_by(|a, b| b.cmp(a));

    let mut sorted_ordinals: Vec<u8> = cards.iter().map(|c| get_rank_ordinal(c.rank)).collect();
    sorted_ordinals.sort_unstable();

    let is_straight = is_straight(&sorted_ordinals);

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

    let is_flush = non_wild_suits.is_empty() || non_wild_suits.iter().all_equal();

    // "Illegal" Hands (FlushFive, FlushHouse)
    if is_flush {
        if counts == [5] {
            return (PokerHand::FlushFive, cards.to_vec());
        }
        if counts == [3, 2] {
            return (PokerHand::FlushHouse, cards.to_vec());
        }
    }

    // Straight Flush
    if is_straight && is_flush {
        return (PokerHand::StraightFlush, cards.to_vec());
    }

    // Five of a Kind
    if counts == [5] {
        return (PokerHand::FiveOfAKind, cards.to_vec());
    }

    // Four of a Kind
    if counts == [4, 1] {
        let (rank, _) = rank_counts.iter().find(|&(_, &c)| c == 4).unwrap();
        let scoring_cards = cards.iter().filter(|c| c.rank == *rank).cloned().collect();
        return (PokerHand::FourOfAKind, scoring_cards);
    }

    // Full House
    if counts == [3, 2] {
        return (PokerHand::FullHouse, cards.to_vec());
    }

    // Flush
    if is_flush {
        return (PokerHand::Flush, cards.to_vec());
    }

    // Straight
    if is_straight {
        return (PokerHand::Straight, cards.to_vec());
    }

    // Three of a Kind
    if counts == [3, 1, 1] {
        let (rank, _) = rank_counts.iter().find(|&(_, &c)| c == 3).unwrap();
        let scoring_cards = cards.iter().filter(|c| c.rank == *rank).cloned().collect();
        return (PokerHand::ThreeOfAKind, scoring_cards);
    }

    // Two Pair
    if counts == [2, 2, 1] {
        let pair_ranks: Vec<Rank> = rank_counts
            .iter()
            .filter_map(|(&r, &c)| if c == 2 { Some(r) } else { None })
            .collect();
        let scoring_cards = cards
            .iter()
            .filter(|c| pair_ranks.contains(&c.rank))
            .cloned()
            .collect();
        return (PokerHand::TwoPair, scoring_cards);
    }

    // Pair
    if counts == [2, 1, 1, 1] {
        let (rank, _) = rank_counts.iter().find(|&(_, &c)| c == 2).unwrap();
        let scoring_cards = cards.iter().filter(|c| c.rank == *rank).cloned().collect();
        return (PokerHand::Pair, scoring_cards);
    }

    // High Card
    let high_rank_ordinal = sorted_ordinals.last().unwrap();
    let scoring_card = *cards
        .iter()
        .find(|c| get_rank_ordinal(c.rank) == *high_rank_ordinal)
        .unwrap();
    (PokerHand::HighCard, vec![scoring_card])
}
