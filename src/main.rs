use crate::game::{GameState, Solver};
use crate::greedy::GreedySolver;
use crate::task::{read_task, Solution, submit, Task};

mod task;
mod game;
mod greedy;
mod geom;

fn hyper_search(test_id: u32) {
    let task = read_task(test_id);
    let mut best_result: Option<GameState> = None;
    let mut best_solver: Option<GreedySolver> = None;
    let mut best_solution: Option<Solution> = None;

    for degrade_exp in [false, true] {
        for exp_mul in [0.0001, 0.001, 0.1, 0.3, 1.0, 3.0, 10.0, 100.0, 1000.0] {
            let solver = GreedySolver {
                degrade_exp: degrade_exp,
                exp_mul: exp_mul,
            };
            let (game_state, solution) = solve_task(&task, &solver);
            if game_state.score > best_result.as_ref().map_or(0, |g| g.score) {
                best_result = Some(game_state);
                best_solver = Some(solver);
                best_solution = Some(solution);
            }
        }
    }

    eprintln!("Task: {:?} Best solution obtained with: {:?}", test_id, best_solver);
    submit(test_id, &best_solution.unwrap());
}

fn solve_task<'a, 'b>(task: &'b Task, solver: &'a impl Solver) -> (GameState<'b>, Solution) {
    let mut game = GameState::new(task);
    let solution = solver.solve(&mut game);
    (game, solution)
}

fn main() {
    for i in 1..=25 {
        hyper_search(i)
    }
}
