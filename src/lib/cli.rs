use chrono::{Datelike, Utc};
use clap::Parser;
use colored::Colorize;
use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use rand::{RngCore, SeedableRng};

use super::fret_board::{FRET_SPAN, NUM_FRETS};
use super::notes::{
    accidental_to_note, note_to_string, Accidental, Note, FLAT_ACCIDENTALS, NOTES, NUM_NOTES,
};
use super::scales::{get_steps_by_scale, scale_to_string, Scale, SCALES};
use super::tunings::{tuning_to_string, Tuning};

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
    let Args {
        tuning,
        root_notes,
        scales,
        starting_frets,
        full_randomness,
        uncolored,
        ..
    } = Args::parse();

    let mut rng: Box<dyn RngCore> = if full_randomness {
        Box::new(rand::thread_rng())
    } else {
        let seed = Utc::now().date_naive().num_days_from_ce() as u64;
        Box::new(StdRng::seed_from_u64(seed))
    };

    let tuning = tuning.unwrap();

    let mut flat = false;
    let root_note = if let Some(ref arg_notes) = root_notes {
        let arg_note = arg_notes.choose(&mut rng).unwrap();
        if FLAT_ACCIDENTALS.contains(arg_note) {
            flat = true
        }
        accidental_to_note(arg_note)
    } else {
        NOTES.choose(&mut rng).copied().unwrap()
    };
    let format = Format {
        flat,
        colored: !uncolored,
    };

    let scale = if let Some(ref arg_scales) = scales {
        arg_scales.choose(&mut rng).copied().unwrap()
    } else {
        SCALES.choose(&mut rng).copied().unwrap()
    };

    let all_frets: Vec<usize> = (0..=NUM_FRETS - FRET_SPAN).collect();
    let starting_fret = if let Some(ref arg_frets) = starting_frets {
        arg_frets.choose(&mut rng).copied().unwrap()
    } else {
        all_frets.choose(&mut rng).copied().unwrap()
    };

    let root_note_index = NOTES.iter().position(|&note| note == root_note).unwrap();
    let steps = get_steps_by_scale(scale);
    let notes_in_scale = steps
        .iter()
        .map(|step| {
            let note_index = (root_note_index + *step) % NUM_NOTES;
            (NOTES[note_index], *step)
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
        format_with_color(note_to_string(root_note, flat), 0, colored),
        scale_to_string(scale),
        starting_fret,
        tuning_to_string(tuning),
    );

    println!(
        "The notes in this scale are: {}",
        notes_in_scale
            .iter()
            .map(|(note, step)| format_with_color(note_to_string(*note, flat), *step, colored))
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
