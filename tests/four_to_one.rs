extern crate cask;
extern crate gamesman;

use cask::Cask;
use gamesman::status::Status;
use gamesman::game::Game;
use gamesman::solver::solve;

static INITIAL_POSITION: u32 = 4;

fn do_move(pos: u32, mv: u32) -> u32 {
    return pos - mv;
}

fn gen_moves(pos: u32) -> Vec<u32> {
    if pos == 0 {
        return vec![]
    } else if pos == 1 {
        return vec![1]
    }
    vec![1, 2]
}

fn primitive(pos: u32) -> Status {
    if pos == 0 {
       return Status::Loss
    }
    Status::Undecided
}

#[test]
fn solve_four_to_one() {
    let database = Cask::open("fto1.db", false);
    let game = Game {
        initial_pos: INITIAL_POSITION,
        do_move: do_move,
        gen_moves: gen_moves,
        primitive: primitive,
    };
    solve(&game, game.initial_pos, &database);
}
