mod engine;
mod game;

extern crate piston_window;
extern crate find_folder;

use piston_window::*;

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

    let mut game = game::MainGame::new(rect_size, field_size);

    while let Some(e) = window.next() {
        game.run(&mut window, &mut glyphs, e);
    }
}
