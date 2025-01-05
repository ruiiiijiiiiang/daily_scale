use daily_scale::lib::cli::{get_params, print_output, Params};
use daily_scale::lib::fret_board::build_fret_board;

fn main() {
    let params = get_params();
    let Params {
        tuning,
        starting_fret,
        flat,
        ref notes_in_scale,
        ..
    } = params;

    let fret_board = build_fret_board(tuning, starting_fret, notes_in_scale, flat);

    print_output(params, fret_board);
}
