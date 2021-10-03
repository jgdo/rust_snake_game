extern crate piston_window;

use piston_window::keyboard::Key;

use std::collections::VecDeque;
use std::collections::HashSet;
use rand::Rng;
use array2d::Array2D;


#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Point2<T> {
    pub x: T,
    pub y: T,
}

type Point2i = Point2<i32>;
type Point2f = Point2<f32>;

impl<T> Point2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self {
            x,
            y,
        }
    }
}

impl Point2<i32> {
    pub fn as_coords(&self) -> (usize, usize) {
        (self.y as usize, self.x as usize)
    }
}


struct DirBuffer {
    data: VecDeque<Point2i>,
}

impl DirBuffer {
    pub fn new() -> DirBuffer {
        DirBuffer {
            data: VecDeque::new(),
        }
    }

    pub fn enqueue_dir(&mut self, dir: Point2i) {
        if !self.data.contains(&dir) {
            self.data.push_back(dir);
        }
    }

    pub fn next_dir(&mut self) -> Option<Point2i> {
        self.data.pop_front()
    }
}


#[derive(Hash, Eq, PartialEq)]
pub struct Teleporter {
    pub start: Point2i,
    pub end: Point2i,
}

impl Teleporter {
    pub fn new(start: Point2i, end: Point2i) -> Teleporter {
        Teleporter {
            start,
            end,
        }
    }
}


pub struct Door {
    pub cells: HashSet<Point2i>,
    period: usize,
    pub open: bool,
    current_count: usize,
}

impl Door {
    pub fn new(cells: HashSet<Point2i>, period: usize) -> Door {
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
    Teleport,
    Eat,
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
    pub snake_body: VecDeque<(Point2i, Point2i)>,
    snake_front: Point2i,
    current_length: usize,
    pub food_location: Point2i,
    dir_x: i32,
    dir_y: i32,
    snake_progress: f32,
    snake_step_time: f32,
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
            snake_front: Point2::new(width / 4, height / 2),
            current_length: 5,
            food_location: Point2i::new(1, 1),
            dir_x: 0,
            dir_y: 1,
            snake_progress: 0.0,
            snake_step_time: 0.3,
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

        game.insert_teleporter_2way(Point2i::new(game.width - 1, game.height - 1), Point2i::new(0, 0));

        game.doors.push(Door::new(
            Iterator::collect(IntoIterator::into_iter([Point2i::new(game.width / 2, 5), Point2i::new(game.width / 2, 6), Point2i::new(game.width / 2, 7)])),
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

    pub fn get_interpolated_snake_head(&self) -> Point2f {
        let dir_x = self.dir_x as f32;
        let dir_y = self.dir_y as f32;
        // self.snake_front represents the next position on grid
        // when progress will hit 100%
        // until then we need to interpolate between last position
        // and snake_front
        let progress = self.snake_progress-1.0;

        Point2f::new(self.snake_front.x as f32 + dir_x * progress,
                     self.snake_front.y as f32 + dir_y * progress,
        )
    }

    pub fn get_interpolated_snake_tail(&self) -> Option<Point2f> {
        self.snake_body.front().map(|tail| {
            let dir_x = tail.1.x as f32;
            let dir_y = tail.1.y as f32;
            // self.snake_front represents the next position on grid
            // when progress will hit 100%
            // until then we need to interpolate between last position
            // and snake_front
            let progress = if self.snake_body.len() == self.current_length {
                self.snake_progress
            } else {
                0.0
            };

            Point2f::new(tail.0.x as f32 + dir_x * progress,
                         tail.0.y as f32 + dir_y * progress,
            )
        })
    }

    fn change_dir(&mut self, dir_x: i32, dir_y: i32) {
        self.dir_buffer.enqueue_dir(Point2i::new(dir_x, dir_y));
    }

    fn teleport_if_needed(&self, front: Point2i, event: &mut GameEvent) -> Point2i {
        for t in &self.teleporters {
            if front == t.start
            {
                *event = GameEvent::Teleport;
                return t.end;
            }
        }

        front
    }

    pub fn current_length(&self) -> usize {
        self.current_length
    }

    fn do_snake_step(&mut self, dx: i32, dy: i32, event: &mut GameEvent) -> bool {
        self.snake_body.push_back((self.snake_front, Point2i::new(self.dir_x, self.dir_y)));

        let mut next_front = self.snake_front;
        next_front.x = std::cmp::min(std::cmp::max(0, next_front.x + dx), self.width - 1);
        next_front.y = std::cmp::min(std::cmp::max(0, next_front.y + dy), self.height - 1);

        if !self.cell_is_free(next_front) {
            eprintln!("Snake collision!");
            return false;
        }

        self.snake_front = self.teleport_if_needed(next_front, event);

        if self.snake_front == self.food_location {
            self.current_length += 1;
            *event = GameEvent::Eat;
            
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

    fn cell_is_free(&self, p: Point2i) -> bool {
        if p == self.snake_front { return false; }
        for sp in self.snake_body.iter() {
            if p == (*sp).0 {
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

    pub fn make_step(&mut self, dt: f64) -> GameEvent {
        let mut event = GameEvent::None;

        self.snake_progress += dt as f32 / self.snake_step_time;
        if self.snake_progress < 1.0 {
            return GameEvent::None; // still in microstepping
        }

        self.snake_progress -= 1.0;

        for door in &mut self.doors {
            door.tick();
        }

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


        if self.do_snake_step(self.dir_x, self.dir_y, &mut event) {
            event
        } else {
            GameEvent::Collision
        }
    }

    fn insert_teleporter_2way(&mut self, p1: Point2i, p2: Point2i) {
        self.teleporters.insert(Teleporter::new(p1, p2));
        self.teleporters.insert(Teleporter::new(p2, p1));
    }
}