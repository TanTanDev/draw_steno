use macroquad::prelude::*;
mod parsing;
mod token;
use parsing::*;
use token::Token;

use std::{collections::HashMap, io::Write};

const START_POS: Vec2 = const_vec2!([60f32, 180f32]);

async fn load_textures(library: &[VisualToken]) -> HashMap<Token, Texture2D> {
    let mut textures = HashMap::new();
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

fn draw_steno(
    parsed_steno: &[StenoWord],
    textures: &HashMap<Token, Texture2D>,
    lib: &[VisualToken],
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
            let texture = textures.get(token).unwrap();
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
