use piston_window::*;
use crate::engine;


pub struct MainGame {
    rect_size: f64,
    field_size: f64,
    game: engine::Game,
}

impl MainGame {
    pub fn new(rect_size: f64,
               field_size: f64) -> MainGame {
        MainGame {
            rect_size,
            field_size,
            game: engine::Game::new(field_size as i32, field_size as i32),
        }
    }

    pub fn run(&mut self, window: &mut PistonWindow, glyphs: &mut Glyphs, e: Event) -> bool {
        if let Some(button) = e.press_args() {
            match button {
                Button::Keyboard(key) => self.game.handle_key(key),
                _ => (),
            }
        }

        if let Some(res) = e.update(|arg| {
            self.game.check_step(arg.dt)
        }) {
            if !res {
                return false;
            }
        }

        window.draw_2d(&e, |c, g, device| {
            clear([0.95, 0.95, 0.95, 1.0], g);

            for door in &self.game.doors {
                for cell in &door.cells {
                    let color = if door.open { [0.9, 0.9, 0.9, 1.0] } else { [0.4, 0.4, 0.4, 1.0] };
                    rectangle(color,
                              [cell.x as f64 * self.rect_size, cell.y as f64 * self.rect_size, self.rect_size, self.rect_size], // rectangle
                              c.transform, g);
                }
            }


            for p in self.game.snake_body.iter() {
                rectangle([1.0, 0.0, 1.0, 1.0], // red
                          [p.x as f64 * self.rect_size, p.y as f64 * self.rect_size, self.rect_size, self.rect_size], // rectangle
                          c.transform, g);
            }

            rectangle([1.0, 0.0, 0.0, 1.0], // red
                      [self.game.snake_front.x as f64 * self.rect_size, self.game.snake_front.y as f64 * self.rect_size, self.rect_size, self.rect_size], // rectangle
                      c.transform, g);

            rectangle([0.0, 1.0, 0.0, 1.0], // red
                      [self.game.food_location.x as f64 * self.rect_size, self.game.food_location.y as f64 * self.rect_size, self.rect_size, self.rect_size], // rectangle
                      c.transform, g);

            for y in 0..self.game.game_field.num_rows() {
                for x in 0..self.game.game_field.num_columns() {
                    if self.game.game_field[(y, x)] > 0 {
                        rectangle([0.3, 0.3, 0.3, 1.0],
                                  [x as f64 * self.rect_size, y as f64 * self.rect_size, self.rect_size, self.rect_size], // rectangle
                                  c.transform, g);
                    }
                }
            }

            for t in &self.game.teleporters {
                rectangle([0.1, 0.1, 0.9, 1.0],
                          [t.start.x as f64 * self.rect_size, t.start.y as f64 * self.rect_size, self.rect_size, self.rect_size], // rectangle
                          c.transform, g);
                rectangle([0.1, 0.5, 0.9, 1.0],
                          [t.end.x as f64 * self.rect_size, t.end.y as f64 * self.rect_size, self.rect_size, self.rect_size], // rectangle
                          c.transform, g);
            }

            text([0.0, 0.0, 0.0, 1.0], 16, &self.game.snake_body.len().to_string(), glyphs,
                 c.transform.trans(2.0, self.rect_size * self.field_size - 2.0), g).unwrap();

            // Update glyphs before rendering.
            glyphs.factory.encoder.flush(device);
        });


        return true;
    }
}