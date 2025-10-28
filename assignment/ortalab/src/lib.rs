use ortalib::{Chips, Edition, Enhancement, Joker, Mult, Rank, Round, Suit};
use std::collections::HashSet;

pub mod stage1_hand;
pub mod stage3_jokers;
pub mod stage4_helpers;

/// the entrance of calculation
/// get the chip and mult for a round
///
/// # Arguments
/// round include cards_played, cards_held_in_hand, jokers
///
/// # Return
/// (Chips, Mult) final chips and mult
pub fn calculate_score(round: &Round) -> (Chips, Mult) {
    if round.cards_played.is_empty() {
        return (0.0, 0.0);
    }

    let (hand, scoring_cards) = stage1_hand::identify_hand(&round.cards_played);

    let (base_chips, base_mult) = hand.hand_value();

    let mut total_chips: Chips = base_chips;
    let mut card_additive_mult: Mult = base_mult;
    let mut joker_additive_mult: Mult = 0.0;
    let mut card_multiplicative_mult: Mult = 1.0;
    let mut joker_multiplicative_mult: Mult = 1.0;

    let mut photograph_triggered = false;

    // create a set to check which joker is activated
    let active_jokers: HashSet<Joker> = round.jokers.iter().map(|jc| jc.joker).collect();

    for card in &scoring_cards {
        total_chips += card.rank.rank_value();

        if let Some(enhancement) = card.enhancement {
            match enhancement {
                Enhancement::Bonus => total_chips += 30.0,
                Enhancement::Mult => card_additive_mult += 4.0,
                Enhancement::Glass => card_multiplicative_mult *= 2.0,
                _ => {}
            }
        }

        if let Some(edition) = card.edition {
            match edition {
                Edition::Foil => total_chips += 50.0,
                Edition::Holographic => card_additive_mult += 10.0,
                Edition::Polychrome => card_multiplicative_mult *= 1.5,
            }
        }

        // Greedy, Lusty, Wrathful, Gluttonous
        let is_wild = card.enhancement == Some(Enhancement::Wild);
        if active_jokers.contains(&Joker::GreedyJoker) && (is_wild || card.suit == Suit::Diamonds) {
            joker_additive_mult += 3.0;
        }
        if active_jokers.contains(&Joker::LustyJoker) && (is_wild || card.suit == Suit::Hearts) {
            joker_additive_mult += 3.0;
        }
        if active_jokers.contains(&Joker::WrathfulJoker) && (is_wild || card.suit == Suit::Spades) {
            joker_additive_mult += 3.0;
        }
        if active_jokers.contains(&Joker::GluttonousJoker) && (is_wild || card.suit == Suit::Clubs)
        {
            joker_additive_mult += 3.0;
        }

        // Fibonacci
        if active_jokers.contains(&Joker::Fibonacci) {
            match card.rank {
                Rank::Ace | Rank::Two | Rank::Three | Rank::Five | Rank::Eight => {
                    joker_additive_mult += 8.0;
                }
                _ => {}
            }
        }

        // Odd/Even
        if active_jokers.contains(&Joker::EvenSteven) {
            match card.rank {
                Rank::Ten | Rank::Eight | Rank::Six | Rank::Four | Rank::Two => {
                    joker_additive_mult += 4.0;
                }
                _ => {}
            }
        }
        if active_jokers.contains(&Joker::OddTodd) {
            match card.rank {
                Rank::Ace | Rank::Nine | Rank::Seven | Rank::Five | Rank::Three => {
                    total_chips += 31.0;
                }
                _ => {}
            }
        }

        // Face cards
        if card.rank.is_face() {
            if active_jokers.contains(&Joker::ScaryFace) {
                total_chips += 30.0;
            }
            if active_jokers.contains(&Joker::SmileyFace) {
                joker_additive_mult += 5.0;
            }
            if active_jokers.contains(&Joker::Photograph) && !photograph_triggered {
                joker_multiplicative_mult *= 2.0;
                photograph_triggered = true;
            }
        }
    }

    for card in &round.cards_held_in_hand {
        // steel
        if let Some(Enhancement::Steel) = card.enhancement {
            card_multiplicative_mult *= 1.5;
        }

        // baron
        if active_jokers.contains(&Joker::Baron) && card.rank == Rank::King {
            card_multiplicative_mult *= 1.5;
        }
    }

    let cards_played = &round.cards_played;
    let has_pair = stage3_jokers::contains_pair(cards_played);
    let has_three_kind = stage3_jokers::contains_three_of_a_kind(cards_played);
    let has_two_pair = stage3_jokers::contains_two_pair(cards_played);
    let has_straight = stage3_jokers::contains_straight(cards_played);
    let has_flush = stage3_jokers::contains_flush(cards_played);
    let joker_count = round.jokers.len() as f64;

    let blackboard_active = active_jokers.contains(&Joker::Blackboard)
        && stage4_helpers::check_blackboard(&round.cards_held_in_hand);

    let flowerpot_active = active_jokers.contains(&Joker::FlowerPot)
        && stage4_helpers::check_flower_pot(&scoring_cards);

    let raised_fist_mult = if active_jokers.contains(&Joker::RaisedFist) {
        stage4_helpers::get_raised_fist_mult(&round.cards_held_in_hand)
    } else {
        0.0
    };

    for joker_card in &round.jokers {
        if let Some(edition) = joker_card.edition {
            match edition {
                Edition::Foil => total_chips += 50.0,
                Edition::Holographic => joker_additive_mult += 10.0,
                Edition::Polychrome => joker_multiplicative_mult *= 1.5,
            }
        }

        match joker_card.joker {
            Joker::Joker => joker_additive_mult += 4.0,
            Joker::JollyJoker => {
                if has_pair {
                    joker_additive_mult += 8.0;
                }
            }
            Joker::ZanyJoker => {
                if has_three_kind {
                    joker_additive_mult += 12.0;
                }
            }
            Joker::MadJoker => {
                if has_two_pair {
                    joker_additive_mult += 10.0;
                }
            }
            Joker::CrazyJoker => {
                if has_straight {
                    joker_additive_mult += 12.0;
                }
            }
            Joker::DrollJoker => {
                if has_flush {
                    joker_additive_mult += 10.0;
                }
            }
            Joker::SlyJoker => {
                if has_pair {
                    total_chips += 50.0;
                }
            }
            Joker::WilyJoker => {
                if has_three_kind {
                    total_chips += 100.0;
                }
            }
            Joker::CleverJoker => {
                if has_two_pair {
                    total_chips += 80.0;
                }
            }
            Joker::DeviousJoker => {
                if has_straight {
                    total_chips += 100.0;
                }
            }
            Joker::CraftyJoker => {
                if has_flush {
                    total_chips += 80.0;
                }
            }
            Joker::AbstractJoker => joker_additive_mult += 3.0 * joker_count,

            Joker::RaisedFist => joker_additive_mult += raised_fist_mult,
            Joker::Blackboard => {
                if blackboard_active {
                    joker_multiplicative_mult *= 3.0;
                }
            }
            Joker::FlowerPot => {
                if flowerpot_active {
                    joker_multiplicative_mult *= 3.0;
                }
            }
            _ => {}
        }
    }

    // final calculation
    let final_mult = ((card_additive_mult * card_multiplicative_mult) + joker_additive_mult)
        * joker_multiplicative_mult;

    (total_chips, final_mult)
}
