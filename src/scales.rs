use clap::ValueEnum;
use strum::{Display, EnumIter};

#[derive(Copy, Clone, Debug, ValueEnum, EnumIter, Display)]
pub enum Scale {
    #[strum(to_string = "Major")]
    Major,
    #[strum(to_string = "Harmonic Minor")]
    HarmonicMinor,
    #[strum(to_string = "Melodic Minor")]
    MelodicMinor,
    #[strum(to_string = "Natural Minor")]
    NaturalMinor,
    #[strum(to_string = "Pentatonic Major")]
    PentatonicMajor,
    #[strum(to_string = "Pentatonic Minor")]
    PentatonicMinor,
    #[strum(to_string = "Pentatonic Blues")]
    PentatonicBlues,
    #[strum(to_string = "Pentatonic Neutral")]
    PentatonicNeutral,
    #[strum(to_string = "Whole Diminished")]
    WholeDiminished,
    #[strum(to_string = "Half Diminished")]
    HalfDiminished,
    #[strum(to_string = "Ionian")]
    Ionian,
    #[strum(to_string = "Dorian")]
    Dorian,
    #[strum(to_string = "Phrygian")]
    Phrygian,
    #[strum(to_string = "Lydian")]
    Lydian,
    #[strum(to_string = "Mixolydian")]
    Mixolydian,
    #[strum(to_string = "Aeolian")]
    Aeolian,
    #[strum(to_string = "Locrian")]
    Locrian,
}

impl Scale {
    pub fn get_steps(&self) -> &'static [usize] {
        match self {
            Scale::Major => &[0, 2, 4, 5, 7, 9, 11],
            Scale::HarmonicMinor => &[0, 2, 3, 5, 7, 8, 11],
            Scale::MelodicMinor => &[0, 2, 3, 5, 7, 9, 11],
            Scale::NaturalMinor => &[0, 2, 3, 5, 7, 8, 10],
            Scale::PentatonicMajor => &[0, 2, 4, 7, 9],
            Scale::PentatonicMinor => &[0, 3, 5, 7, 10],
            Scale::PentatonicBlues => &[0, 3, 5, 6, 7, 10],
            Scale::PentatonicNeutral => &[0, 2, 5, 7, 10],
            Scale::WholeDiminished => &[0, 2, 3, 5, 6, 8, 9, 11],
            Scale::HalfDiminished => &[0, 1, 3, 4, 6, 7, 9, 10],
            Scale::Ionian => &[0, 2, 4, 5, 7, 9, 11],
            Scale::Dorian => &[0, 2, 3, 5, 7, 9, 10],
            Scale::Phrygian => &[0, 1, 3, 5, 7, 8, 10],
            Scale::Lydian => &[0, 2, 4, 6, 7, 9, 11],
            Scale::Mixolydian => &[0, 2, 4, 5, 7, 9, 10],
            Scale::Aeolian => &[0, 2, 3, 5, 7, 8, 10],
            Scale::Locrian => &[0, 1, 3, 5, 6, 8, 10],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_steps() {
        assert_eq!(Scale::HarmonicMinor.get_steps(), &[0, 2, 3, 5, 7, 8, 11]);
        assert_eq!(Scale::Phrygian.get_steps(), &[0, 1, 3, 5, 7, 8, 10]);
    }
}
