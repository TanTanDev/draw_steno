use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum_macros::{AsRefStr, EnumIter};

#[rustfmt::skip]
#[derive(Copy, Clone, Debug, Hash, EnumIter, Eq, PartialEq, AsRefStr, Serialize, Deserialize)]
enum Token {
    // vocals
    A, E, I, O, U, Y,
    AO, // å
    AE, // ä
    OO, // ö
    // vowels
    B, C, D, F, G,
    H, J, K, L, M,
    N, P, Q, R, S,
    T, V, W, X, Z,

    // special
    BR,
    STENOGRAFI,
}

struct ProccessedToken {
    consumed_chars: usize,
    token: Option<Token>,
}

enum TokenStop {
    NoSuchCharacter,
    EndOfWord,
    NoMatchContinue,
}

#[derive(Serialize, Deserialize)]
pub struct SerializedVec2 {
    pub x: f32,
    pub y: f32,
}

#[derive(Serialize, Deserialize)]
pub struct VisualToken {
    token: Token,
    start: SerializedVec2,
    end: SerializedVec2,
}

fn str_to_processed_token(input: &str) -> Result<TokenStop, ProccessedToken> {
    let len = input.len();
    let token = match input {
        "br" => Token::BR,
        "a" => Token::A,
        "e" => Token::E,
        "i" => Token::I,
        "o" => Token::O,
        "u" => Token::U,
        "y" => Token::Y,
        "å" => Token::AO,
        "ä" => Token::AE,
        "ö" => Token::OO,
        "b" => Token::B,
        "c" => Token::C,
        "d" => Token::D,
        "f" => Token::F,
        "g" => Token::G,
        "h" => Token::H,
        "j" => Token::J,
        "k" => Token::K,
        "l" => Token::L,
        "m" => Token::M,
        "n" => Token::N,
        "p" => Token::P,
        "q" => Token::Q,
        "r" => Token::R,
        "s" => Token::S,
        "t" => Token::T,
        "v" => Token::V,
        "w" => Token::W,
        "x" => Token::X,
        "z" => Token::Z,
        "stenografi" => Token::STENOGRAFI,
        //_ => return Ok(TokenStop::NoSuchCharacter),
        _ => {
            return Err(ProccessedToken {
                consumed_chars: 1,
                token: None,
            })
        }
    };
    Err(ProccessedToken {
        consumed_chars: len,
        token: Some(token),
    })
}

fn find_return(find: &str, input: &str) -> Result<TokenStop, ProccessedToken> {
    if let Some(index) = input.find(find) {
        if index == 0 {
            str_to_processed_token(find)
        } else {
            Ok(TokenStop::NoMatchContinue)
        }
    } else {
        //Err(ProccessedToken{consumed_chars: 0, token: None})
        Ok(TokenStop::NoMatchContinue)
    }
}

fn find_return_many(input: &str, finds: &[&str]) -> Result<(), ProccessedToken> {
    for find in finds {
        find_return(find, input)?;
    }
    Ok(())
}

fn tokenise(input: &str) -> Result<TokenStop, ProccessedToken> {
    find_return_many(
        input,
        &[
        "stenografi",
        "br",
        "a",
        "e",
        "i",
        "e",
        "i",
        "o",
        "u",
        "y",
        "å",
        "ä",
        "ö",
        "b",
        "c",
        "d",
        "f",
        "g",
        "h",
        "j",
        "k",
        "l",
        "m",
        "n",
        "p",
        "q",
        "r",
        "s",
        "t",
        "v",
        "w",
        "x",
        "z",
            ],
    )?;

    Ok(TokenStop::EndOfWord)
}

type StenoWord = Vec<Token>;
type StenoSentence = Vec<StenoWord>;

fn parse(input: &str) -> Result<StenoSentence, ()> {
    let input = input.to_lowercase();
    let input_words = input.split(' ');
    let mut sentence = Vec::new();
    for word in input_words {
        let mut steno_word = Vec::new();
        let mut char_index = 0;
        loop {
            let token_result = tokenise(word.split_at(char_index - 0).1);
            match token_result {
                Err(proccessed_token) => {
                    char_index += proccessed_token.consumed_chars;
                    if let Some(token) = proccessed_token.token {
                        steno_word.push(token);
                    }
                }
                Ok(err) => {
                    match err {
                        TokenStop::NoSuchCharacter => return Err(()),
                        TokenStop::EndOfWord => break,
                        TokenStop::NoMatchContinue => (),
                    };
                }
            };
        }
        sentence.push(steno_word);
    }
    Ok(sentence)
}

use macroquad::prelude::*;

use std::{collections::HashMap, fmt::Debug, hash::Hash, io::Write};
async fn load_textures(library: &Vec<VisualToken>) -> HashMap<Token, Texture2D> {
    let mut textures = HashMap::new();
    //for token in Token::iter() {
    for visual_token in library {
        let token_name = visual_token.token.as_ref().to_lowercase();
        let file_name = format!("res/{}.png", token_name);
        let texture_result = load_texture(&file_name).await;
        if let Ok(texture) = texture_result {
            textures.insert(visual_token.token, texture);
        }
    }
    textures
}

#[macroquad::main("stenografi")]
async fn main() {
    let library_result = load_string("res/library.json").await;
    let library: Vec<VisualToken> = match library_result {
        Ok(library_string) => serde_json::from_str(&library_string).unwrap(),
        Err(_) => {
            let default_library: Vec<VisualToken> = [VisualToken {
                token: Token::A,
                start: SerializedVec2 { x: 0f32, y: 0f32 },
                end: SerializedVec2 { x: 0f32, y: 0f32 },
            }]
            .into();
            let serialized = serde_json::to_string(&default_library).unwrap();
            let mut file = std::fs::File::create("res/library.json").unwrap();
            file.write_all(serialized.as_bytes()).unwrap();
            default_library
        }
    };
    let textures = load_textures(&library).await;

    //let parsed_text = parse("nu ä d dags f e lit medelande").unwrap();
    let mut text_input = String::new();
    loop {
        clear_background(WHITE);
        let parsed = parse(text_input.as_str()).unwrap();
        if let Some(c) = get_char_pressed() {
            text_input.push(c);
        }
        if let Some(key) = get_last_key_pressed() {
            match key {
                KeyCode::Backspace => {
                    let _ = text_input.pop();
                }
                KeyCode::Escape => {
                    let _ = text_input.clear();
                }
                _ => (),
            }
        };
        let zoom = 0.5f32;
        let (w, h) = (screen_width(), screen_height());
        set_camera(&Camera2D {
            target: vec2(w * 1.0f32, h),
            zoom: vec2(1f32 / w * 2f32 * zoom, -1f32 / h * 2f32 * zoom),
            ..Default::default()
        });
        draw_steno(&parsed, &textures, &library);
        set_default_camera();
        draw_text(
            text_input.as_str(),
            40f32,
            screen_height() - 60f32,
            25f32,
            BLACK,
        );
        next_frame().await
    }
}

const START_POS: Vec2 = const_vec2!([60f32, 180f32]);

fn draw_steno(
    parsed_steno: &StenoSentence,
    textures: &HashMap<Token, Texture2D>,
    lib: &Vec<VisualToken>,
) {
    let max_word_height = 330f32;
    let average_word_width = 290f32;
    let mut position = vec2(START_POS.x, START_POS.y);
    let word_spacing = 80f32;
    let bounds = Rect::new(
        position.x,
        position.y,
        screen_width() - position.x,
        screen_height() - position.y,
    );
    let mut current_line = 0;
    for word_tokens in parsed_steno.iter() {
        for token in word_tokens {
            let texture = textures.get(&token).unwrap();
            let token_data = lib.iter().find(|vt| vt.token == *token).unwrap();
            let pivot = vec2(-token_data.start.x, -token_data.start.y);
            draw_texture(*texture, position.x + pivot.x, position.y + pivot.y, BLACK);
            // offset the next position to start at the end of this letter
            let delta = pivot + vec2(token_data.end.x, token_data.end.y);
            position += delta;
        }
        // new word
        position.x += word_spacing;
        position.y = START_POS.y + current_line as f32 * max_word_height;
        if position.x + average_word_width > bounds.w + bounds.x {
            current_line += 1;
            position.x = START_POS.x;
            position.y += max_word_height;
        }
    }
}
