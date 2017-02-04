extern crate cask;

use self::cask::Cask;
use game::Game;
use status::Status;
use std::fmt::Display;


pub fn solve<P: Copy + Display, M: Copy + Display>(game: &Game<P, M>,
                                               position: P,
                                               database: &Cask) -> Status {
    let v: Status = (game.primitive)(position);
    if v != Status::Undecided {
        return v
    } else {
        let moves: Vec<M> = (game.gen_moves)(position);
        let vals: Vec<Status> = moves.iter().map(
            |m| solve(game, (game.do_move)(position, *m), &database)
            ).collect();
        if vals.contains(&Status::Loss) {
            database.put(position.to_string(), Status::Win.to_string());
            return Status::Win;
        } else if vals.contains(&Status::Tie) {
            database.put(position.to_string(), Status::Tie.to_string());
            return Status::Tie;
        } else {
            database.put(position.to_string(), Status::Loss.to_string());
            return Status::Loss;
        }
    }
}
