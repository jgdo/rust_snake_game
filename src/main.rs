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

    let mut game = engine::Game::new(field_size as i32, field_size as i32);

    while let Some(e) = window.next() {
        if let Some(button) = e.press_args() {
            match button {
                Button::Keyboard(key) => game.handle_key(key),
                _ => (),
            }
        }

        window.draw_2d(&e, |c, g, device| {
            clear([0.95, 0.95, 0.95, 1.0], g);

            for door in &game.doors {
                for cell in &door.cells {
                    let color = if door.open { [0.9, 0.9, 0.9, 1.0] } else { [0.4, 0.4, 0.4, 1.0] };
                    rectangle(color,
                              [cell.x as f64 * rect_size, cell.y as f64 * rect_size, rect_size, rect_size], // rectangle
                              c.transform, g);
                }
            }


            for p in game.snake_body.iter() {
                rectangle([1.0, 0.0, 1.0, 1.0], // red
                          [p.x as f64 * rect_size, p.y as f64 * rect_size, rect_size, rect_size], // rectangle
                          c.transform, g);
            }

            rectangle([1.0, 0.0, 0.0, 1.0], // red
                      [game.snake_front.x as f64 * rect_size, game.snake_front.y as f64 * rect_size, rect_size, rect_size], // rectangle
                      c.transform, g);

            rectangle([0.0, 1.0, 0.0, 1.0], // red
                      [game.food_location.x as f64 * rect_size, game.food_location.y as f64 * rect_size, rect_size, rect_size], // rectangle
                      c.transform, g);

            for y in 0..game.game_field.num_rows() {
                for x in 0..game.game_field.num_columns() {
                    if game.game_field[(y, x)] > 0 {
                        rectangle([0.3, 0.3, 0.3, 1.0],
                                  [x as f64 * rect_size, y as f64 * rect_size, rect_size, rect_size], // rectangle
                                  c.transform, g);
                    }
                }
            }

            for t in &game.teleporters {
                rectangle([0.1, 0.1, 0.9, 1.0],
                          [t.start.x as f64 * rect_size, t.start.y as f64 * rect_size, rect_size, rect_size], // rectangle
                          c.transform, g);
                rectangle([0.1, 0.5, 0.9, 1.0],
                          [t.end.x as f64 * rect_size, t.end.y as f64 * rect_size, rect_size, rect_size], // rectangle
                          c.transform, g);
            }

            text([0.0, 0.0, 0.0, 1.0], 16, &game.snake_body.len().to_string(),&mut glyphs,
                 c.transform.trans(2.0, rect_size*field_size-2.0), g).unwrap();

            // Update glyphs before rendering.
            glyphs.factory.encoder.flush(device);
        });

        e.update(|arg| {
            game.check_step(arg.dt);
        });
    }
}
