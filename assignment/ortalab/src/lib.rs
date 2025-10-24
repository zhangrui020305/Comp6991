// src/lib.rs

// 1. 导入 Stage 3 需要的 Joker
use ortalib::{Chips, Edition, Enhancement, Joker, Mult, Round};

// 2. 声明所有模块
pub mod stage1_hand;
pub mod stage3_jokers; // <-- 确保这一行存在

/// 这是 `main.rs` 调用的主计分函数。
pub fn calculate_score(round: &Round) -> (Chips, Mult) {
    
    // (如果没打牌，返回 0)
    if round.cards_played.is_empty() {
        return (0.0, 0.0); // (Chips, Mult)
    }

    // --- Stage 1: Hand Identification (来自 stage1_hand) ---
    // (这会返回识别出的牌型和用于计分的卡牌)
    let (hand, scoring_cards) = stage1_hand::identify_hand(&round.cards_played);

    // --- Stage 2: Card Modifiers (基础分) ---
    let (base_chips, base_mult) = hand.hand_value();

    // 初始化计分变量
    let mut total_chips: Chips = base_chips;
    let mut additive_mult: Mult = base_mult;
    let mut multiplicative_mult: Mult = 1.0;

    // (遍历计分卡牌 - Stage 2 逻辑)
    for card in &scoring_cards {
        // (Step 2.1) 加上卡牌的点数筹码
        total_chips += card.rank.rank_value();

        // (Step 2.2, 2.3, 3.1) 处理卡牌的 *Enhancement* (增强)
        if let Some(enhancement) = card.enhancement {
            match enhancement {
                Enhancement::Bonus => total_chips += 30.0,
                Enhancement::Mult => additive_mult += 4.0,
                Enhancement::Glass => multiplicative_mult *= 2.0,
                _ => {} // Steel 和 Wild 在打出时不计分
            }
        }

        // (Step 2.2, 2.3, 3.1) 处理卡牌的 *Edition* (版本)
        if let Some(edition) = card.edition {
            match edition {
                Edition::Foil => total_chips += 50.0,
                Edition::Holographic => additive_mult += 10.0,
                Edition::Polychrome => multiplicative_mult *= 1.5,
                _ => {}
            }
        }
    }

    // (遍历手牌 - Stage 2 逻辑)
    for card in &round.cards_held_in_hand {
        if let Some(Enhancement::Steel) = card.enhancement {
            multiplicative_mult *= 1.5;
        }
    }
    
    // --- (新) Stage 3: Easy Jokers ---

    // 3. 预先计算所有 Joker 条件 (!! 必须在 'for' 循环之前 !!)
    let cards = &round.cards_played;
    let has_pair = stage3_jokers::contains_pair(cards);
    let has_three_kind = stage3_jokers::contains_three_of_a_kind(cards);
    let has_two_pair = stage3_jokers::contains_two_pair(cards);
    let has_straight = stage3_jokers::contains_straight(cards);
    let has_flush = stage3_jokers::contains_flush(cards);
    
    let joker_count = round.jokers.len() as f64; // 用于 Abstract Joker

    // 4. 循环遍历所有激活的小丑牌 (!! 必须在 'let has_pair' 之后 !!)
    for joker_card in &round.jokers {
        // 注意: Stage 3 假设小丑牌的 Edition (Foil, Holo...) 不影响其效果
        match joker_card.joker {
            // 固定倍率
            Joker::Joker => additive_mult += 4.0,
            
            // 条件倍率
            Joker::JollyJoker => if has_pair { additive_mult += 8.0; },
            Joker::ZanyJoker => if has_three_kind { additive_mult += 12.0; },
            Joker::MadJoker => if has_two_pair { additive_mult += 10.0; },
            Joker::CrazyJoker => if has_straight { additive_mult += 12.0; },
            Joker::DrollJoker => if has_flush { additive_mult += 10.0; },

            // 条件筹码
            Joker::SlyJoker => if has_pair { total_chips += 50.0; },
            Joker::WilyJoker => if has_three_kind { total_chips += 100.0; },
            Joker::CleverJoker => if has_two_pair { total_chips += 80.0; },
            Joker::DeviousJoker => if has_straight { total_chips += 100.0; },
            Joker::CraftyJoker => if has_flush { total_chips += 80.0; },

            // 特殊小丑
            Joker::AbstractJoker => additive_mult += 3.0 * joker_count,

            // 忽略 Stage 4/5 的小丑 (例如 BurntJoker, RaisedFist 等)
            _ => {}
        }
    }
    
    // --- 最终计算 ---
    let final_mult = additive_mult * multiplicative_mult;
    
    (total_chips, final_mult)
}