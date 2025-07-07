use clap::ValueEnum;
use strum::Display;

use crate::notes::Note;

#[derive(Debug, Clone, Copy, ValueEnum, Display)]
pub enum Tuning {
    #[strum(to_string = "Standard E (6 string)")]
    StandardE6,
    #[strum(to_string = "Open G (6 string)")]
    OpenG6,
    #[strum(to_string = "Open E (6 string)")]
    OpenE6,
    #[strum(to_string = "Open D (6 string)")]
    OpenD6,
    #[strum(to_string = "Open C (6 string)")]
    OpenC6,
    #[strum(to_string = "Open A (6 string)")]
    OpenA6,
    #[strum(to_string = "Drop D (6 string)")]
    DropD6,
    #[strum(to_string = "Standard D (6 string)")]
    StandardD6,
    #[strum(to_string = "Drop C# (6 string)")]
    DropCSharp6,
    #[strum(to_string = "Standard C# (6 string)")]
    StandardCSharp6,
    #[strum(to_string = "Drop C (6 string)")]
    DropC6,
    #[strum(to_string = "Standard C (6 string)")]
    StandardC6,
    #[strum(to_string = "Standard B (7 string)")]
    StandardB7,
    #[strum(to_string = "Drop A (7 string)")]
    DropA7,
    #[strum(to_string = "Standard A (7 string)")]
    StandardA7,
    #[strum(to_string = "All fourths (7 string)")]
    AllFourths7,
}

impl Tuning {
    pub fn get_notes(&self) -> &'static [Note] {
        match self {
            Tuning::StandardE6 => &[Note::E, Note::A, Note::D, Note::G, Note::B, Note::E],
            Tuning::OpenG6 => &[Note::D, Note::G, Note::D, Note::G, Note::B, Note::D],
            Tuning::OpenE6 => &[Note::E, Note::B, Note::E, Note::GSharp, Note::B, Note::E],
            Tuning::OpenD6 => &[Note::D, Note::A, Note::D, Note::FSharp, Note::A, Note::D],
            Tuning::OpenC6 => &[Note::C, Note::G, Note::C, Note::G, Note::C, Note::E],
            Tuning::OpenA6 => &[Note::E, Note::A, Note::E, Note::A, Note::CSharp, Note::E],
            Tuning::DropD6 => &[Note::D, Note::A, Note::D, Note::G, Note::B, Note::E],
            Tuning::StandardD6 => &[Note::D, Note::G, Note::C, Note::F, Note::A, Note::D],
            Tuning::DropCSharp6 => &[
                Note::CSharp,
                Note::GSharp,
                Note::CSharp,
                Note::FSharp,
                Note::ASharp,
                Note::DSharp,
            ],
            Tuning::StandardCSharp6 => &[
                Note::CSharp,
                Note::FSharp,
                Note::CSharp,
                Note::E,
                Note::GSharp,
                Note::CSharp,
            ],
            Tuning::DropC6 => &[Note::C, Note::G, Note::C, Note::F, Note::A, Note::D],
            Tuning::StandardC6 => &[
                Note::C,
                Note::F,
                Note::ASharp,
                Note::DSharp,
                Note::G,
                Note::C,
            ],
            Tuning::StandardB7 => &[
                Note::B,
                Note::E,
                Note::A,
                Note::D,
                Note::G,
                Note::B,
                Note::E,
            ],
            Tuning::DropA7 => &[
                Note::A,
                Note::E,
                Note::A,
                Note::D,
                Note::G,
                Note::B,
                Note::E,
            ],
            Tuning::StandardA7 => &[
                Note::A,
                Note::D,
                Note::G,
                Note::C,
                Note::F,
                Note::A,
                Note::D,
            ],
            Tuning::AllFourths7 => &[
                Note::B,
                Note::E,
                Note::A,
                Note::D,
                Note::G,
                Note::C,
                Note::F,
            ],
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_notes() {
        assert_eq!(
            Tuning::StandardCSharp6.get_notes(),
            &[
                Note::CSharp,
                Note::FSharp,
                Note::CSharp,
                Note::E,
                Note::GSharp,
                Note::CSharp,
            ]
        );
        assert_eq!(
            Tuning::DropA7.get_notes(),
            &[
                Note::A,
                Note::E,
                Note::A,
                Note::D,
                Note::G,
                Note::B,
                Note::E
            ]
        );
    }
}
