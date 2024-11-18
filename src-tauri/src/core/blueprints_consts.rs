#![allow(dead_code)]

/// 品质枚举
#[derive(Debug)]
pub enum QualityType {
    NORMAL,
    UNCOMMON,
    RARE,
    EPIC,
    LEGENDARY,
}

impl QualityType {
    pub fn as_str(&self) -> &'static str {
        match self {
            QualityType::NORMAL => "normal",
            QualityType::UNCOMMON => "uncommon",
            QualityType::RARE => "rare",
            QualityType::EPIC => "epic",
            QualityType::LEGENDARY => "legendary",
        }
    }
}

/// 方向枚举
#[allow(non_camel_case_types)]
pub enum DirectionType {
    NORTH,       // 北
    NORTH_EAST,  // 东北
    EAST,        // 东
    SOUTH_EAST,  // 东南
    SOUTH,       // 南
    SOUTH_WEST,  // 西南
    WEST,        // 西
    NORTH_WEST,  // 西北
}

impl DirectionType {
    pub fn value(&self) -> i64 {
        match self {
            DirectionType::NORTH => 0,
            DirectionType::NORTH_EAST => 2,
            DirectionType::EAST => 4,
            DirectionType::SOUTH_EAST => 6,
            DirectionType::SOUTH => 8,
            DirectionType::SOUTH_WEST => 10,
            DirectionType::WEST => 12,
            DirectionType::NORTH_WEST => 14,
        }
    }
}
