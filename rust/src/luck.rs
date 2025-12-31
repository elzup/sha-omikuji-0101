use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum Rank {
    Excellent,
    Good,
    Normal,
    Bad,
    Terrible,
}

impl Rank {
    pub fn from_score(score: u8) -> Self {
        match score {
            90..=100 => Rank::Excellent,
            70..=89 => Rank::Good,
            40..=69 => Rank::Normal,
            10..=39 => Rank::Bad,
            _ => Rank::Terrible,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Rank::Excellent => "Excellent",
            Rank::Good => "Good",
            Rank::Normal => "Normal",
            Rank::Bad => "Bad",
            Rank::Terrible => "Terrible",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum LuckType {
    Life,
    Health,
    Wealth,
    Career,
    Love,
    Marriage,
    Family,
    Friendship,
    Study,
    Challenge,
    Opportunity,
    Motivation,
    Debug,
    WiFi,
    Windfall,
    Chaos,
}

impl LuckType {
    pub const ALL: [LuckType; 16] = [
        LuckType::Life,
        LuckType::Health,
        LuckType::Wealth,
        LuckType::Career,
        LuckType::Love,
        LuckType::Marriage,
        LuckType::Family,
        LuckType::Friendship,
        LuckType::Study,
        LuckType::Challenge,
        LuckType::Opportunity,
        LuckType::Motivation,
        LuckType::Debug,
        LuckType::WiFi,
        LuckType::Windfall,
        LuckType::Chaos,
    ];

    pub fn name(&self) -> &'static str {
        match self {
            LuckType::Life => "Life Luck",
            LuckType::Health => "Health Luck",
            LuckType::Wealth => "Wealth Luck",
            LuckType::Career => "Career Luck",
            LuckType::Love => "Love Luck",
            LuckType::Marriage => "Marriage Luck",
            LuckType::Family => "Family Luck",
            LuckType::Friendship => "Friendship Luck",
            LuckType::Study => "Study Luck",
            LuckType::Challenge => "Challenge Luck",
            LuckType::Opportunity => "Opportunity Luck",
            LuckType::Motivation => "Motivation Luck",
            LuckType::Debug => "Debug Luck",
            LuckType::WiFi => "WiFi Luck",
            LuckType::Windfall => "Windfall Luck",
            LuckType::Chaos => "Chaos Luck",
        }
    }

    pub fn index(&self) -> usize {
        LuckType::ALL.iter().position(|&t| t == *self).unwrap()
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct LuckScore {
    pub luck_type: LuckType,
    pub raw_value: u8,
    pub score: u8,
    pub rank: Rank,
    pub active: bool,
}

impl LuckScore {
    pub fn new(luck_type: LuckType, raw_value: u8, active: bool) -> Self {
        let score = ((raw_value as u32) * 100 / 255) as u8;
        let rank = Rank::from_score(score);
        Self {
            luck_type,
            raw_value,
            score,
            rank,
            active,
        }
    }
}

pub fn calculate_luck_scores(scores: &[u8; 16], flags: u64) -> Vec<LuckScore> {
    LuckType::ALL
        .iter()
        .enumerate()
        .map(|(i, &luck_type)| {
            let active = (flags >> i) & 1 == 1;
            LuckScore::new(luck_type, scores[i], active)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rank_from_score() {
        assert_eq!(Rank::from_score(100), Rank::Excellent);
        assert_eq!(Rank::from_score(90), Rank::Excellent);
        assert_eq!(Rank::from_score(89), Rank::Good);
        assert_eq!(Rank::from_score(70), Rank::Good);
        assert_eq!(Rank::from_score(69), Rank::Normal);
        assert_eq!(Rank::from_score(40), Rank::Normal);
        assert_eq!(Rank::from_score(39), Rank::Bad);
        assert_eq!(Rank::from_score(10), Rank::Bad);
        assert_eq!(Rank::from_score(9), Rank::Terrible);
        assert_eq!(Rank::from_score(0), Rank::Terrible);
    }

    #[test]
    fn test_luck_score_calculation() {
        let luck = LuckScore::new(LuckType::Wealth, 255, true);
        assert_eq!(luck.score, 100);

        let luck = LuckScore::new(LuckType::Wealth, 0, true);
        assert_eq!(luck.score, 0);

        let luck = LuckScore::new(LuckType::Wealth, 127, true);
        assert_eq!(luck.score, 49);
    }

    #[test]
    fn test_luck_type_count() {
        assert_eq!(LuckType::ALL.len(), 16);
    }
}
