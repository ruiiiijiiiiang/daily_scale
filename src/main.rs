mod cli;
mod fret_board;
mod notes;
mod scales;
mod tunings;

use crate::{
    cli::{get_params, print_output, Params},
    fret_board::build_fret_board,
};

fn main() {
    let params = get_params();
    let Params {
        tuning,
        starting_fret,
        ref notes_in_scale,
        ref format,
        ..
    } = params;

    let fret_board = build_fret_board(tuning, starting_fret, notes_in_scale, format);

    print_output(params, fret_board);
}
