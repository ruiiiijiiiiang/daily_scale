use clap::ValueEnum;

#[derive(Copy, Clone, Debug, ValueEnum)]
pub enum Scale {
    Major,
    HarmonicMinor,
    MelodicMinor,
    NaturalMinor,
    PentatonicMajor,
    PentatonicMinor,
    PentatonicBlues,
    PentatonicNeutral,
    WholeDiminished,
    HalfDiminished,
    Ionian,
    Dorian,
    Phrygian,
    Lydian,
    Mixolydian,
    Aeolian,
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

    pub fn to_str(self) -> &'static str {
        match self {
            Scale::Major => "Major",
            Scale::HarmonicMinor => "Harmonic Minor",
            Scale::MelodicMinor => "Melodic Minor",
            Scale::NaturalMinor => "Natural Minor",
            Scale::PentatonicMajor => "Pentatonic Major",
            Scale::PentatonicMinor => "Pentatonic Minor",
            Scale::PentatonicBlues => "Pentatonic Blues",
            Scale::PentatonicNeutral => "Pentatonic Neutral",
            Scale::WholeDiminished => "Whole Diminished",
            Scale::HalfDiminished => "Half Diminished",
            Scale::Ionian => "Ionian",
            Scale::Dorian => "Dorian",
            Scale::Phrygian => "Phrygian",
            Scale::Lydian => "Lydian",
            Scale::Mixolydian => "Mixolydian",
            Scale::Aeolian => "Aeolian",
            Scale::Locrian => "Locrian",
        }
    }
}

pub const SCALES: [Scale; 17] = [
    Scale::Major,
    Scale::HarmonicMinor,
    Scale::MelodicMinor,
    Scale::NaturalMinor,
    Scale::PentatonicMajor,
    Scale::PentatonicMinor,
    Scale::PentatonicBlues,
    Scale::PentatonicNeutral,
    Scale::WholeDiminished,
    Scale::HalfDiminished,
    Scale::Ionian,
    Scale::Dorian,
    Scale::Phrygian,
    Scale::Lydian,
    Scale::Mixolydian,
    Scale::Aeolian,
    Scale::Locrian,
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_steps() {
        assert_eq!(Scale::HarmonicMinor.get_steps(), &[0, 2, 3, 5, 7, 8, 11]);
        assert_eq!(Scale::Phrygian.get_steps(), &[0, 1, 3, 5, 7, 8, 10]);
    }

    #[test]
    fn test_to_string() {
        assert_eq!(Scale::PentatonicBlues.to_str(), "Pentatonic Blues");
        assert_eq!(Scale::Dorian.to_str(), "Dorian");
    }
}
