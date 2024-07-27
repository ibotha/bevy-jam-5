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

pub const CAPTAIN: &str = "Cap'n";
pub const CREW1: &str = "Patchy";
pub const CREW2: &str = "Long beard";
pub const CREW3: &str = "Short beard";
