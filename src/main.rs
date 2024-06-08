use crate::game::{GameState, Solver};
use crate::greedy::GreedySolver;
use crate::task::{read_task, submit};

mod task;
mod game;
mod greedy;
mod geom;

fn solve_task(test_id: u32) {
    let solver = GreedySolver {};
    let t = read_task(test_id);
    let mut game = GameState::new(&t);
    let solution = solver.solve(&mut game);
    // println!("{:?}", solution.moves.len());
    // println!("{:#?}", solution);
    submit(test_id, &solution)
}

fn main() {
    for i in 1..=25 {
        solve_task(i)
    }
}
