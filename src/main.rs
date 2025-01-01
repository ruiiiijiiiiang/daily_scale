use chrono::prelude::*;
use clap::Parser;
use rand::prelude::*;
use rand::SeedableRng;

mod lib {
    pub mod fret_board;
    pub mod notes;
    pub mod scales;
}
use lib::fret_board::*;
use lib::notes::*;
use lib::scales::*;

#[derive(Parser, Debug)]
#[command(name = "daily-scale", version, about = "Have you practiced today?", long_about = None)]
struct Args {
    #[arg(
        value_enum,
        value_delimiter = ',',
        required = false,
        short = 'n',
        long,
        help = "Pool of root notes to randomly choose from"
    )]
    root_notes: Option<Vec<Note>>,

    #[arg(
        value_enum,
        value_delimiter = ',',
        required = false,
        short = 's',
        long,
        help = "Pool of scales to randomly choose from"
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

fn main() {
    let Args {
        root_notes,
        scales,
        starting_frets,
        ..
    } = Args::parse();

    let num_days_since_epoch = Utc::now().date_naive().num_days_from_ce();
    let seed = num_days_since_epoch.to_string().parse::<u64>().unwrap();
    let mut rng = StdRng::seed_from_u64(seed);

    let root_note = if let Some(ref arg_notes) = root_notes {
        arg_notes.choose(&mut rng).unwrap()
    } else {
        NOTES.choose(&mut rng).unwrap()
    };

    let scale = if let Some(ref arg_scales) = scales {
        arg_scales.choose(&mut rng).unwrap()
    } else {
        SCALES.choose(&mut rng).unwrap()
    };

    let all_frets: Vec<usize> = (0..=NUM_FRETS - FRET_SPAN).collect();
    let starting_fret = if let Some(ref arg_frets) = starting_frets {
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

    let fret_board: Vec<String> = build_fret_board(*starting_fret, &notes_in_scale);

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
