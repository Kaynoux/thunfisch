use std::time::Duration;

use crate::prelude::*;

pub const MAX_DEPTH: usize = 128;

#[allow(clippy::too_many_lines)]
pub fn calc_search_time(args: &[&str], board: Board) -> (usize, Duration) {
    // Fixed Depth
    if args.len() >= 2 && args[0] == "depth" {
        return (
            args[1].parse().unwrap_or_default(),
            Duration::new(24 * 3600, 0),
        );
    }

    // Time control
    let mut wtime: u64 = 0;
    let mut btime: u64 = 0;
    let mut winc: u64 = 0;
    let mut binc: u64 = 0;
    let mut movestogo: u64 = 0;
    let mut movetime: u64 = 0;
    let mut fixtime: u64 = 0;

    // iter through search args
    let mut iter = args.iter();
    while let Some(&tok) = iter.next() {
        match tok {
            "wtime" => {
                if let Some(&val) = iter.next() {
                    wtime = val.parse().unwrap_or(0);
                }
            }
            "btime" => {
                if let Some(&val) = iter.next() {
                    btime = val.parse().unwrap_or(0);
                }
            }
            "winc" => {
                if let Some(&val) = iter.next() {
                    winc = val.parse().unwrap_or(0);
                }
            }
            "binc" => {
                if let Some(&val) = iter.next() {
                    binc = val.parse().unwrap_or(0);
                }
            }
            "movestogo" => {
                if let Some(&val) = iter.next() {
                    movestogo = val.parse().unwrap_or(0);
                }
            }
            "movetime" => {
                if let Some(&val) = iter.next() {
                    movetime = val.parse().unwrap_or(0);
                }
            }
            "fixtime" => {
                if let Some(&val) = iter.next() {
                    fixtime = val.parse().unwrap_or(0);
                }
            }
            _ => {}
        }
    }

    // Calculate time budget
    let have_tc = wtime > 0 || btime > 0 || movetime > 0 || fixtime > 0;
    let time_limit = if !have_tc {
        // no time control at all
        Duration::new(24 * 3600, 0)
    } else if fixtime != 0 {
        Duration::from_millis(fixtime)
    } else if movetime > 0 {
        Duration::from_millis(movetime)
    } else {
        // get current color specific values
        let (time_left, inc) = if board.current_color() == White {
            (wtime, winc)
        } else {
            (btime, binc)
        };
        // if no movestogo set we estimate it is 30 (very dumb change later)
        let moves_to_go = if movestogo > 0 { movestogo } else { 30 };
        let mut time_per_move = time_left / moves_to_go;
        // add half of increment (safe that way than 100%)
        time_per_move += inc / 2;
        // safety margins
        let safety = std::cmp::max(time_left / 20, 50);
        // never take up all time
        if time_per_move + safety >= time_left {
            time_per_move = time_left.saturating_sub(safety);
        }
        // Minimum 10 ms
        if time_per_move < 10 {
            time_per_move = 10;
        }

        Duration::from_millis(time_per_move)
    };
    return (MAX_DEPTH, time_limit);
}
