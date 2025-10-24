use ortalib::{Chips, Mult, Round};

pub mod stage1_hand;

pub fn calculate_score(round: &Round) -> (Chips, Mult) {
    if round.cards_played.is_empty() {
        return (0.0, 0.0); // (Chips, Mult)
    }

    // 1. 识别牌型和哪些卡牌计分
    let (hand, scoring_cards) = stage1_hand::identify_hand(&round.cards_played);

    // 2. 获取牌型的基础 Chips 和 Mult
    //    (e.g., ThreeOfAKind -> (30.0, 3.0))
    let (mut total_chips, total_mult) = hand.hand_value();

    // 3. (Step 2.1) 将计分卡牌的 rank_value 加到 total_chips
    //    (e.g., 10, 10, 10 -> +10.0, +10.0, +10.0)
    for card in scoring_cards {
        total_chips += card.rank.rank_value();
    }

    // 4. 返回 Stage 1 的最终 Chips 和 Mult
    (total_chips, total_mult)
}