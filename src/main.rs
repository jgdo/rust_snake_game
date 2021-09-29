extern crate piston_window;

use piston_window::*;
use std::collections::VecDeque;
use std::collections::HashSet;
use rand::Rng;
use array2d::Array2D;
use piston_window::keyboard::Key;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct Point2d {
    x: i32,
    y: i32,
}

impl Point2d {
    pub fn new(x: i32, y: i32) -> Point2d {
        Point2d {
            x,
            y,
        }
    }

    pub fn as_coords(&self) -> (usize, usize) {
        (self.y as usize, self.x as usize)
    }
}

struct DirBuffer {
    data: VecDeque<Point2d>,
}

impl DirBuffer {
    pub fn new() -> DirBuffer {
        DirBuffer {
            data: VecDeque::new(),
        }
    }

    pub fn enqueue_dir(&mut self, dir: Point2d) {
        if !self.data.contains(&dir) {
            self.data.push_back(dir);
        }
    }

    pub fn next_dir(&mut self) -> Option<Point2d> {
        self.data.pop_front()
    }
}


#[derive(Hash, Eq, PartialEq)]
struct Teleporter {
    start: Point2d,
    end: Point2d,
}

impl Teleporter {
    pub fn new(start: Point2d, end: Point2d) -> Teleporter {
        Teleporter {
            start,
            end,
        }
    }
}



struct Door {
    cells: HashSet<Point2d>,
    period: usize,
    open: bool,
    current_count: usize,
}

impl Door {
    pub fn new(cells: HashSet<Point2d>, period: usize) -> Door {
        Door {
            cells,
            period,
            open: false,
            current_count: period,
        }
    }

    pub fn tick(&mut self) {
        if self.current_count > 0 {
            self.current_count -= 1;
        } else {
            self.current_count = self.period;
            self.open = !self.open;
        }
    }
}

/*

enum HeadProgress {
    SnakeHead(Point2d),
    Collision
}

trait GameElement {
    fn handle(&self, head: Point2d) -> HeadProgress;
}

impl GameElement for Teleporter {
    fn handle(&self, head: Point2d) -> HeadProgress {
        return HeadProgress::SnakeHead(self.end);
    }
}

impl GameElement for Door {
    fn handle(&self, head: Point2d) -> HeadProgress {
        if self.open {
            return HeadProgress::Collision;
        }
        return HeadProgress::SnakeHead(head);
    }
}

struct Wall {
}

impl GameElement for Wall {
    fn handle(&self, head: Point2d) -> HeadProgress {
        return HeadProgress::Collision;
    }
}


struct GameField<'a> {
    width: i32,
    height: i32,
    cells: Array2D<Option<&'a dyn GameElement>>,
    teleporters: HashSet<Teleporter>,
    doors: Vec<Door>,
    walls: Vec<Wall>,
}

impl<'a> GameField<'a> {
    pub fn new(width: i32,
               height: i32) -> GameField<'a> {
        let mut field = GameField {
            width,
            height,
            cells: Array2D::filled_with(None, height as usize, width as usize),
            teleporters: Default::default(),
            doors: vec![],
            walls: vec![]
        };

        for y in 0..field.height {
            if !(5..8).contains(&y) {
                field.walls.push(Wall {});
                field.cells[(y as usize, (field.width / 2) as usize)] = Some(&field.walls[field.walls.len()-1]);
            }
        }

        return field;
    }
}

*/

struct Game {
    width: i32,
    height: i32,
    snake_body: VecDeque<Point2d>,
    snake_front: Point2d,
    current_length: usize,
    food_location: Point2d,
    next_time: f64,
    dir_x: i32,
    dir_y: i32,
    rng: rand::rngs::ThreadRng,
    dir_buffer: DirBuffer,
    game_field: Array2D<u32>,
    teleporters: HashSet<Teleporter>,
    doors: Vec<Door>,
}

impl Game {
    pub fn new(width: i32, height: i32) -> Game {
        let mut game = Game {
            width: width as i32,
            height: height as i32,
            snake_body: VecDeque::new(),
            snake_front: Point2d::new(width / 4, height / 2),
            current_length: 5,
            food_location: Point2d::new(1, 1),
            next_time: 0.0,
            dir_x: 0,
            dir_y: 1,
            rng: rand::thread_rng(),
            dir_buffer: DirBuffer::new(),
            game_field: Array2D::filled_with(0, height as usize, width as usize),
            teleporters: HashSet::new(),
            doors: Vec::new(),
        };
        for y in 0..game.height {
            if !(5..8).contains(&y) {
                game.game_field[(y as usize, (game.width / 2) as usize)] = 1
            }
        }

        game.insert_teleporter_2way(Point2d::new(game.width - 1, game.height - 1), Point2d::new(0, 0));

        game.doors.push(Door::new(
            Iterator::collect(IntoIterator::into_iter([Point2d::new(game.width / 2, 5), Point2d::new(game.width / 2, 6), Point2d::new(game.width / 2, 7)])),
            5));

        game
    }

    fn handle_key(&mut self, key: piston_window::Key)
    {
        match key {
            Key::Left => self.change_dir(-1, 0),
            Key::Right => self.change_dir(1, 0),
            Key::Up => self.change_dir(0, -1),
            Key::Down => self.change_dir(0, 1),
            _ => ()
        }
    }

    fn change_dir(&mut self, dir_x: i32, dir_y: i32) {
        self.dir_buffer.enqueue_dir(Point2d::new(dir_x, dir_y));
    }

    fn teleport_if_needed(&self, front: Point2d) -> Point2d {
        for t in &self.teleporters {
            if front == t.start
            {
                return t.end;
            }
        }

        front
    }

    fn move_pos(&mut self, dx: i32, dy: i32) {
        self.snake_body.push_back(self.snake_front);

        let mut next_front = self.snake_front;
        next_front.x = std::cmp::min(std::cmp::max(0, next_front.x + dx), self.width - 1);
        next_front.y = std::cmp::min(std::cmp::max(0, next_front.y + dy), self.height - 1);

        if !self.cell_is_free(next_front) {
            eprintln!("Snake collision!");
            std::process::exit(1);
        }


        self.snake_front = self.teleport_if_needed(next_front);

        if self.snake_front == self.food_location {
            self.current_length += 1;

            loop {
                self.food_location.x = self.rng.gen_range(0..self.width);
                self.food_location.y = self.rng.gen_range(0..self.height);

                if self.cell_is_free(self.food_location) {
                    break;
                }
            }
        }

        while self.snake_body.len() > self.current_length {
            self.snake_body.pop_front();
        }
    }

    fn check_step(&mut self, dt: f64) {
        let time_step = 0.3;

        self.next_time += dt;
        while self.next_time >= time_step {
            self.next_time -= time_step;

            self.make_step();
        }
    }

    fn cell_is_free(&self, p: Point2d) -> bool {
        if p == self.snake_front { return false; }
        for sp in self.snake_body.iter() {
            if p == *sp {
                return false;
            }
        }

        if self.game_field[p.as_coords()] > 0 {
            return false;
        }

        for door in &self.doors {
            if !door.open {
                for cell in &door.cells {
                    if p == *cell {
                        return false;
                    }
                }
            }
        }

        // should we also check for food?

        true
    }

    fn make_step(&mut self) {
        for door in &mut self.doors {
            door.tick();
        }

        match self.dir_buffer.next_dir() {
            Some(dir) => {
                // prevent self collision by pressing reverse direction
                if self.dir_x != dir.x && self.dir_y != dir.y {
                    self.dir_x = dir.x;
                    self.dir_y = dir.y;
                }
            }
            None => ()
        }
        self.move_pos(self.dir_x, self.dir_y);
    }

    fn insert_teleporter_2way(&mut self, p1: Point2d, p2: Point2d) {
        self.teleporters.insert(Teleporter::new(p1, p2));
        self.teleporters.insert(Teleporter::new(p2, p1));
    }
}

fn main() {
    let rect_size = 20.0;
    let field_size = 20.0;

    let mut window: PistonWindow =
        WindowSettings::new("Rust Snake", [rect_size * field_size; 2])
            .build().unwrap();

    let mut game = Game::new(field_size as i32, field_size as i32);

    while let Some(e) = window.next() {
        if let Some(button) = e.press_args() {
            match button {
                Button::Keyboard(key) => game.handle_key(key),
                _ => (),
            }
        }

        window.draw_2d(&e, |c, g, _| {
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

            // TODO text([1.0, 0.0, 1.0, 1.0], 8, "hello!", c.transform, g )
        });

        e.update(|arg| {
            game.check_step(arg.dt);
        });
    }
}
