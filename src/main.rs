mod engine;
mod game;

extern crate piston_window;
extern crate find_folder;

use piston_window::*;
use crate::ActiveScreen::{MainGame, InitScreen};

enum ActiveScreen {
    MainGame(game::MainGame),
    InitScreen,
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

    let mut active = InitScreen;

    while let Some(e) = window.next() {
        match &mut active {
            MainGame(ref mut game) => {
                if !game.run(&mut window, &mut glyphs, e) {
                    active = InitScreen;
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

                    text([0.0, 0.0, 0.0, 1.0], 64, "Press enter", &mut glyphs,
                         c.transform.trans(40.0, rect_size * field_size / 2.0), g).unwrap();
                    text([0.0, 0.0, 0.0, 1.0], 64, "to start", &mut glyphs,
                         c.transform.trans(40.0, rect_size * field_size / 2.0 + 50.0), g).unwrap();

                    // Update glyphs before rendering.
                    glyphs.factory.encoder.flush(device);
                });
            }
        }
    }
}
