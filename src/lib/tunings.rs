use super::notes::Note;
use clap::ValueEnum;

// TODO: Add more tunings
#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum Tuning {
    StandardE6,
    DropD6,
    StandardB7,
    DropA7,
}

pub const fn tuning_to_string(tuning: Tuning) -> &'static str {
    match tuning {
        Tuning::StandardE6 => "Standard E",
        Tuning::DropD6 => "Drop D",
        Tuning::StandardB7 => "Standard B",
        Tuning::DropA7 => "Drop A",
    }
}

pub const fn get_notes_by_tuning(tuning: Tuning) -> &'static [Note] {
    match tuning {
        Tuning::StandardE6 => &[Note::E, Note::A, Note::D, Note::G, Note::B, Note::E],
        Tuning::DropD6 => &[Note::D, Note::A, Note::D, Note::G, Note::B, Note::E],
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
    }
}
