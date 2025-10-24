// src/lib.rs

use ortalib::{Chips, Edition, Enhancement, Joker, Mult, Round};

pub mod stage1_hand;
pub mod stage3_jokers;

/// 这是 `main.rs` 调用的主计分函数。
pub fn calculate_score(round: &Round) -> (Chips, Mult) {
    
    if round.cards_played.is_empty() {
        return (0.0, 0.0);
    }

    // --- Stage 1: Hand Identification ---
    let (hand, scoring_cards) = stage1_hand::identify_hand(&round.cards_played);

    // --- Stage 2 & 3: Scoring ---
    let (base_chips, base_mult) = hand.hand_value();

    // --- (初始化变量) ---
    // (已修复: 拆分了 card_additive_mult 和 joker_additive_mult)
    let mut total_chips: Chips = base_chips;
    let mut card_additive_mult: Mult = base_mult; 
    let mut joker_additive_mult: Mult = 0.0; 
    
    // (修复: 拆分了 card_multiplicative_mult 和 joker_multiplicative_mult)
    let mut card_multiplicative_mult: Mult = 1.0;
    let mut joker_multiplicative_mult: Mult = 1.0;

    // --- (遍历计分卡牌 - Stage 2 逻辑) ---
    for card in &scoring_cards {
        total_chips += card.rank.rank_value();

        // 卡牌增强
        if let Some(enhancement) = card.enhancement {
            match enhancement {
                Enhancement::Bonus => total_chips += 30.0,
                Enhancement::Mult => card_additive_mult += 4.0, 
                // (修复: 加到 card_multiplicative_mult)
                Enhancement::Glass => card_multiplicative_mult *= 2.0,
                _ => {} 
            }
        }

        // 卡牌版本
        if let Some(edition) = card.edition {
            match edition {
                Edition::Foil => total_chips += 50.0,
                Edition::Holographic => card_additive_mult += 10.0, 
                // (修复: 加到 card_multiplicative_mult)
                Edition::Polychrome => card_multiplicative_mult *= 1.5,
                _ => {}
            }
        }
    }

    // --- (遍历手牌 - Stage 2 逻辑) ---
    for card in &round.cards_held_in_hand {
        if let Some(Enhancement::Steel) = card.enhancement {
            // (修复: 加到 card_multiplicative_mult)
            card_multiplicative_mult *= 1.5;
        }
    }
    
    // --- (Stage 3: Easy Jokers) ---
    // (预计算条件)
    let cards = &round.cards_played;
    let has_pair = stage3_jokers::contains_pair(cards);
    let has_three_kind = stage3_jokers::contains_three_of_a_kind(cards);
    let has_two_pair = stage3_jokers::contains_two_pair(cards);
    let has_straight = stage3_jokers::contains_straight(cards);
    let has_flush = stage3_jokers::contains_flush(cards);
    let joker_count = round.jokers.len() as f64;

    // --- (循环遍历小丑牌) ---
    for joker_card in &round.jokers {
        
        // (处理 Joker 的 Edition)
        if let Some(edition) = joker_card.edition {
            match edition {
                Edition::Foil => total_chips += 50.0,
                Edition::Holographic => joker_additive_mult += 10.0,
                // (修复: 加到 joker_multiplicative_mult)
                Edition::Polychrome => joker_multiplicative_mult *= 1.5,
                _ => {}
            }
        }

        // (处理 Joker 自身的效果)
        match joker_card.joker {
            Joker::Joker => joker_additive_mult += 4.0,
            
            Joker::JollyJoker => if has_pair { joker_additive_mult += 8.0; },
            Joker::ZanyJoker => if has_three_kind { joker_additive_mult += 12.0; },
            Joker::MadJoker => if has_two_pair { joker_additive_mult += 10.0; },
            Joker::CrazyJoker => if has_straight { joker_additive_mult += 12.0; },
            Joker::DrollJoker => if has_flush { joker_additive_mult += 10.0; },

            Joker::SlyJoker => if has_pair { total_chips += 50.0; },
            Joker::WilyJoker => if has_three_kind { total_chips += 100.0; },
            Joker::CleverJoker => if has_two_pair { total_chips += 80.0; },
            Joker::DeviousJoker => if has_straight { total_chips += 100.0; },
            Joker::CraftyJoker => if has_flush { total_chips += 80.0; },

            Joker::AbstractJoker => joker_additive_mult += 3.0 * joker_count,

            // 忽略 Stage 4/5 的小丑
            _ => {}
        }
    }
    
    // --- 最终计算 (应用正确的计分公式) ---
    let final_mult = (
        (card_additive_mult * card_multiplicative_mult) + joker_additive_mult
    ) * joker_multiplicative_mult;
    
    (total_chips, final_mult)
}