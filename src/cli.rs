use chrono::{Datelike, Utc};
use clap::Parser;
use colored::Colorize;
use rand::{rng, rngs::StdRng, seq::IndexedRandom, Rng, SeedableRng};
use strum::IntoEnumIterator;

use crate::{
    fret_board::{FRET_SPAN, NUM_FRETS},
    notes::{get_flat_accidentals, Accidental, Note, NUM_NOTES},
    scales::Scale,
    tunings::Tuning,
};

pub struct Format {
    pub flat: bool,
    pub colored: bool,
}

pub struct Params {
    pub tuning: Tuning,
    pub root_note: Note,
    pub scale: Scale,
    pub starting_fret: usize,
    pub notes_in_scale: Vec<(Note, usize)>,
    pub format: Format,
}

pub fn get_params() -> Params {
    let args = Args::parse();

    if args.full_randomness {
        get_params_impl(&mut rng(), args)
    } else {
        let seed = Utc::now().date_naive().num_days_from_ce() as u64;
        get_params_impl(&mut StdRng::seed_from_u64(seed), args)
    }
}

fn get_params_impl<R: Rng + ?Sized>(rng: &mut R, args: Args) -> Params {
    let Args {
        tuning,
        root_notes,
        scales,
        starting_frets,
        uncolored,
        ..
    } = args;

    let notes = Note::iter().collect::<Vec<Note>>();
    let flat_accidentals = get_flat_accidentals();

    let tuning = tuning.unwrap();

    let mut flat = false;
    let root_note = if let Some(ref arg_notes) = root_notes {
        let arg_note = arg_notes.choose(rng).unwrap();
        if flat_accidentals.contains(arg_note) {
            flat = true
        }
        arg_note.to_note()
    } else {
        notes.choose(rng).copied().unwrap()
    };
    let format = Format {
        flat,
        colored: !uncolored,
    };

    let scale = if let Some(ref arg_scales) = scales {
        arg_scales.choose(rng).copied().unwrap()
    } else {
        Scale::iter()
            .collect::<Vec<Scale>>()
            .choose(rng)
            .copied()
            .unwrap()
    };

    let all_frets: Vec<usize> = (0..=NUM_FRETS - FRET_SPAN).collect();
    let starting_fret = if let Some(ref arg_frets) = starting_frets {
        arg_frets.choose(rng).copied().unwrap()
    } else {
        all_frets.choose(rng).copied().unwrap()
    };

    let root_note_index = notes.iter().position(|&note| note == root_note).unwrap();
    let steps = scale.get_steps();
    let notes_in_scale = steps
        .iter()
        .map(|step| {
            let note_index = (root_note_index + *step) % NUM_NOTES;
            (notes[note_index], *step)
        })
        .collect::<Vec<(Note, usize)>>();

    Params {
        tuning,
        root_note,
        scale,
        starting_fret,
        notes_in_scale,
        format,
    }
}

pub fn print_output(params: Params, fret_board: Vec<String>) {
    let Params {
        tuning,
        root_note,
        scale,
        starting_fret,
        ref format,
        ref notes_in_scale,
        ..
    } = params;
    let Format { flat, colored } = *format;

    for string in fret_board {
        println!("{}", string);
    }

    println!(
        "Here's the scale of the day: {} {} starting at fret {} in {} tuning",
        format_with_color(root_note.to_str(flat), 0, colored),
        scale,
        starting_fret,
        tuning,
    );

    println!(
        "The notes in this scale are: {}",
        notes_in_scale
            .iter()
            .map(|(note, step)| format_with_color(note.to_str(flat), *step, colored))
            .collect::<Vec<String>>()
            .join(", ")
    );
}

pub fn format_with_color(note_string: &str, step: usize, colored: bool) -> String {
    if !colored {
        return String::from(note_string);
    }
    match step {
        0 => format!("{}", note_string.green()),
        3 => format!("{}", note_string.red()),
        4 => format!("{}", note_string.red()),
        5 => format!("{}", note_string.cyan()),
        6 => format!("{}", note_string.black()),
        7 => format!("{}", note_string.blue()),
        9 => format!("{}", note_string.magenta()),
        10 => format!("{}", note_string.yellow()),
        11 => format!("{}", note_string.yellow()),
        _ => String::from(note_string),
    }
}

#[derive(Parser, Debug)]
#[command(name = "daily-scale", version, about = "Have you practiced today?", long_about = None)]
struct Args {
    #[arg(
        value_enum,
        required = false,
        short = 't',
        long,
        default_value = "standard-e6",
        help = "Select the tuning you want to play in"
    )]
    tuning: Option<Tuning>,

    #[arg(
        value_enum,
        value_delimiter = ',',
        required = false,
        short = 's',
        long,
        help = "Provide a comma separated list of scales"
    )]
    scales: Option<Vec<Scale>>,

    #[arg(
        value_enum,
        value_delimiter = ',',
        required = false,
        short = 'n',
        long,
        help = "Provide a comma separated list of root notes for the scale"
    )]
    root_notes: Option<Vec<Accidental>>,

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
        help = "Provide a comma separated list of numbers for the starting fret"
    )]
    starting_frets: Option<Vec<usize>>,

    #[arg(
        required = false,
        short = 'r',
        long,
        help = "If enabled, the scale generator will use a fully random seed instead of today's date"
    )]
    full_randomness: bool,

    #[arg(
        required = false,
        short = 'c',
        long,
        help = "If enabled, the output will be in plain text without color"
    )]
    uncolored: bool,
}

