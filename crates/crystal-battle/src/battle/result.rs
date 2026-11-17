//! What a battle leaves behind for the game to write back into its save.

use super::outcome::Outcome;
use super::setup::{Bag, Pokemon, World};

/// The battle's result: the player's side as the battle left it, and what
/// the game does next.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BattleResult {
    pub outcome: Outcome,
    /// The player's party after the battle: HP, PP, status, experience,
    /// levels, stats and moves as they changed, a caught mon included (the
    /// battle doesn't nickname it: its nickname is its species' name).
    pub party: Vec<Pokemon>,
    /// The party slot of the mon caught (`Outcome::Caught`).
    pub caught: Option<usize>,
    /// The player's money, prize money included.
    pub money: u32,
    /// Mom's savings, her share of the prize money included.
    pub mom_money: u32,
    /// Money scattered by Pay Day, which the game adds after a won battle
    /// (`CheckPayDay`).
    pub pay_day: u32,
    /// The bag after the battle.
    pub bag: Bag,
    /// Party slots whose mon may evolve now (`wEvolvableFlags`): the game
    /// runs `EvolveAfterBattle` after a won battle.
    pub may_evolve: Vec<usize>,
    /// The world after the battle: the Pokédex and Unown dex with what the
    /// battle saw and caught.
    pub world: World,
}
