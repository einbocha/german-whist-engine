#[cfg(feature = "strategies")]
pub mod strategies;

use einbocha_playing_cards::{CardSet, DECK_52, PlayingCard, Suit};
use rand::{Rng, RngExt, seq::SliceRandom};

/// The enum defines both players of the game (A or B).
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Player {
    A,
    B,
}

impl Player {
    /// Get the other player.
    pub fn other(&self) -> Self {
        match self {
            Player::A => Player::B,
            Player::B => Player::A,
        }
    }

    /// Returns the player's unique ID to index associated datastructures.
    pub fn id(&self) -> usize {
        match self {
            Player::A => 0,
            Player::B => 1,
        }
    }

    /// Get a random player.
    pub fn random<R: Rng>(rng: &mut R) -> Self {
        if rng.random_bool(0.5) {
            Player::A
        } else {
            Player::B
        }
    }
}

/// All information available / visible to the player.
#[derive(Debug)]
pub struct View {
    pub tricks: [u8; 2],
    pub current_player: Player,
    pub turn: u8,
    pub table: [Option<PlayingCard>; 2],
    pub top_card: Option<PlayingCard>,
    pub trump: Suit,
    pub hand: CardSet,
}

/// Deterministic game state.
#[derive(Clone, Debug)]
pub struct GameState {
    tricks: [u8; 2],
    current_player: Player,
    turn: u8,
    deck: Vec<PlayingCard>,
    table: [Option<PlayingCard>; 2],
    top_card: Option<PlayingCard>,
    trump: Suit,
    player_hands: [CardSet; 2],
}

impl GameState {
    /// Creates a new randomized initial game state.
    pub fn initial_state<R: Rng>(rng: &mut R) -> Self {
        let mut deck: Vec<PlayingCard> = Vec::from(DECK_52);
        (&mut deck).shuffle(rng);

        let mut player_hands: [CardSet; 2] = [CardSet::new(), CardSet::new()];

        for _ in 0..13 {
            for i in 0_usize..2 {
                player_hands[i].add(deck.pop().unwrap());
            }
        }

        let top_card: PlayingCard = deck.pop().unwrap();
        let trump: Suit = top_card.suit();

        Self {
            tricks: [0, 0],
            current_player: Player::random(rng),
            turn: 0,
            deck,
            player_hands,
            trump,
            top_card: Some(top_card),
            table: [None, None],
        }
    }

    /// Get all not hidden information that is not secret to a player.
    pub fn spectator_view(&self) {
        todo!("create new SpectatorView struct")
    }

    /// Get the player's view on the game.
    pub fn player_view(&self, player: Player) -> View {
        View {
            tricks: self.tricks,
            current_player: self.current_player,
            turn: self.turn,
            table: self.table,
            top_card: self.top_card,
            trump: self.trump,
            hand: self.player_hands[player.id()],
        }
    }

    /// Determines which cards a player can play at the moment.
    pub fn legal_actions(&self, player: Player) -> CardSet {
        if player != self.current_player {
            return CardSet::new(); // Not current player => may not play any card
        }

        // => player has to make an action
        // The player's table card slot has to be empty

        if let Some(card) = self.table[player.other().id()] {
            let mut filtered_hand: CardSet = self.player_hands[player.id()];
            filtered_hand.filter_by_mask(CardSet::mask_suit(card.suit()));

            if filtered_hand.is_empty() {
                // player can choose freely
                self.player_hands[player.id()]
            } else {
                // player has to follow suit
                filtered_hand
            }
        } else {
            self.player_hands[player.id()]
        }
    }

    /// Get the player whose turn it is right now.
    pub fn current_player(&self) -> Player {
        self.current_player
    }

    /// Have both players already played their card for this turn?
    fn end_of_round(&self) -> bool {
        self.turn % 2 == 0
    }

    /// Is the game still in the phase where tricks don't count as points?
    fn phase_one(&self) -> bool {
        self.top_card.is_some()
    }

    /// Checks whether the player can play this card right now.
    /// The player has to be the current player.
    pub fn if_legal_apply_action(&mut self, player: Player, action: PlayingCard) {
        if !self.legal_actions(player).contains(action) {
            return;
        }
        // => action (the card) is legal => can be removed from the hand
        self.player_hands[player.id()].remove(action);
        self.table[player.id()] = Some(action);

        self.turn += 1;

        if self.end_of_round() {
            let second_player: Player = player;
            let first_player: Player = second_player.other();
            // there are two cards on the table
            let second_card: PlayingCard = self.table[second_player.id()].unwrap();
            let first_card: PlayingCard = self.table[first_player.id()].unwrap();

            let winner = if second_card.suit() == first_card.suit() {
                if first_card.rank() > second_card.rank() {
                    first_player
                } else {
                    second_player
                }
            } else if second_card.suit() == self.trump {
                second_player
            } else {
                first_player
            };

            if self.phase_one() {
                self.player_hands[winner.id()].add(self.top_card.unwrap());
                self.player_hands[winner.other().id()].add(self.deck.pop().unwrap());
                self.top_card = self.deck.pop();
            } else {
                self.tricks[winner.id()] += 1;
            }

            self.table = [None, None];
            self.current_player = winner;
        } else {
            self.current_player = player.other();
        }
    }

    /// Is the game over?
    pub fn finished(&self) -> bool {
        self.turn > 51
    }

    /// Does a player won the game and which player has won the game?
    pub fn winner(&self) -> Option<Player> {
        if self.finished() {
            let player_a: Player = Player::A;
            let player_b: Player = Player::B;
            if self.tricks[player_a.id()] > self.tricks[player_b.id()] {
                Some(player_a)
            } else if self.tricks[player_b.id()] > self.tricks[player_a.id()] {
                Some(player_b)
            } else {
                unreachable!("Impossible to have a draw")
            }
        } else {
            None
        }
    }
}
