// src/stage4_helpers.rs

use ortalib::{Card, Enhancement, Rank, Suit};
use std::collections::HashSet;

// 帮助函数 (从 stage1 复制) 来获取点数
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

/// 检查 Blackboard (黑板)
/// 规则：所有手牌都是 ♠ 或 ♣ (或 Wild)，或者没有手牌
pub fn check_blackboard(cards: &[Card]) -> bool {
    if cards.is_empty() {
        return true;
    }
    cards.iter().all(|card| {
        if card.enhancement == Some(Enhancement::Wild) {
            return true; // Wild 算作黑色
        }
        matches!(card.suit, Suit::Spades | Suit::Clubs)
    })
}

/// 检查 Flower Pot (花盆)
/// 规则：*计分牌* 中包含所有四种花色
/// (Wild 卡可以填补缺失的花色)
pub fn check_flower_pot(cards: &[Card]) -> bool {
    if cards.len() < 4 {
        return false;
    }

    let mut suits_present: HashSet<Suit> = cards
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

    // 集合中的花色 + Wild 卡的数量 >= 4
    suits_present.len() + wild_count >= 4
}

/// 计算 Raised Fist (举起的拳头) 的加成
/// 规则：手牌中最低点数的牌 (最右边) 的点数 * 2
pub fn get_raised_fist_mult(cards: &[Card]) -> f64 {
    if cards.is_empty() {
        return 0.0;
    }

    // 1. 找到最低的 ordinal
    let lowest_ordinal = match cards.iter().map(|c| get_rank_ordinal(c.rank)).min() {
        Some(o) => o,
        None => return 0.0, // 没有牌
    };

    // 2. 从右边开始，找到第一张匹配该 ordinal 的牌
    let rightmost_lowest_card = cards
        .iter()
        .rev()
        .find(|c| get_rank_ordinal(c.rank) == lowest_ordinal);

    // 3. 返回该牌的点数 * 2
    if let Some(card) = rightmost_lowest_card {
        card.rank.rank_value() * 2.0
    } else {
        0.0
    }
}
