// src/lib.rs

// 1. 在 use 语句中加入 Edition 和 Enhancement
use ortalib::{Chips, Edition, Enhancement, Mult, Round};

pub mod stage1_hand;

/// 这是 `main.rs` 调用的主计分函数。
///
/// (Stage 1 的实现被 Stage 2 的逻辑替换)
pub fn calculate_score(round: &Round) -> (Chips, Mult) {
    if round.cards_played.is_empty() {
        return (0.0, 0.0); // (Chips, Mult)
    }

    // --- Stage 1: Hand Identification (现在支持 Wild card) ---
    // `scoring_cards` 是 *构成* 牌型的牌 (例如 "三条" 里的 3 张)
    // 但对于 顺子, 同花, 满堂红 等, 它是所有 5 张牌。
    let (hand, scoring_cards) = stage1_hand::identify_hand(&round.cards_played);

    // --- Stage 2: Scoring Logic ---
    
    // (Step 1) 获取牌型的基础分
    let (base_chips, base_mult) = hand.hand_value();

    // 初始化计分变量
    let mut total_chips: Chips = base_chips;
    let mut additive_mult: Mult = base_mult;      // 加法倍率 (e.g., +4)
    let mut multiplicative_mult: Mult = 1.0;  // 乘法倍率 (e.g., x1.5)

    // --- (Step 2.1, 2.2, 2.3, 3.1) 遍历所有 *计分卡牌* ---
    // (注意: `scoring_cards` 是 `Vec<Card>`，我们需要迭代它的引用)
    for card in &scoring_cards {
        
        // (Step 2.1) 加上卡牌的点数筹码
        total_chips += card.rank.rank_value();

        // (Step 2.2, 2.3, 3.1) 处理卡牌的 *Enhancement* (增强)
        if let Some(enhancement) = card.enhancement {
            match enhancement {
                Enhancement::Bonus => total_chips += 30.0,     // (Step 2.2)
                Enhancement::Mult => additive_mult += 4.0,      // (Step 2.3)
                Enhancement::Glass => multiplicative_mult *= 2.0,   // (Step 3.1)
                // Steel(钢铁) 和 Wild(万能) 在 *打出时* 不影响计分
                _ => {} 
            }
        }

        // (Step 2.2, 2.3, 3.1) 处理卡牌的 *Edition* (版本)
        if let Some(edition) = card.edition {
            match edition {
                Edition::Foil => total_chips += 50.0,           // (Step 2.2)
                Edition::Holographic => additive_mult += 10.0, // (Step 2.3)
                Edition::Polychrome => multiplicative_mult *= 1.5, // (Step 3.1)
                _ => {}
            }
        }
    }

    // --- (Step 3.1) 遍历所有 *手牌* (Held Cards) ---
    // 专门检查 Steel(钢铁) 卡
    for card in &round.cards_held_in_hand {
        if let Some(Enhancement::Steel) = card.enhancement {
            multiplicative_mult *= 1.5; // (Step 3.1)
        }
        // 注意: 手牌的 Edition (Foil, Holo, Poly) 不计分
    }
    
    // (Joker 逻辑将在 Stage 3 添加到这里)

    // 最终计算: (总筹码) * (总加法倍率) * (总乘法倍率)
    let final_mult = additive_mult * multiplicative_mult;
    
    (total_chips, final_mult)
}