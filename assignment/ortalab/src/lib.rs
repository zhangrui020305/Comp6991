// src/lib.rs

use ortalib::{Chips, Edition, Enhancement, Joker, Mult, Rank, Round, Suit};
use std::collections::HashSet; // 用于 Stage 4 优化

// 声明所有模块
pub mod stage1_hand;
pub mod stage3_jokers;
pub mod stage4_helpers; // <-- 新增 Stage 4

/// 这是 `main.rs` 调用的主计分函数。
pub fn calculate_score(round: &Round) -> (Chips, Mult) {
    
    if round.cards_played.is_empty() {
        return (0.0, 0.0);
    }

    // --- Stage 1: Hand Identification ---
    let (hand, scoring_cards) = stage1_hand::identify_hand(&round.cards_played);

    // --- 初始化所有计分变量 ---
    let (base_chips, base_mult) = hand.hand_value();

    let mut total_chips: Chips = base_chips;
    let mut card_additive_mult: Mult = base_mult; 
    let mut joker_additive_mult: Mult = 0.0; 
    let mut card_multiplicative_mult: Mult = 1.0;
    let mut joker_multiplicative_mult: Mult = 1.0;

    // --- Stage 4: 状态变量 ---
    let mut photograph_triggered = false;
    
    // (优化) 创建一个 Set，用于快速检查哪些小丑是激活的
    let active_jokers: HashSet<Joker> = round.jokers.iter().map(|jc| jc.joker).collect();

    // --- 1. "On Scored" 循环 (遍历计分卡牌) ---
    // (处理 Stage 2 卡牌修饰 + Stage 4 "On Scored" 小丑)
    for card in &scoring_cards {
        
        // (Stage 2) 卡牌点数
        total_chips += card.rank.rank_value();

        // (Stage 2) 卡牌增强 (Enhancement)
        if let Some(enhancement) = card.enhancement {
            match enhancement {
                Enhancement::Bonus => total_chips += 30.0,
                Enhancement::Mult => card_additive_mult += 4.0, 
                Enhancement::Glass => card_multiplicative_mult *= 2.0,
                _ => {} 
            }
        }

        // (Stage 2) 卡牌版本 (Edition)
        if let Some(edition) = card.edition {
            match edition {
                Edition::Foil => total_chips += 50.0,
                Edition::Holographic => card_additive_mult += 10.0, 
                Edition::Polychrome => card_multiplicative_mult *= 1.5,
                _ => {}
            }
        }

        // --- (Stage 4) "On Scored" 小丑逻辑 ---
        
        // 花色小丑 (Greedy, Lusty, Wrathful, Gluttonous)
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
        if active_jokers.contains(&Joker::GluttonousJoker) && (is_wild || card.suit == Suit::Clubs) {
            joker_additive_mult += 3.0;
        }

        // 斐波那契 (Fibonacci)
        if active_jokers.contains(&Joker::Fibonacci) {
            match card.rank {
                Rank::Ace | Rank::Two | Rank::Three | Rank::Five | Rank::Eight => {
                    joker_additive_mult += 8.0;
                }
                _ => {}
            }
        }

        // 奇偶 (Odd/Even)
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

        // 脸牌 (Face cards: K, Q, J)
        // 假设 ortalib 提供了 `is_face()`
        if card.rank.is_face() {
            if active_jokers.contains(&Joker::ScaryFace) {
                total_chips += 30.0;
            }
            if active_jokers.contains(&Joker::SmileyFace) {
                joker_additive_mult += 5.0;
            }
            // 照片 (Photograph) - 只触发一次
            if active_jokers.contains(&Joker::Photograph) && !photograph_triggered {
                joker_multiplicative_mult *= 2.0; // 照片是 Joker 乘法
                photograph_triggered = true;
            }
        }
    } // -- 结束 "On Scored" 循环 --

    
    // --- 2. "On Held" 循环 (遍历手牌) ---
    // (处理 Stage 2 Steel + Stage 4 Baron)
    for card in &round.cards_held_in_hand {
        // (Stage 2) 钢铁 (Steel)
        if let Some(Enhancement::Steel) = card.enhancement {
            card_multiplicative_mult *= 1.5;
        }
        
        // (Stage 4) 男爵 (Baron)
        if active_jokers.contains(&Joker::Baron) && card.rank == Rank::King {
            joker_multiplicative_mult *= 1.5;
        }
    } // -- 结束 "On Held" 循环 --


    // --- 3. "Independent" 逻辑 (在所有循环之后) ---

    // (Stage 3) 预计算条件
    let cards_played = &round.cards_played;
    let has_pair = stage3_jokers::contains_pair(cards_played);
    let has_three_kind = stage3_jokers::contains_three_of_a_kind(cards_played);
    let has_two_pair = stage3_jokers::contains_two_pair(cards_played);
    let has_straight = stage3_jokers::contains_straight(cards_played);
    let has_flush = stage3_jokers::contains_flush(cards_played);
    let joker_count = round.jokers.len() as f64;

    // (Stage 4) 预计算条件
    let blackboard_active = active_jokers.contains(&Joker::Blackboard) 
        && stage4_helpers::check_blackboard(&round.cards_held_in_hand);
        
    let flowerpot_active = active_jokers.contains(&Joker::FlowerPot) 
        && stage4_helpers::check_flower_pot(&scoring_cards);
        
    let raised_fist_mult = if active_jokers.contains(&Joker::RaisedFist) { 
        stage4_helpers::get_raised_fist_mult(&round.cards_held_in_hand) 
    } else { 0.0 };

    // --- 4. "Independent Joker" 循环 (遍历小丑牌) ---
    // (处理 Stage 3 小丑 + Stage 3 小丑版本 + Stage 4 独立小丑)
    for joker_card in &round.jokers {
        
        // (Stage 3) 处理小丑的 Edition
        if let Some(edition) = joker_card.edition {
            match edition {
                Edition::Foil => total_chips += 50.0,
                Edition::Holographic => joker_additive_mult += 10.0,
                Edition::Polychrome => joker_multiplicative_mult *= 1.5,
                _ => {}
            }
        }

        // (Stage 3 & 4) 处理 "Independent" 小丑的效果
        match joker_card.joker {
            // --- Stage 3 小丑 ---
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

            // --- Stage 4 "Independent" 和 "On Held" 小丑 ---
            Joker::RaisedFist => joker_additive_mult += raised_fist_mult,
            Joker::Blackboard => if blackboard_active { joker_multiplicative_mult *= 3.0; },
            Joker::FlowerPot => if flowerpot_active { joker_multiplicative_mult *= 3.0; },
            
            // Baron (男爵) 在 "On Held" 循环中处理 (因为它按 K 的数量缩放)
            // 所有 "On Scored" 小丑都在 "On Scored" 循环中处理
            _ => {} 
        }
    } // -- 结束 Joker 循环 --

    
    // --- 5. 最终计算 ---
    // (公式不变)
    let final_mult = (
        (card_additive_mult * card_multiplicative_mult) + joker_additive_mult
    ) * joker_multiplicative_mult;
    
    (total_chips, final_mult)
}