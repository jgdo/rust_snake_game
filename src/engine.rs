extern crate piston_window;

use piston_window::keyboard::Key;

use std::collections::VecDeque;
use std::collections::HashSet;
use rand::Rng;
use array2d::Array2D;


#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Point2d {
    pub x: i32,
    pub y: i32,
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

    pub fn enqueue_dir(&mut self, dir: Point2d){
        if !self.data.contains(&dir) {
            self.data.push_back(dir);
        }
    }

    pub fn next_dir(&mut self) -> Option<Point2d> {
        self.data.pop_front()
    }
}


#[derive(Hash, Eq, PartialEq)]
pub struct Teleporter {
    pub start: Point2d,
    pub end: Point2d,
}

impl Teleporter {
    pub fn new(start: Point2d, end: Point2d) -> Teleporter {
        Teleporter {
            start,
            end,
        }
    }
}



pub struct Door {
    pub cells: HashSet<Point2d>,
    period: usize,
    pub open: bool,
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

pub enum GameEvent {
    None,
    Turn,
    Collision,
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

pub struct Game {
    width: i32,
    height: i32,
    pub snake_body: VecDeque<Point2d>,
    pub snake_front: Point2d,
    current_length: usize,
    pub food_location: Point2d,
    next_time: f64,
    dir_x: i32,
    dir_y: i32,
    rng: rand::rngs::ThreadRng,
    dir_buffer: DirBuffer,
    pub game_field: Array2D<u32>,
    pub teleporters: HashSet<Teleporter>,
    pub doors: Vec<Door>,
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

    pub fn handle_key(&mut self, key: piston_window::Key)
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

    pub fn current_length(&self) -> usize {
        self.current_length
    }

    fn move_pos(&mut self, dx: i32, dy: i32) -> bool {
        self.snake_body.push_back(self.snake_front);

        let mut next_front = self.snake_front;
        next_front.x = std::cmp::min(std::cmp::max(0, next_front.x + dx), self.width - 1);
        next_front.y = std::cmp::min(std::cmp::max(0, next_front.y + dy), self.height - 1);

        if !self.cell_is_free(next_front) {
            eprintln!("Snake collision!");
            return false;
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

        true
    }

    pub fn check_step(&mut self, dt: f64) -> GameEvent {
        let time_step = 0.3;

        self.next_time += dt;
        while self.next_time >= time_step {
            self.next_time -= time_step;

            return self.make_step();
        }

        GameEvent::None
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

    fn make_step(&mut self) -> GameEvent{
        for door in &mut self.doors {
            door.tick();
        }

        let mut event = GameEvent::None;

        match self.dir_buffer.next_dir() {
            Some(dir) => {
                // prevent self collision by pressing reverse direction
                if self.dir_x != dir.x && self.dir_y != dir.y {
                    self.dir_x = dir.x;
                    self.dir_y = dir.y;
                    event = GameEvent::Turn;
                }
            }
            None => ()
        }

        if self.move_pos(self.dir_x, self.dir_y) {
            event
        } else {
            GameEvent::Collision
        }
    }

    fn insert_teleporter_2way(&mut self, p1: Point2d, p2: Point2d) {
        self.teleporters.insert(Teleporter::new(p1, p2));
        self.teleporters.insert(Teleporter::new(p2, p1));
    }
}