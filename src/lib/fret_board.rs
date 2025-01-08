use super::cli::{format_with_color, Format};
use super::notes::{note_to_string, Note, NOTES, NUM_NOTES};
use super::tunings::{get_notes_by_tuning, Tuning};

pub const NUM_FRETS: usize = 24;

pub const FRET_SPAN: usize = 5;

pub fn build_fret_board(
    tuning: Tuning,
    starting_fret: usize,
    notes_in_scale: &[(Note, usize)],
    format: &Format,
) -> Vec<String> {
    let mut fret_board = Vec::new();
    let notes_in_tuning = get_notes_by_tuning(tuning);
    for (string_counter, string) in notes_in_tuning.iter().enumerate() {
        let string_char = if string_counter < (notes_in_tuning.len() - NUM_THICK_STRINGS) {
            '='
        } else {
            '-'
        };
        let fret_board_string =
            build_fret_board_string(starting_fret, notes_in_scale, *string, string_char, format);
        fret_board.insert(0, fret_board_string);
    }
    let fret_num_string = build_fret_num_string(starting_fret);
    fret_board.push(fret_num_string);
    fret_board
}

const NUM_THICK_STRINGS: usize = 3;

const FRET_LENGTH: [usize; 25] = [
    0, 10, 10, 9, 9, 9, 8, 8, 8, 8, 7, 7, 7, 7, 7, 6, 6, 6, 6, 6, 6, 5, 5, 5, 5,
];

fn format_note(note: Note, step: usize, string_char: char, format: &Format) -> String {
    let Format { flat, colored } = *format;
    let note_string = note_to_string(note, flat);
    let colored_note = format_with_color(note_string, step, colored);
    if note_string.len() == 1 {
        format!("{}{}", colored_note, string_char)
    } else {
        colored_note
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

fn build_fret_board_string(
    starting_fret: usize,
    notes_in_scale: &[(Note, usize)],
    string: Note,
    string_char: char,
    format: &Format,
) -> String {
    let mut fret_board_string = String::new();
    let empty_string_note_index = NOTES.iter().position(|&note| note == string).unwrap();
    let notes_in_string = (0..=NUM_NOTES)
        .map(|fret| {
            let note_index = (empty_string_note_index + fret) % NUM_NOTES;
            NOTES[note_index]
        })
        .collect::<Vec<Note>>();
    for fret in starting_fret..(starting_fret + FRET_SPAN) {
        let note = notes_in_string[fret % NUM_NOTES];
        if fret == 0 {
            if let Some((_, step)) = notes_in_scale
                .iter()
                .find(|(note_in_scale, _)| *note_in_scale == note)
            {
                fret_board_string.push_str(format_note(note, *step, string_char, format).as_str());
            } else {
                fret_board_string.push(string_char);
                fret_board_string.push(string_char);
            }
        } else {
            fret_board_string.push('|');
            let fret_length = FRET_LENGTH[fret];
            if let Some((_, step)) = notes_in_scale
                .iter()
                .find(|(note_in_scale, _)| *note_in_scale == note)
            {
                let fret_length_odd = fret_length % 2 != 0;
                let first_half_fret_length = fret_length / 2 - if fret_length_odd { 0 } else { 1 };
                let second_half_fret_length = fret_length / 2 - 1;
                for _ in 0..first_half_fret_length {
                    fret_board_string.push(string_char);
                }
                fret_board_string.push_str(format_note(note, *step, string_char, format).as_str());
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
    fret_board_string
}

fn build_fret_num_string(starting_fret: usize) -> String {
    let mut fret_num_string = String::new();
    (starting_fret..(starting_fret + FRET_SPAN)).for_each(|fret| {
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
    });
    fret_num_string.push('|');
    fret_num_string
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_note() {
        assert_eq!(
            format_note(
                Note::A,
                0,
                '-',
                &Format {
                    flat: true,
                    colored: false
                }
            ),
            "A-"
        );
        assert_eq!(
            format_note(
                Note::CSharp,
                0,
                '=',
                &Format {
                    flat: false,
                    colored: false
                },
            ),
            "C#"
        );
        assert_eq!(
            format_note(
                Note::GSharp,
                0,
                '-',
                &Format {
                    flat: true,
                    colored: false
                },
            ),
            "Ab"
        );
    }

    #[test]
    fn test_format_fret_num() {
        assert_eq!(format_fret_num(0), "0 ");
        assert_eq!(format_fret_num(14), "14");
    }

    #[test]
    fn test_build_fret_board_string() {
        assert_eq!(
            build_fret_board_string(
                5,
                &[(Note::A, 0), (Note::B, 2), (Note::C, 3)],
                Note::E,
                '=',
                &Format {
                    flat: false,
                    colored: false
                },
            ),
            "|====A====|========|===B====|===C====|========|"
        );
        assert_eq!(
            build_fret_board_string(
                12,
                &[(Note::DSharp, 1), (Note::E, 2), (Note::FSharp, 4)],
                Note::D,
                '=',
                &Format {
                    flat: false,
                    colored: false
                },
            ),
            "|=======|===D#==|===E===|======|==F#==|"
        );
        assert_eq!(
            build_fret_board_string(
                0,
                &[(Note::B, 0), (Note::CSharp, 2), (Note::DSharp, 4)],
                Note::B,
                '-',
                &Format {
                    flat: true,
                    colored: false
                },
            ),
            "B-|----------|----Db----|---------|----Eb---|"
        );
    }

    #[test]
    fn test_build_fret_num_string() {
        assert_eq!(
            build_fret_num_string(0),
            "  |    1     |    2     |    3    |    4    |"
        );
        assert_eq!(
            build_fret_num_string(12),
            "|   12  |   13  |   14  |  15  |  16  |"
        );
    }

    #[test]
    fn test_build_fret_board() {
        assert_eq!(
            build_fret_board(
                Tuning::OpenG6,
                0,
                &[
                    (Note::A, 0),
                    (Note::B, 2),
                    (Note::C, 3),
                    (Note::D, 5),
                    (Note::E, 7),
                    (Note::F, 8),
                    (Note::GSharp, 11),
                ],
                &Format {
                    flat: false,
                    colored: false
                },
            ),
            vec![
                "D-|----------|----E-----|----F----|---------|",
                "B-|----C-----|----------|----D----|---------|",
                "--|----G#----|----A-----|---------|----B----|",
                "D=|==========|====E=====|====F====|=========|",
                "==|====G#====|====A=====|=========|====B====|",
                "D=|==========|====E=====|====F====|=========|",
                "  |    1     |    2     |    3    |    4    |",
            ]
        );
        assert_eq!(
            build_fret_board(
                Tuning::StandardB7,
                7,
                &[
                    (Note::A, 0),
                    (Note::B, 2),
                    (Note::CSharp, 4),
                    (Note::D, 5),
                    (Note::E, 7),
                    (Note::FSharp, 9),
                    (Note::G, 10),
                ],
                &Format {
                    flat: true,
                    colored: false
                },
            ),
            vec![
                "|---B----|--------|---Db---|---D---|-------|",
                "|---Gb---|---G----|--------|---A---|-------|",
                "|---D----|--------|---E----|-------|---Gb--|",
                "|===A====|========|===B====|=======|===Db==|",
                "|===E====|========|===Gb===|===G===|=======|",
                "|===B====|========|===Db===|===D===|=======|",
                "|===Gb===|===G====|========|===A===|=======|",
                "|   7    |   8    |   9    |   10  |   11  |",
            ]
        );
        assert_eq!(
            build_fret_board(
                Tuning::OpenE6,
                15,
                &[
                    (Note::A, 0),
                    (Note::ASharp, 1),
                    (Note::CSharp, 4),
                    (Note::DSharp, 6),
                    (Note::FSharp, 9),
                    (Note::GSharp, 11),
                ],
                &Format {
                    flat: false,
                    colored: false
                },
            ),
            vec![
                "|------|--G#--|--A---|--A#--|------|",
                "|------|--D#--|------|------|--F#--|",
                "|------|------|--C#--|------|--D#--|",
                "|======|==G#==|==A===|==A#==|======|",
                "|======|==D#==|======|======|==F#==|",
                "|======|==G#==|==A===|==A#==|======|",
                "|  15  |  16  |  17  |  18  |  19  |"
            ]
        );
    }
}
