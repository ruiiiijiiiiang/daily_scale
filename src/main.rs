use chrono::prelude::*;
use clap::{Parser, ValueEnum};
use rand::prelude::*;
use rand::SeedableRng;

const NUM_NOTES: usize = 12;

const NUM_FRETS: usize = 24;

const FRET_SPAN: usize = 5;

const NUM_THICK_STRINGS: usize = 3;

#[derive(Parser, Debug)]
#[command(name = "daily-scale", version, about = "Have you practiced today?", long_about = None)]
struct Args {
    #[arg(
        value_enum,
        value_delimiter = ',',
        required = false,
        short = 'n',
        long,
        help = "Pool of root notes to choose from"
    )]
    root_notes: Option<Vec<Note>>,

    #[arg(
        value_enum,
        value_delimiter = ',',
        required = false,
        short = 's',
        long,
        help = "Pool of scales to choose from"
    )]
    scales: Option<Vec<Scale>>,

    #[arg(
        value_parser = value_parser!(usize),
        value_delimiter = ',',
        required = false,
        short = 'f',
        long,
        value_parser = |s: &str| {
            let num = s.parse::<usize>().map_err(|_| "Not a valid number")?;
            if num <= (NUM_FRETS - FRET_SPAN + 1) {
                Ok(num)
            } else {
                Err(format!("Number must be <= {}", NUM_FRETS - FRET_SPAN + 1))
            }
        },
        help = "Your choice of frets to start the scale on"
    )]
    starting_frets: Option<Vec<usize>>,
}

#[derive(Copy, Clone, Debug, ValueEnum, PartialEq, Eq)]
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

const NOTES: [Note; 12] = [
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

const fn note_to_string(note: Note) -> &'static str {
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

#[derive(Copy, Clone, Debug, ValueEnum, Hash, Eq, PartialEq)]
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

const SCALES: [Scale; 17] = [
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

const fn get_steps_by_scale(key: Scale) -> &'static [usize] {
    match key {
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

const fn scale_to_string(scale: Scale) -> &'static str {
    match scale {
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

const FRET_LENGTH: [usize; 25] = [
    0, 10, 10, 9, 9, 9, 8, 8, 8, 8, 7, 7, 7, 7, 7, 6, 6, 6, 6, 6, 6, 5, 5, 5, 5,
];

const STANDARD_TUNING: [Note; 6] = [Note::E, Note::A, Note::D, Note::G, Note::B, Note::E];

fn format_note(note: Note, string_char: char) -> String {
    let note_string = String::from(note_to_string(note));
    if note_string.len() == 1 {
        format!("{}{}", note_string, string_char)
    } else {
        note_string
    }
}

fn format_fret_num(fret_num: usize) -> String {
    let fret_num_string = fret_num.to_string();
    if fret_num_string.len() == 1 {
        format!("{} ", fret_num_string)
    } else {
        fret_num_string
    }
}

fn main() {
    let args = Args::parse();

    let num_days_since_epoch = Utc::now().date_naive().num_days_from_ce();
    let seed = num_days_since_epoch.to_string().parse::<u64>().unwrap();
    let mut rng = StdRng::seed_from_u64(seed);

    let root_note = if let Some(ref arg_notes) = args.root_notes {
        arg_notes.choose(&mut rng).unwrap()
    } else {
        NOTES.choose(&mut rng).unwrap()
    };

    let scale = if let Some(ref arg_scales) = args.scales {
        arg_scales.choose(&mut rng).unwrap()
    } else {
        SCALES.choose(&mut rng).unwrap()
    };

    let all_frets: Vec<usize> = (0..=NUM_FRETS - FRET_SPAN).collect();
    let starting_fret = if let Some(ref arg_frets) = args.starting_frets {
        arg_frets.choose(&mut rng).unwrap()
    } else {
        all_frets.choose(&mut rng).unwrap()
    };

    let root_note_index = NOTES.iter().position(|&note| note == *root_note).unwrap();
    let steps = get_steps_by_scale(*scale);
    let notes_in_scale = steps
        .iter()
        .map(|step| {
            let note_index = (root_note_index + step) % NUM_NOTES;
            NOTES[note_index]
        })
        .collect::<Vec<Note>>();

    let mut fret_board: Vec<String> = Vec::new();
    for (string_counter, string) in STANDARD_TUNING.iter().enumerate() {
        let string_char = if string_counter < NUM_THICK_STRINGS {
            '='
        } else {
            '-'
        };
        let mut fret_board_string: String = String::new();
        let empty_string_note_index = NOTES.iter().position(|&note| note == *string).unwrap();
        let notes_in_string = (0..=NUM_NOTES)
            .map(|fret| {
                let note_index = (empty_string_note_index + fret) % NUM_NOTES;
                NOTES[note_index]
            })
            .collect::<Vec<Note>>();
        for fret in *starting_fret..(*starting_fret + FRET_SPAN) {
            let note = notes_in_string[fret % NUM_NOTES];
            if fret == 0 {
                if notes_in_scale.contains(&note) {
                    fret_board_string.push_str(format_note(note, string_char).as_str());
                } else {
                    fret_board_string.push(string_char);
                    fret_board_string.push(string_char);
                }
            } else {
                fret_board_string.push('|');
                let fret_length = FRET_LENGTH[fret];
                if notes_in_scale.contains(&note) {
                    let fret_length_odd = fret_length % 2 != 0;
                    let first_half_fret_length =
                        fret_length / 2 - if fret_length_odd { 0 } else { 1 };
                    let second_half_fret_length = fret_length / 2 - 1;
                    for _ in 0..first_half_fret_length {
                        fret_board_string.push(string_char);
                    }
                    fret_board_string.push_str(format_note(note, string_char).as_str());
                    for _ in 0..second_half_fret_length {
                        fret_board_string.push(string_char);
                    }
                } else {
                    for _ in 0..fret_length {
                        fret_board_string.push(string_char);
                    }
                }
            }
        }
        fret_board_string.push('|');
        fret_board.insert(0, fret_board_string);
    }
    let mut fret_num_string: String = String::new();
    for fret in *starting_fret..(*starting_fret + FRET_SPAN) {
        if fret == 0 {
            fret_num_string.push(' ');
            fret_num_string.push(' ');
        } else {
            fret_num_string.push('|');
            let fret_length = FRET_LENGTH[fret];
            let fret_length_odd = fret_length % 2 != 0;
            let first_half_fret_length = fret_length / 2 - if fret_length_odd { 0 } else { 1 };
            let second_half_fret_length = fret_length / 2 - 1;
            for _ in 0..first_half_fret_length {
                fret_num_string.push(' ');
            }
            fret_num_string.push_str(format_fret_num(fret).as_str());
            for _ in 0..second_half_fret_length {
                fret_num_string.push(' ');
            }
        }
    }
    fret_num_string.push('|');
    fret_board.push(fret_num_string);

    for string in fret_board {
        println!("{}", string);
    }

    println!(
        "Here's the scale of the day: {} {} starting at fret {}",
        note_to_string(*root_note),
        scale_to_string(*scale),
        starting_fret
    );

    println!(
        "The notes in this scale are: {}",
        notes_in_scale
            .iter()
            .map(|note| note_to_string(*note))
            .collect::<Vec<&str>>()
            .join(", ")
    );
}
