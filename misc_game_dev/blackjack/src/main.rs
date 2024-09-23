use std::io::stdout;

use rand::{Rng, thread_rng};
use crossterm::{
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    ExecutableCommand,
    cursor::MoveTo,
    terminal
};

use deckofcards::*;


// #[derive(Copy, Clone)]
// enum Suit {
//     DIAMONDS,
//     HEARTS,
//     SPADES,
//     CLUBS,
// }

// #[derive(Copy, Clone)]
// struct Card {
//     rank: i32,
//     suit: Suit,
// }

// impl Card {
// }

// #[derive(Clone)]
// struct Deck {
//     cards: Vec<Card>,
//     dealt: Vec<Card>,
// }

// impl Deck {
//     fn new(n_decks: usize) -> Deck {
//         let mut deck = Deck { cards: vec![], dealt: vec![] };
//         let mut cards: Vec<Card> = vec![];
//         for _ in 0..n_decks {
//             cards.extend(deck._new_single_deck());
//         }
//         deck.cards = cards;
//         deck.shuffle();
//         deck
//     }

//     fn reset(&mut self) {
//         self.cards.extend(self.dealt.clone());
//         self.dealt = vec![];
//     }

//     fn shuffle(&mut self) {
//         let mut rng = thread_rng();

//         // Knuth shuffle
//         let l = self.cards.len();
//         for n in 0..l {
//             let i = rng.gen_range(0..(l - n));
//             self.cards.swap(i, l - n - 1);
//         }
//     }

//     fn _new_single_deck(&self) -> Vec<Card> {
//         vec![
//             Card {rank: 2, suit: Suit::SPADES},
//             Card {rank: 3, suit: Suit::SPADES},
//             Card {rank: 4, suit: Suit::SPADES},
//         ]
//     }
// }

// struct Hand {
//     cards: Vec<Card>
// }

// impl Hand {

// }

fn main() -> std::io::Result<()> {
    stdout().execute(terminal::Clear(terminal::ClearType::All))?;

    // setup
    let mut deck = Deck::new();
    let mut dealer = Hand::new();
    let mut player = Hand::new();

    deck.deal_to_hand(&mut dealer, 2);
    deck.deal_to_hand(&mut player, 2);

    

    // stdout()
    //     .execute(MoveTo(0, 0))?
    //     .execute(SetForegroundColor(Color::Red))?
    //     .execute(Print("."))?
    //     .execute(ResetColor)?;

    // stdout()
    //     .execute(MoveTo(1, 1))?
    //     .execute(SetForegroundColor(Color::White))?
    //     .execute(Print("%%"))?
    //     .execute(ResetColor)?;
    
    // stdout()
    //     .execute(MoveTo(1, 4))?
    //     .execute(SetForegroundColor(Color::White))?
    //     .execute(Print("@@"))?
    //     .execute(ResetColor)?;
    

    Ok(())
}

