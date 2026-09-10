use crate::{Player, View};
use einbocha_playing_cards::{CardSet, PlayingCard};
use rand::{rng, seq::IteratorRandom};

pub trait LegalStrategy: Send + Sync {
    /// Implement this function for your type to implement the LegalStrategy trait.
    /// You can rely on the game to handle illegal actions in general,
    /// but if you want custom handling make sure to only return legal actions
    fn choose_card(&self, player: Player, view: View, legal_actions: CardSet) -> PlayingCard;
}

impl<F> LegalStrategy for F
where
    F: Fn(Player, View, CardSet) -> PlayingCard + Send + Sync,
{
    /// Implemented by the generic implemenentation of LegalStrategy for:
    /// Fn(Player, View, CardSet) -> PlayingCard + Send + Sync
    fn choose_card(&self, player: Player, view: View, legal_actions: CardSet) -> PlayingCard {
        self(player, view, legal_actions)
    }
}

// todo: does a random strategy strategy, i.e. a strategy which chooses everytime another random
// strategy to do its work, make any sense? So the strategy changes its behavior everytime or maybe
// only every few rounds (would require a state)

/// Simply picks the first available legal action.
/// Note: Since HashSet iteration order is undefined, this is effectively pseudo-random,
/// but it is very fast.
pub fn first_legal_strategy(
    _player: Player,
    _view: View,
    mut legal_actions: CardSet,
) -> PlayingCard {
    legal_actions.pop_any().expect("No legal actions available")
}

/// Always picks the legally allowed card with the highest rank.
pub fn highest_rank_strategy(_player: Player, _view: View, legal_actions: CardSet) -> PlayingCard {
    legal_actions
        .iter()
        .max_by_key(|c| c.rank())
        .expect("No legal actions available")
}

/// Always picks the legally allowed card with the lowest rank.
pub fn lowest_rank_strategy(_player: Player, _view: View, legal_actions: CardSet) -> PlayingCard {
    legal_actions
        .iter()
        .min_by_key(|c| c.rank())
        .expect("No legal actions available")
}

/// Collects the actions into a vector and picks one entirely at random.
pub fn random_strategy(_player: Player, _view: View, legal_actions: CardSet) -> PlayingCard {
    legal_actions
        .iter()
        .choose(&mut rng())
        .expect("No legal actions available")
}
