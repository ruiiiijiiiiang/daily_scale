use clap::ValueEnum;

pub const NUM_NOTES: usize = 12;

#[derive(Copy, Clone, Debug, ValueEnum, PartialEq, Eq)]
pub enum Note {
    A = 0,
    ASharp = 1,
    B = 2,
    C = 3,
    CSharp = 4,
    D = 5,
    DSharp = 6,
    E = 7,
    F = 8,
    FSharp = 9,
    G = 10,
    GSharp = 11,
}

pub const NOTES: [Note; 12] = [
    Note::A,
    Note::ASharp,
    Note::B,
    Note::C,
    Note::CSharp,
    Note::D,
    Note::DSharp,
    Note::E,
    Note::F,
    Note::FSharp,
    Note::G,
    Note::GSharp,
];

pub const fn note_to_string(note: Note) -> &'static str {
    match note {
        Note::A => "A",
        Note::ASharp => "A#",
        Note::B => "B",
        Note::C => "C",
        Note::CSharp => "C#",
        Note::D => "D",
        Note::DSharp => "D#",
        Note::E => "E",
        Note::F => "F",
        Note::FSharp => "F#",
        Note::G => "G",
        Note::GSharp => "G#",
    }
}
