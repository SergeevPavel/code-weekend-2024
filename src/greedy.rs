use crate::game::{GameState, HeroState, MonsterId, MonsterState, Solver};
use crate::task::Solution;

#[derive(Debug)]
pub struct GreedySolver {
    pub exp_mul: f32,
    pub degrade_exp: bool,
}

impl Default for GreedySolver {
    fn default() -> Self {
        GreedySolver {
            exp_mul: 1.0,
            degrade_exp: false,
        }
    }
}

impl GreedySolver {
    pub fn monster_priority(&self, game_state: &GameState, m_id: MonsterId) -> i64 {
        let m = &game_state.monsters[m_id];
        let dst = m.p.dst_sqr(&game_state.hero.p);
        let approx_moves = ((dst as f32).sqrt() - game_state.hero.range() as f32).max(0f32) / (game_state.hero.speed() as f32);
        let approx_attacks = m.hp as f32 / (game_state.hero.power() as f32);

        let gold = m.gold as f32;
        let exp = m.exp as f32;

        let speed_coeff = game_state.task.hero.level_speed_coeff as f32 / 100f32;
        let power_coeff = game_state.task.hero.level_power_coeff as f32 / 100f32;
        let range_coeff = game_state.task.hero.level_range_coeff as f32 / 100f32;
        let lvl_coeff = (speed_coeff + power_coeff + range_coeff) / 3f32;

        let exp_reward = exp / (HeroState::exp_for_lvl(game_state.hero.lvl + 1) as f32) * lvl_coeff;

        let steps = game_state.task.num_turns as f32;
        let progress = (game_state.steps() as f32) / steps;

        let reward = if self.degrade_exp {
            exp_reward * (1f32 - progress) * self.exp_mul + gold * progress
        } else {
            exp_reward * self.exp_mul + gold
        };
        let fee = (approx_moves + approx_attacks) / steps;

        let priority = reward / fee;

        // eprintln!("m_id: {:?} priority: {:?}", m_id, priority);
        return (priority * 10000000f32) as i64
    }
}

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