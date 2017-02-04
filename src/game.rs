use status::Status;

#[derive(Clone)]
pub struct Game<P: Copy, M: Copy> {
    pub initial_pos: P,
    pub do_move: fn(P, M) -> P,
    pub gen_moves: fn(P) -> Vec<M>,
    pub primitive: fn(P) -> Status,
}
