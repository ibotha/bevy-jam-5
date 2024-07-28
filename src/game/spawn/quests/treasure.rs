use std::fmt::Display;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Item {
    MonkeyPaw,
    Cannon,
    Gold,
    SirensCoveMap,
    SirensScale,
    NorthernSeaMap,
    SirenKiller,
    Journal,
    GreekFire,
    SirenChild,
}

impl Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Item::MonkeyPaw => "Monkey's Paw",
            Item::Cannon => "Cannon",
            Item::Gold => "Gold Coin",
            Item::SirensCoveMap => "Siren Cove Map",
            Item::NorthernSeaMap => "Northern Sea Map",
            Item::SirensScale => "Siren Scale",
            Item::SirenKiller => "Siren Killer",
            Item::Journal => "Siren Killer's Journal",
            Item::GreekFire => "Greek Fire",
            Item::SirenChild => "Siren Child",
        })
    }
}
