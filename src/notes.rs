use clap::ValueEnum;
use std::collections::HashSet;
use strum::EnumIter;

pub const NUM_NOTES: usize = 12;

#[derive(Copy, Clone, Debug, ValueEnum, PartialEq, Eq, Hash)]
pub enum Accidental {
    AFlat,
    A,
    ASharp,
    BFlat,
    B,
    C,
    CSharp,
    DFlat,
    D,
    DSharp,
    EFlat,
    E,
    F,
    FSharp,
    GFlat,
    G,
    GSharp,
}

impl Accidental {
    pub fn to_note(self) -> Note {
        match self {
            Accidental::AFlat => Note::GSharp,
            Accidental::A => Note::A,
            Accidental::ASharp => Note::ASharp,
            Accidental::BFlat => Note::ASharp,
            Accidental::B => Note::B,
            Accidental::C => Note::C,
            Accidental::CSharp => Note::CSharp,
            Accidental::DFlat => Note::CSharp,
            Accidental::D => Note::D,
            Accidental::DSharp => Note::DSharp,
            Accidental::EFlat => Note::DSharp,
            Accidental::E => Note::E,
            Accidental::F => Note::F,
            Accidental::FSharp => Note::FSharp,
            Accidental::GFlat => Note::FSharp,
            Accidental::G => Note::G,
            Accidental::GSharp => Note::GSharp,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, EnumIter)]
pub enum Note {
    A,
    ASharp,
    B,
    C,
    CSharp,
    D,
    DSharp,
    E,
    F,
    FSharp,
    G,
    GSharp,
}

impl Note {
    pub fn to_str(self, flat: bool) -> &'static str {
        match self {
            Note::A => "A",
            Note::ASharp => {
                if flat {
                    "Bb"
                } else {
                    "A#"
                }
            }
            Note::B => "B",
            Note::C => "C",
            Note::CSharp => {
                if flat {
                    "Db"
                } else {
                    "C#"
                }
            }
            Note::D => "D",
            Note::DSharp => {
                if flat {
                    "Eb"
                } else {
                    "D#"
                }
            }
            Note::E => "E",
            Note::F => "F",
            Note::FSharp => {
                if flat {
                    "Gb"
                } else {
                    "F#"
                }
            }
            Note::G => "G",
            Note::GSharp => {
                if flat {
                    "Ab"
                } else {
                    "G#"
                }
            }
        }
    }
}

pub fn get_flat_accidentals() -> HashSet<Accidental> {
    [
        Accidental::AFlat,
        Accidental::BFlat,
        Accidental::DFlat,
        Accidental::EFlat,
        Accidental::GFlat,
    ]
    .into_iter()
    .collect::<HashSet<Accidental>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_note() {
        assert_eq!(Accidental::CSharp.to_note(), Note::CSharp);
        assert_eq!(Accidental::AFlat.to_note(), Note::GSharp);
    }

    #[test]
    fn test_to_string() {
        assert_eq!(Note::CSharp.to_str(true), "Db");
        assert_eq!(Note::GSharp.to_str(false), "G#");
    }
}
