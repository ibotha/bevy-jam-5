use std::fmt::Display;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Item {
    MonkeyPaw,
    Cannon,
    Gold,
}

impl Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Item::MonkeyPaw => "Monkey's Paw",
            Item::Cannon => "Cannon",
            Item::Gold => "Gold Coin",
        })
    }
}
