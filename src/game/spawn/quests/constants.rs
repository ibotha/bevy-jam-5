#[macro_export]
macro_rules! dialogue {
    ( $name:expr ; $( $x:expr ),* ) => {

            crate::game::spawn::quests::dialogue::Dialogue::new($name)
            $(
                .para($x)
            )*


    };
}

#[macro_export]
macro_rules! captain {
    ($($x:expr),*) => {
        dialogue!(CAPTAIN; $($x),*)
    };
}

#[macro_export]
macro_rules! crew1 {
    ($($x:expr),*) => {
        dialogue!(CREW1; $($x),*)
    };
}

#[macro_export]
macro_rules! crew2 {
    ($($x:expr),*) => {
        dialogue!(CREW2; $($x),*)
    };
}

#[macro_export]
macro_rules! crew3 {
    ($($x:expr),*) => {
        dialogue!(CREW3; $($x),*)
    };
}

#[macro_export]
macro_rules! crew {
    ($($x:expr),*) => {
        dialogue!(CREW; $($x),*)
    };
}

#[macro_export]
macro_rules! dock_worker {
    ($($x:expr),*) => {
        dialogue!(DOCK_WORKER; $($x),*)
    };
}

#[macro_export]
macro_rules! monster_hunter {
    ($($x:expr),*) => {
        dialogue!(MONSTER_HUNTER; $($x),*)
    };
}

#[macro_export]
macro_rules! map_merchant {
    ($($x:expr),*) => {
        dialogue!(MAP_MERCHANT; $($x),*)
    };
}

#[macro_export]
macro_rules! trinket_seller {
    ($($x:expr),*) => {
        dialogue!(TRINKET_SELLER; $($x),*)
    };
}

#[macro_export]
macro_rules! narrator {
    ($($x:expr),*) => {
        dialogue!("Narrator"; $($x),*)
    };
}

#[macro_export]
macro_rules! widow {
    ($($x:expr),*) => {
        dialogue!(WIDOW; $($x),*)
    };
}

#[macro_export]
macro_rules! sirens {
    ($($x:expr),*) => {
        dialogue!(SIRENS; $($x),*)
    };
}
#[macro_export]
macro_rules! siren {
    ($($x:expr),*) => {
        dialogue!(SIREN; $($x),*)
    };
}
#[macro_export]
macro_rules! siren_child {
    ($($x:expr),*) => {
        dialogue!(SIREN_CHILD; $($x),*)
    };
}
#[macro_export]
macro_rules! prisoner {
    ($($x:expr),*) => {
        dialogue!(PRISONER; $($x),*)
    };
}
#[macro_export]
macro_rules! king_triton {
    ($($x:expr),*) => {
        dialogue!(KING_TRITON; $($x),*)
    };
}

pub const CAPTAIN: &str = "Cap'n";
pub const CREW: &str = "Crew";
pub const CREW1: &str = "Patchy";
pub const CREW2: &str = "Long Beard";
pub const CREW3: &str = "Short Beard";
pub const DOCK_WORKER: &str = "Dock Worker";
pub const MAP_MERCHANT: &str = "Mapper Goodwyn";
pub const MONSTER_HUNTER: &str = "Hunter Fluffikins III";
pub const TRINKET_SELLER: &str = "Trinketier Keyir";
pub const WIDOW: &str = "Old Woman";
pub const PRISONER: &str = "Prisoner";
pub const SIRENS: &str = "Sirens";
pub const SIREN: &str = "Siren";
pub const SIREN_CHILD: &str = "Siren Child";
pub const KING_TRITON: &str = "King Triton";
