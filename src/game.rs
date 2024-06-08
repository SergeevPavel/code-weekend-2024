use std::cmp::max;

use crate::game::MoveError::{AttackOutOfRange, MonsterIsDead, MoveOutOfRange, OutOfField};
use crate::geom::Point;
use crate::task::{Command, Hero, Solution, Task};


#[derive(Debug, Clone)]
pub struct HeroState<'a> {
    pub hero: &'a Hero,
    pub p: Point,
    pub lvl: i32,
    pub exp: i32,
}

impl HeroState<'_> {
    pub fn exp_for_lvl(l: i32) -> i32 {
        1000 + l * (l - 1) * 50
    }

    pub fn add_exp(&mut self, exp: i32) {
        self.exp += exp;
        let lvl_up_exp = Self::exp_for_lvl(self.lvl + 1);
        if self.exp >= lvl_up_exp {
            self.exp -= lvl_up_exp;
            self.lvl += 1;
        } else if self.exp < 0 {
            todo!("Implement exp decreasing!")
        }
    }

    pub fn speed(&self) -> i32 {
        let base_speed = self.hero.base_speed;
        let lvl = self.lvl;
        let speed_coeff = self.hero.level_speed_coeff;
        base_speed + base_speed * lvl * speed_coeff / 100
    }

    pub fn power(&self) -> i32 {
        let base_power = self.hero.base_power;
        let lvl = self.lvl;
        let power_coeff = self.hero.level_power_coeff;
        base_power + base_power * lvl * power_coeff / 100
    }

    pub fn range(&self) -> i32 {
        let base_range = self.hero.base_range;
        let lvl = self.lvl;
        let range_coeff = self.hero.level_range_coeff;
        base_range + base_range * lvl * range_coeff / 100
    }
}

#[derive(Debug, Clone)]
pub struct MonsterState {
    pub id: MonsterId,
    pub p: Point,
    pub hp: i32,
    pub gold: i32,
    pub exp: i32,
}

struct Reward {
    gold: i32,
    exp: i32
}

impl MonsterState {
    fn take_damage(&mut self, dmg: i32) -> MoveResult<Option<Reward>> {
        if self.hp == 0 {
            return Err(MonsterIsDead)
        }
        self.hp = max(0, self.hp - dmg);
        if self.hp == 0 {
            let reward = Reward {
                gold: self.gold,
                exp: self.exp,
            };
            self.gold = 0;
            self.exp = 0;
            Ok(Some(reward))
        } else {
            Ok(None)
        }
    }
}

pub type MonsterId = usize;

#[derive(Debug, Clone)]
pub struct GameState<'a> {
    pub task: &'a Task,
    pub score: i32,
    pub hero: HeroState<'a>,
    pub monsters: Vec<MonsterState>,
    pub commands: Vec<Command>
}

#[derive(Debug)]
pub enum MoveError {
    OutOfField,
    MoveOutOfRange,
    AttackOutOfRange,
    MonsterIsDead
}

pub type MoveResult<T> = Result<T, MoveError>;

impl GameState<'_> {
    pub fn new<'a>(task: &'a Task) -> GameState<'a> {
        let monsters = task.monsters.iter().enumerate().map(|(m_id, m)| {
            MonsterState {
                id: m_id,
                p: Point::new(m.x, m.y),
                hp: m.hp,
                gold: m.gold,
                exp: m.exp,
            }
        }).collect();
        GameState {
            task,
            score: 0,
            hero: HeroState {
                hero: &task.hero,
                p: Point::new(task.start_x, task.start_y),
                lvl: 0,
                exp: 0,
            },
            monsters: monsters,
            commands: vec![],
        }
    }

    pub fn alive_monsters(&self) -> impl Iterator<Item = MonsterId> + '_ {
        self.monsters.iter().filter_map(|m| {
            if m.hp != 0 {
                Some(m.id)
            } else {
                None
            }
        })
    }

    pub fn steps(&self) -> i32 {
        self.commands.len() as i32
    }

    pub fn is_game_over(&self) -> bool {
        (self.commands.len() as i32) >= self.task.num_turns
    }

    pub fn do_move(&mut self, x: i32, y: i32, comment: Option<String>) -> MoveResult<()> {
        if x < 0 || y < 0 || x > self.task.width || y > self.task.height {
            return Err(OutOfField)
        }
        let speed = self.hero.speed();
        if !self.hero.p.is_in_radius_of(&Point::new(x, y), speed) {
            return Err(MoveOutOfRange)
        }
        self.hero.p = Point::new(x, y);
        self.commands.push(Command::Move {
            comment: comment,
            target_x: x,
            target_y: y,
        });
        Ok(())
    }

    pub fn can_attack(&self, target_id: MonsterId) -> bool {
        let range = self.hero.range();
        self.monsters[target_id].p.is_in_radius_of(&self.hero.p, range)
    }

    pub fn do_attack(&mut self, target_id: MonsterId, comment: Option<String>) -> MoveResult<()> {
        if !self.can_attack(target_id) {
            return Err(AttackOutOfRange)
        }
        if let Some(reward) = self.monsters[target_id].take_damage(self.hero.power())? {
            self.hero.add_exp(reward.exp);
            self.score += reward.gold;
        }
        self.commands.push(Command::Attack {
            comment: comment,
            target_id: target_id,
        });
        Ok(())
    }
}

pub trait Solver {
    fn solve(&self, game_state: &mut GameState) -> Solution;
}