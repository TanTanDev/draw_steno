use std::str::FromStr;

use crate::token::Token;
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;

pub type StenoWord = Vec<Token>;
pub type StenoSentence = Vec<StenoWord>;

struct ProcessedToken {
    consumed_chars: usize,
    token: Token,
}

#[derive(Serialize, Deserialize)]
pub struct SerializedVec2 {
    pub x: f32,
    pub y: f32,
}

#[derive(Serialize, Deserialize)]
pub struct VisualToken {
    pub token: Token,
    pub start: SerializedVec2,
    pub end: SerializedVec2,
}

// iterates all possible tokens in decleration order, to convert the input str to a token
fn tokenise(input: &str) -> Result<(), ProcessedToken> {
    for token in Token::iter() {
        let token_str = token.as_ref();
        if let Some(index) = input.find(token_str) {
            if index == 0 {
                if let Ok(token) = Token::from_str(token_str) {
                    return Err(ProcessedToken {
                        consumed_chars: token_str.len(),
                        token,
                    });
                }
            }
        }
    }
    Ok(())
}

pub fn parse(input: &str) -> Result<StenoSentence, ()> {
    let input = input.to_lowercase();
    let input_words = input.split(' ');
    let mut sentence = Vec::new();
    for word in input_words {
        let mut tokens = Vec::new();
        let mut char_index = 0;
        while let Err(proccessed_token) = tokenise(word.split_at(char_index).1) {
            char_index += proccessed_token.consumed_chars;
            tokens.push(proccessed_token.token);
        }
        sentence.push(tokens);
    }
    Ok(sentence)
}
