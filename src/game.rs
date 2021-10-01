use piston_window::*;
use crate::engine;

use engine::GameEvent;


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

    pub fn current_score(&self) -> u32 {
        self.game.current_length() as u32
    }

    pub fn run(&mut self, window: &mut PistonWindow, glyphs: &mut Glyphs, e: Event) -> GameEvent {
        let mut game_event = GameEvent::None;

        if let Some(button) = e.press_args() {
            match button {
                Button::Keyboard(key) => self.game.handle_key(key),
                _ => (),
            }
        }

        if let Some(res) = e.update(|arg| {
            self.game.make_step(arg.dt)
        }) {
            if let GameEvent::Collision = res {
                return GameEvent::Collision;
            }

            game_event = res;
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

            let snake_slices = self.game.snake_body.as_slices();

            if snake_slices.0.len() > 0 {
                // do not render very first element
                for p in snake_slices.0[1..].iter() {
                    rectangle([1.0, 0.2, 0.2, 1.0], // red
                              [p.0.x as f64 * self.rect_size, p.0.y as f64 * self.rect_size, self.rect_size, self.rect_size], // rectangle
                              c.transform, g);
                }
                
                for p in snake_slices.1.iter() {
                    rectangle([1.0, 0.2, 0.2, 1.0], // red
                              [p.0.x as f64 * self.rect_size, p.0.y as f64 * self.rect_size, self.rect_size, self.rect_size], // rectangle
                              c.transform, g);
                }
            }

            self.game.get_interpolated_snake_tail().map(|snake_tail| {
                rectangle([1.0, 0.2, 0.2, 1.0], // red
                          [snake_tail.x as f64 * self.rect_size, snake_tail.y as f64 * self.rect_size, self.rect_size, self.rect_size], // rectangle
                          c.transform, g);
            });



            let snake_head = self.game.get_interpolated_snake_head();

            rectangle([0.8, 0.0, 0.0, 1.0], // red
                      [snake_head.x as f64 * self.rect_size, snake_head.y as f64 * self.rect_size, self.rect_size, self.rect_size], // rectangle
                      c.transform, g);

            rectangle([0.1, 0.8, 0.1, 1.0], // green
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

            text([0.0, 0.0, 0.0, 1.0], 16, &self.game.current_length().to_string(), glyphs,
                 c.transform.trans(2.0, self.rect_size * self.field_size - 2.0), g).unwrap();

            // Update glyphs before rendering.
            glyphs.factory.encoder.flush(device);
        });


        return game_event;
    }
}