// src/stage3_jokers.rs

use ortalib::{Card, Enhancement, Rank, Suit};
use itertools::Itertools;
use std::collections::HashMap;

// --- 内部帮助函数 (从 stage1_hand.rs 复制而来) ---
// (这些是检查 'Straight' 和 'Flush' 所必需的)

fn get_rank_ordinal(rank: Rank) -> u8 {
    match rank {
        Rank::Ace => 14, Rank::King => 13, Rank::Queen => 12, Rank::Jack => 11,
        Rank::Ten => 10, Rank::Nine => 9, Rank::Eight => 8, Rank::Seven => 7,
        Rank::Six => 6, Rank::Five => 5, Rank::Four => 4, Rank::Three => 3,
        Rank::Two => 2,
    }
}

fn is_straight(sorted_ranks: &[u8]) -> bool {
    if sorted_ranks.len() < 5 { return false; } // Joker 检查可以用于少于5张牌的情况
    
    // 检查 A-2-3-4-5
    if sorted_ranks.contains(&2) &&
       sorted_ranks.contains(&3) &&
       sorted_ranks.contains(&4) &&
       sorted_ranks.contains(&5) &&
       sorted_ranks.contains(&14) {
        return true;
    }
    
    // 检查普通顺子
    // 我们只关心是否存在5张连续的牌
    for window in sorted_ranks.windows(5) {
        if window.windows(2).all(|w| w[1] == w[0] + 1) {
            return true;
        }
    }
    false
}

// 检查同花 (已更新以支持 Wild Card)
fn check_flush(cards: &[Card]) -> bool {
    let non_wild_suits: Vec<Suit> = cards
        .iter()
        .filter_map(|c| {
            if c.enhancement == Some(Enhancement::Wild) { None } else { Some(c.suit) }
        })
        .collect();

    // 如果所有非万能牌的花色都一样，或者所有牌都是万能牌，则为同花
    (non_wild_suits.is_empty() || non_wild_suits.iter().all_equal()) && !cards.is_empty()
}

// --- 公共 API (给 lib.rs 使用) ---

/// 检查牌组是否包含 "一对" (Pair)
/// 规则：三条、四条、五条都算作 "包含一对"
pub fn contains_pair(cards: &[Card]) -> bool {
    let rank_counts = cards.iter().map(|c| c.rank).counts();
    rank_counts.values().any(|&count| count >= 2)
}

/// 检查牌组是否包含 "三条" (Three of a Kind)
/// 规则：四条、五条都算作 "包含三条"
pub fn contains_three_of_a_kind(cards: &[Card]) -> bool {
    let rank_counts = cards.iter().map(|c| c.rank).counts();
    rank_counts.values().any(|&count| count >= 3)
}

/// 检查牌组是否包含 "两对" (Two Pair)
/// 规则：必须是两个*不同*的点数。四条和五条 *不算* "两对"。
pub fn contains_two_pair(cards: &[Card]) -> bool {
    let rank_counts = cards.iter().map(|c| c.rank).counts();
    // 计算有多少个点数至少是 "一对"
    let pair_groups = rank_counts.values().filter(|&&count| count >= 2).count();
    pair_groups >= 2
}

/// 检查牌组是否包含 "顺子" (Straight)
pub fn contains_straight(cards: &[Card]) -> bool {
    if cards.len() < 5 { return false; }
    let mut sorted_ordinals: Vec<u8> = cards
        .iter()
        .map(|c| get_rank_ordinal(c.rank))
        .dedup() // 移除重复的点数 (例如 [10, 10, 9, 8, 7] 也是顺子)
        .collect();
    sorted_ordinals.sort_unstable();
    is_straight(&sorted_ordinals)
}

/// 检查牌组是否包含 "同花" (Flush)
pub fn contains_flush(cards: &[Card]) -> bool {
    if cards.len() < 5 { return false; }
    check_flush(cards)
}