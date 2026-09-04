use einbocha_playing_cards::{CardSet, DECK_52, PlayingCard, Suit};
use rand::{Rng, RngExt, rng, seq::SliceRandom};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Player {
    A,
    B,
}

impl Player {
    pub fn other(&self) -> Self {
        match self {
            Player::A => Player::B,
            Player::B => Player::A,
        }
    }

    pub fn id(&self) -> usize {
        match self {
            Player::A => 0,
            Player::B => 1,
        }
    }

    pub fn random<R: Rng>(rng: &mut R) -> Self {
        if (&mut *rng).random_bool(0.5) {
            Player::A
        } else {
            Player::B
        }
    }
}

#[derive(Debug)]
pub struct View {
    pub tricks: [usize; 2],
    pub turn: usize,
    pub table: Option<PlayingCard>,
    pub top_card: Option<PlayingCard>,
    pub trump: Suit,
    pub hand: CardSet,
}

#[derive(Clone, Debug)]
pub struct GameState {
    tricks: [usize; 2],
    current_player: Player,
    turn: usize,
    deck: Vec<PlayingCard>,
    table: [Option<PlayingCard>; 2],
    top_card: Option<PlayingCard>,
    trump: Suit,
    player_hands: [CardSet; 2],
}

impl GameState {
    pub fn initial_state() -> Self {
        let mut deck: Vec<PlayingCard> = Vec::from(DECK_52);
        (&mut deck).shuffle(&mut rng());

        let mut player_hands: [CardSet; 2] = [CardSet::new(), CardSet::new()];

        for _ in 0..13 {
            for i in 0_usize..2 {
                player_hands[i].add((&mut deck).pop().unwrap());
            }
        }

        let top_card: PlayingCard = (&mut deck).pop().unwrap();
        let trump: Suit = top_card.suit();

        Self {
            tricks: [0, 0],
            current_player: Player::random(&mut rng()),
            turn: 0,
            deck,
            player_hands,
            trump,
            top_card: Some(top_card),
            table: [None, None],
        }
    }

    pub fn god_view(&self) {
        todo!("create new GodView struct")
    }

    pub fn spectator_view(&self) {
        todo!("create new SpectatorView struct")
    }

    /// Why does the player view only contain the opponents card on the table?
    pub fn player_view(&self, player: Player) -> View {
        View {
            tricks: self.tricks,
            turn: self.turn,
            table: self.table[player.other().id()],
            top_card: self.top_card,
            trump: self.trump,
            hand: self.player_hands[player.id()],
        }
    }

    /// What if the player itself has already played a card?
    /// Where is the current player check, i.e. what if it is not the current player?
    pub fn legal_actions(&self, player: Player) -> CardSet {
        if let Some(card) = self.table[player.other().id()] {
            let mut following: CardSet = self.player_hands[player.id()];
            following.filter_by_suit(card.suit());

            if following.is_empty() {
                self.player_hands[player.id()]
            } else {
                following
            }
        } else {
            self.player_hands[player.id()]
        }
    }

    pub fn current_player(&self) -> Player {
        self.current_player
    }

    fn end_of_round(&self) -> bool {
        self.turn % 2 == 0
    }

    fn phase_one(&self) -> bool {
        self.top_card.is_some()
    }

    /// Compare player with the current player as an additional check
    pub fn if_legal_apply_action(&mut self, player: Player, action: PlayingCard) {
        if !self.legal_actions(player).contains(action) {
            return;
        }

        self.player_hands[player.id()].remove(action);
        (&mut self.table)[player.id()] = Some(action);

        self.turn += 1;

        if self.end_of_round() {
            let second_player: Player = player;
            let first_player: Player = second_player.other();

            let second_card: PlayingCard = (&mut self.table)[second_player.id()].unwrap();
            let first_card: PlayingCard = (&mut self.table)[first_player.id()].unwrap();

            let winner: Player;
            if first_card.suit() == self.trump && second_card.suit() != self.trump {
                winner = first_player;
            } else if second_card.suit() == self.trump && first_card.suit() != self.trump {
                winner = second_player;
            } else if second_card.suit() == first_card.suit() {
                if first_card.rank() > second_card.rank() {
                    winner = first_player;
                } else {
                    winner = second_player;
                }
            } else {
                winner = first_player;
            }

            if self.phase_one() {
                self.player_hands[winner.id()].add((&mut self.top_card).unwrap());
                self.player_hands[winner.other().id()].add((&mut self.deck).pop().unwrap());
                self.top_card = (&mut self.deck).pop();
            } else {
                self.tricks[winner.id()] += 1;
            }

            self.table = [None, None];
            self.current_player = winner;
        } else {
            self.current_player = player.other();
        }
    }

    pub fn finished(&self) -> bool {
        self.turn > 51
    }

    pub fn winner(&self) -> Option<Player> {
        if self.finished() {
            let player_a: Player = Player::A;
            let player_b: Player = Player::B;
            if self.tricks[player_a.id()] > self.tricks[player_b.id()] {
                Some(player_a)
            } else if self.tricks[player_b.id()] > self.tricks[player_a.id()] {
                Some(player_b)
            } else {
                panic!("Impossible to have a draw")
            }
        } else {
            None
        }
    }
}
