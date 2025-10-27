// src/stage1_hand.rs

// 导入 ortalib 库中的必要类型
// (已为 Stage 2 更新，加入了 Enhancement)
use ortalib::{Card, Enhancement, PokerHand, Rank, Suit};
// 导入 Itertools 来帮助我们分组
use itertools::Itertools;
use std::collections::HashMap;

/// ------------------------------------------------------------------
/// 关键假设：Rank Enum 的 Ordinal Value
/// ------------------------------------------------------------------
///
/// `Rank::rank_value()` 返回的是 "Chips" (J, Q, K 都是 10.0)，
/// 不能用于判断 "Straight" (顺子)。
///
/// 我们必须假设 `Rank` enum 的变体 (variants) 并为其分配一个
/// 顺序值 (ordinal value) 来用于排序和比较。
///
fn get_rank_ordinal(rank: Rank) -> u8 {
    match rank {
        Rank::Ace => 14, // Ace 默认是高位
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

/// 帮助函数：检查一个已排序的 u8 rank 列表是否构成顺子
fn is_straight(sorted_ranks: &[u8]) -> bool {
    if sorted_ranks.len() != 5 {
        return false;
    }

    // 检查 A-2-3-4-5 (在我们的 ordinal 中是 [2, 3, 4, 5, 14])
    if sorted_ranks == [2, 3, 4, 5, 14] {
        return true;
    }

    // 检查普通顺子 (e.g., [6, 7, 8, 9, 10])
    sorted_ranks.windows(2).all(|w| w[1] == w[0] + 1)
}

/// 核心逻辑：识别牌型和构成牌型的卡牌
///
/// 返回一个元组 (Tuple):
/// 1. `PokerHand`: 识别出的牌型
/// 2. `Vec<Card>`: *只有*那些对计分有贡献的卡牌
///
/// 这个函数被 `lib.rs` 设为 `pub` (公开)。
pub fn identify_hand(cards: &[Card]) -> (PokerHand, Vec<Card>) {
    // Stage 1/2 假定总是 5 张牌
    if cards.len() != 5 {
        // 作为备用，如果不是5张牌，则返回 HighCard
        let scoring_cards = cards
            .iter()
            .max_by_key(|c| get_rank_ordinal(c.rank))
            .cloned()
            .map_or_else(Vec::new, |c| vec![c]);
        return (PokerHand::HighCard, scoring_cards);
    }

    // --- 1. 预计算 ---

    // 按 Rank 分组并计数 (e.g., {Rank::Ten: 3, Rank::Ace: 1, Rank::Two: 1})
    let rank_counts: HashMap<Rank, usize> = cards.iter().map(|c| c.rank).counts();

    // 获取 Rank 计数的列表 (e.g., [3, 1, 1] or [2, 2, 1])
    let mut counts: Vec<usize> = rank_counts.values().cloned().collect();
    counts.sort_unstable_by(|a, b| b.cmp(a)); // 倒序 [3, 1, 1]

    // 获取 Rank 顺序值用于判断顺子
    let mut sorted_ordinals: Vec<u8> = cards.iter().map(|c| get_rank_ordinal(c.rank)).collect();
    sorted_ordinals.sort_unstable(); // [2, 10, 10, 10, 11]

    let is_straight = is_straight(&sorted_ordinals);

    // --- (Stage 2 更新) WILD CARD FLUSH 逻辑 ---
    // 找出所有非 Wild 牌的花色
    let non_wild_suits: Vec<Suit> = cards
        .iter()
        .filter_map(|c| {
            // 假设 card.enhancement 是 Option<Enhancement>
            if c.enhancement == Some(Enhancement::Wild) {
                None // 这是万能牌，忽略它的花色
            } else {
                Some(c.suit) // 这不是万能牌，记录它的花色
            }
        })
        .collect();

    // 如果所有非万能牌的花色都一样 (all_equal)，或者所有牌都是万能牌 (is_empty)，
    // 那么这就是一个 Flush。
    let is_flush = non_wild_suits.is_empty() || non_wild_suits.iter().all_equal();
    // --- 结束 Stage 2 更新 ---

    // --- 2. 牌型识别 (从高到低) ---
    // (这里的逻辑现在会正确地使用上面计算出的 `is_flush`)

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
        // (已修复 'find' 语法: |&(_, &c)|)
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
        // (已修复 'find' 语法: |&(_, &c)|)
        let (rank, _) = rank_counts.iter().find(|&(_, &c)| c == 3).unwrap();
        let scoring_cards = cards.iter().filter(|c| c.rank == *rank).cloned().collect();
        return (PokerHand::ThreeOfAKind, scoring_cards);
    }

    // Two Pair
    if counts == [2, 2, 1] {
        // 找到所有 count 为 2 的 Ranks
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
        // (已修复 'find' 语法: |&(_, &c)|)
        let (rank, _) = rank_counts.iter().find(|&(_, &c)| c == 2).unwrap();
        let scoring_cards = cards.iter().filter(|c| c.rank == *rank).cloned().collect();
        return (PokerHand::Pair, scoring_cards);
    }

    // High Card
    // 找到 ordinal 最高的卡
    let high_rank_ordinal = sorted_ordinals.last().unwrap();
    let scoring_card = cards
        .iter()
        .find(|c| get_rank_ordinal(c.rank) == *high_rank_ordinal)
        .unwrap()
        .clone();
    (PokerHand::HighCard, vec![scoring_card])
}
