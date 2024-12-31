use clap::{Parser, ValueEnum};
use once_cell::sync::Lazy;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::HashMap;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

const NUM_NOTES: u8 = 12;

const NUM_FRETS: u8 = 24;

const FRET_SPAN: u8 = 5;

#[derive(Parser, Debug)]
#[command(name = "daily-scale", version, about = "Have you practiced today?", long_about = None)]
struct Args {
    #[arg(value_enum, value_delimiter = ',', required = false, short = 'n', long)]
    root_notes: Option<Vec<Note>>,

    #[arg(
        value_parser = clap::value_parser!(u8).range(0..=(NUM_FRETS - FRET_SPAN) as i64),
        value_delimiter = ',',
        required = false,
        short = 'f',
        long,
    )]
    starting_frets: Option<Vec<u8>>,

    #[arg(value_enum, value_delimiter = ',', required = false, short = 's', long)]
    scales: Option<Vec<Scale>>,
}

#[derive(Copy, Clone, Debug, ValueEnum, EnumIter, PartialEq, Eq)]
#[repr(u8)]
enum Note {
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

impl Note {
    unsafe fn unsafe_get_note(note: u8) -> Self {
        std::mem::transmute(note)
    }

    fn get_index(self) -> u8 {
        self as u8
    }
}

#[derive(Copy, Clone, Debug, ValueEnum, EnumIter, Hash, Eq, PartialEq)]
enum Scale {
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

struct ScaleSteps {
    map: HashMap<Scale, Vec<u8>>,
}

static SCALE_STEPS: Lazy<ScaleSteps> = Lazy::new(|| ScaleSteps {
    map: HashMap::from([
        (Scale::Major, vec![0, 2, 4, 5, 7, 9, 11, 12]),
        (Scale::HarmonicMinor, vec![0, 2, 3, 5, 7, 8, 11, 12]),
        (Scale::MelodicMinor, vec![0, 2, 3, 5, 7, 9, 11, 12]),
        (Scale::NaturalMinor, vec![0, 2, 3, 5, 7, 8, 10, 12]),
        (Scale::PentatonicMajor, vec![0, 2, 4, 7, 9, 12]),
        (Scale::PentatonicMinor, vec![0, 3, 5, 7, 10, 12]),
        (Scale::PentatonicBlues, vec![0, 3, 5, 6, 7, 10, 12]),
        (Scale::PentatonicNeutral, vec![0, 2, 5, 7, 10, 12]),
        (Scale::WholeDiminished, vec![0, 2, 3, 5, 6, 8, 9, 11, 12]),
        (Scale::HalfDiminished, vec![0, 1, 3, 4, 6, 7, 9, 10, 12]),
        (Scale::Ionian, vec![0, 2, 4, 5, 7, 9, 11, 12]),
        (Scale::Dorian, vec![0, 2, 3, 5, 7, 9, 10, 12]),
        (Scale::Phrygian, vec![0, 1, 3, 5, 7, 8, 10, 12]),
        (Scale::Lydian, vec![0, 2, 4, 6, 7, 9, 11, 12]),
        (Scale::Mixolydian, vec![0, 2, 4, 5, 7, 9, 10, 12]),
        (Scale::Aeolian, vec![0, 2, 3, 5, 7, 8, 10, 12]),
        (Scale::Locrian, vec![0, 1, 3, 5, 6, 8, 10, 12]),
    ]),
});

const FRET_LENGTH: [u8; 25] = [
    10, 10, 9, 9, 9, 8, 8, 8, 8, 7, 7, 7, 7, 7, 6, 6, 6, 6, 6, 6, 5, 5, 5, 5, 5,
];

const STANDARD_TUNING: [Note; 6] = [Note::E, Note::A, Note::D, Note::G, Note::B, Note::E];

fn main() {
    let args = Args::parse();
    let mut rng = thread_rng();

    let all_notes: Vec<Note> = Note::iter().collect();
    let root_note = if let Some(ref arg_notes) = args.root_notes {
        arg_notes.choose(&mut rng).unwrap()
    } else {
        all_notes.choose(&mut rng).unwrap()
    };

    let all_scales: Vec<Scale> = Scale::iter().collect();
    let scale = if let Some(ref arg_scales) = args.scales {
        arg_scales.choose(&mut rng).unwrap()
    } else {
        all_scales.choose(&mut rng).unwrap()
    };

    let all_frets: Vec<u8> = (0..=NUM_FRETS - FRET_SPAN).collect();
    let starting_fret = if let Some(ref arg_frets) = args.starting_frets {
        arg_frets.choose(&mut rng).unwrap()
    } else {
        all_frets.choose(&mut rng).unwrap()
    };

    let root_note_index = root_note.get_index();
    let steps = SCALE_STEPS.map.get(scale).unwrap();
    let notes_in_scale = steps
        .iter()
        .map(|step| unsafe {
            let note_index = (root_note_index + step) % NUM_NOTES;
            Note::unsafe_get_note(note_index)
        })
        .collect::<Vec<Note>>();

    let mut fret_board: Vec<String> = Vec::new();
    for string in STANDARD_TUNING {
        let mut fret_board_string: String = String::new();
        let empty_string_note_index = string.get_index();
        let notes_in_string = (0..=NUM_NOTES)
            .map(|fret| unsafe {
                let note_index = (empty_string_note_index + fret) % NUM_NOTES;
                Note::unsafe_get_note(note_index)
            })
            .collect::<Vec<Note>>();
        for fret in *starting_fret..(*starting_fret + FRET_SPAN) {
            fret_board_string.push('|');
            let note = notes_in_string[(fret % NUM_NOTES) as usize];
            if notes_in_scale.contains(&note) {
                let half_fret_length = FRET_LENGTH[fret as usize] / 2;
                for _ in 0..half_fret_length {
                    fret_board_string.push('-');
                }
                fret_board_string.push('O');
                for _ in 0..half_fret_length {
                    fret_board_string.push('-');
                }
            } else {
                for _ in 0..FRET_LENGTH[fret as usize] {
                    fret_board_string.push('-');
                }
            }
        }
        fret_board_string.push('|');
        fret_board.push(fret_board_string);
    }

    let root_note_format = format!("{:?}", root_note);
    let scale_format = format!("{:?}", scale);
    let starting_fret_format = format!("{:?}", starting_fret);
    let notes_in_scale_format = format!("{:?}", notes_in_scale);
    println!(
        "Root note: {} scale: {} starting on fret {}; notes in scale: {}",
        root_note_format, scale_format, starting_fret_format, notes_in_scale_format
    );
    fret_board.reverse();
    for string in fret_board {
        println!("{}", string);
    }
}
