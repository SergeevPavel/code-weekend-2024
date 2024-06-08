use crate::game::{GameState, MonsterId, Solver};
use crate::task::Solution;

pub struct GreedySolver {}

fn find_monster_to_move_to(game_state: &GameState) -> Option<MonsterId> {
    game_state.alive_monsters().min_by_key(|m| {
        m.p.dst_sqr(&game_state.hero.p)
    }).map(|m| m.id)
}

impl Solver for GreedySolver {
    fn solve(game_state: &mut GameState) -> Solution {
        while !game_state.is_game_over() {
            let target_to_attack = game_state.alive_monsters().map(|m| m.id).find(|id| {
                game_state.can_attack(*id)
            });
            if let Some(m_id) = target_to_attack {
                game_state.do_attack(m_id, None).unwrap();
                continue
            }

            let target_to_move_to = find_monster_to_move_to(game_state);
            let next_p = target_to_move_to.map(|m_id| {
                let m = &game_state.monsters[m_id];
                game_state.hero.p.in_radius_with_direction(&m.p, game_state.hero.speed())
            });
            if let Some(next_p) = next_p {
                game_state.do_move(next_p.x, next_p.y, None).unwrap();
            }
        }
        Solution {
            moves: game_state.commands.clone(),
        }
    }
}