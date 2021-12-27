mod engine;
mod game;
mod sound;
mod text_helpers;

extern crate piston_window;
extern crate find_folder;

use piston_window::*;
use crate::ActiveScreen::{MainGame, InitScreen, LooseScreen};
#[cfg(target_os = "windows")]
use winit::window::Icon;
#[cfg(target_os = "windows")]
use winit::platform::windows::IconExtWindows;


use crate::engine::GameEvent;
use crate::sound::Sound;
use crate::text_helpers::draw_text_multiline;


enum ActiveScreen {
    MainGame(game::MainGame),
    InitScreen,
    LooseScreen(u32),
}

fn main() {
    let rect_size = 20.0;
    let field_size = 20.0;

    let mut window: PistonWindow =
        WindowSettings::new("Rust Snake", [rect_size * field_size; 2])
            .build().unwrap();

    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets").unwrap();
    println!("Found assets: {:?}", assets);
    let mut glyphs = window.load_font(assets.join("FiraSans-Regular.ttf")).unwrap();

    let sound = Sound::new(&assets);

    let mut active = InitScreen;

    #[cfg(target_os = "windows")]
    {
        let icon = Icon::from_path(assets.join("icon.ico"), None);
        println!("icon: {:?}", icon);
        window.window.ctx.window().set_window_icon(icon.ok());
    }

    while let Some(e) = window.next() {
        match &mut active {
            MainGame(ref mut game) => {
                if let Some(button) = e.press_args() {
                    match button {
                        Button::Keyboard(Key::Escape) => {
                            active = InitScreen;
                            continue;
                        }
                        _ => (),
                    }
                }

                let game_result = game.run(&mut window, &mut glyphs, e);
                sound.play_for_event(&game_result);
                if let GameEvent::Collision = game_result {
                    active = LooseScreen(game.current_score());
                }
            }
            InitScreen => {
                if let Some(button) = e.press_args() {
                    match button {
                        Button::Keyboard(Key::Return) => {
                            active = MainGame(game::MainGame::new(rect_size, field_size));
                        }
                        Button::Keyboard(Key::Escape) => {
                            return;
                        }
                        _ => (),
                    }
                }

                window.draw_2d(&e, |c, g, device| {
                    clear([0.95, 0.95, 0.95, 1.0], g);

                    draw_text_multiline([0.0, 0.0, 0.0, 1.0], 64, "Snake Game", Size::from([rect_size * field_size, 100.0]),
                                        &mut glyphs, c.transform, g,
                    ).unwrap();

                    draw_text_multiline([0.0, 0.8, 0.0, 1.0], 48, "Press enter\nto start!", Size::from([rect_size * field_size, rect_size * field_size - 100.0]),
                                        &mut glyphs, c.transform.trans(0.0, 100.0), g,
                    ).unwrap();


                    // Update glyphs before rendering.
                    glyphs.factory.encoder.flush(device);
                });
            }
            LooseScreen(score) => {
                let score = *score;

                if let Some(button) = e.press_args() {
                    match button {
                        Button::Keyboard(Key::Return) => {
                            active = InitScreen;
                        }
                        Button::Keyboard(Key::Escape) => {
                            return;
                        }
                        _ => (),
                    }
                }

                window.draw_2d(&e, |c, g, device| {
                    clear([0.95, 0.95, 0.95, 1.0], g);


                    draw_text_multiline([0.8, 0.0, 0.0, 1.0], 48, &format!("You lost!\n \nScore: {}\n \n \nPress enter\nto return to\nmain screen.", score),
                                        Size::from([rect_size * field_size, rect_size * field_size]),
                                        &mut glyphs, c.transform, g,
                    ).unwrap();


                    // Update glyphs before rendering.
                    glyphs.factory.encoder.flush(device);
                });
            }
        }
    }
}
