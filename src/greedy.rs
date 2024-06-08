use crate::game::{GameState, MonsterId, MonsterState, Solver};
use crate::task::Solution;

pub struct GreedySolver {
    // pub exp_reward: f32,
}

impl GreedySolver {
    pub fn monster_priority(&self, game_state: &GameState, m_id: MonsterId) -> i64 {
        let m = &game_state.monsters[m_id];
        let dst = m.p.dst_sqr(&game_state.hero.p);
        let approx_moves = ((dst as f32).sqrt() - game_state.hero.range() as f32).max(0f32) / (game_state.hero.speed() as f32);
        let approx_attacks = m.hp as f32 / (game_state.hero.power() as f32);

        let gold = m.gold as f32;
        let exp = m.exp as f32;

        let steps = game_state.task.num_turns as f32;
        let progress = (game_state.steps() as f32) / steps;

        let reward = exp * (1f32 - progress) + gold * progress;
        let fee = (approx_moves + approx_attacks) / steps;

        let priority = reward / fee;

        // eprintln!("m_id: {:?} priority: {:?}", m_id, priority);
        return (priority * 10000000f32) as i64
    }
}

// fn find_monster_to_move_to(game_state: &GameState) -> Option<MonsterId> {
//     game_state.alive_monsters().min_by_key(|m| {
//         m.p.dst_sqr(&game_state.hero.p)
//     }).map(|m| m.id)
// }

impl Solver for GreedySolver {
    fn solve(&self, game_state: &mut GameState) -> Solution {
        while !game_state.is_game_over() {
            if let Some(m_id) = game_state.alive_monsters().max_by_key(|m_id| {
                self.monster_priority(game_state, *m_id)
            }) {
                if game_state.can_attack(m_id) {
                    game_state.do_attack(m_id, None).unwrap();
                } else {
                    let m = &game_state.monsters[m_id];
                    let next_p = game_state.hero.p.in_radius_with_direction(&m.p, game_state.hero.speed());
                    game_state.do_move(next_p.x, next_p.y, None).unwrap();
                }
            } else {
                break;
            }
        }
        Solution {
            moves: game_state.commands.clone(),
        }
    }
}